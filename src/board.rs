//! WEB商談コーナー(オンライン商談の呼びかけ掲示板)と、Google Meetを使った
//! 世界中の人と繋がるTV CHATコーナー(雑談・交流の呼びかけ掲示板)。
//!
//! **DB非依存の設計方針を踏襲**し、プロセス内メモリ(`Mutex<Vec<Post>>`)
//! のみで完結する——サーバー再起動で消える点も含め、「投稿は古くなると
//! 自動削除される」という掲示板の性質と矛盾しない設計判断。
//!
//! 投稿は`RETENTION_HOURS`(既定72時間、環境変数`E_GOV_BOARD_RETENTION_HOURS`
//! で変更可)を過ぎると次回アクセス時に自動的に削除される(遅延パージ方式:
//! 投稿一覧を読む/書くたびに期限切れをふるい落とす、専用のバックグラウンド
//! タスクは持たない——投稿数が少ない前提のこのデモ規模では十分)。

use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    /// WEB商談コーナー(オンライン商談の呼びかけ)。
    Negotiation,
    /// Google Meetを使った世界中の人とのTV CHATコーナー。
    TvChat,
}

impl Category {
    fn parse(s: &str) -> Option<Category> {
        match s {
            "negotiation" => Some(Category::Negotiation),
            "tvchat" => Some(Category::TvChat),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Post {
    pub category: Category,
    pub name: String,
    pub message: String,
    /// `https://meet.google.com/...`等の任意リンク。TV CHATコーナーでの
    /// Google Meet招待URL、WEB商談コーナーでの商談リンク双方に使う。
    pub meet_link: Option<String>,
    pub created_at_epoch: i64,
}

/// フォーム投稿1件あたりの上限文字数(乱用防止の簡易ガード)。
const MAX_NAME_LEN: usize = 60;
const MAX_MESSAGE_LEN: usize = 800;
const MAX_LINK_LEN: usize = 300;
/// 投稿は既定でこの時間を過ぎると自動削除される。
const DEFAULT_RETENTION_HOURS: i64 = 72;
/// 掲示板が無制限に肥大化しないための、カテゴリごとの保持件数上限
/// (期限内でも超過分は古い順に削除する)。
const MAX_POSTS_PER_CATEGORY: usize = 200;

static POSTS: Lazy<Mutex<Vec<Post>>> = Lazy::new(|| Mutex::new(Vec::new()));

fn retention_seconds() -> i64 {
    std::env::var("E_GOV_BOARD_RETENTION_HOURS")
        .ok()
        .and_then(|v| v.parse::<i64>().ok())
        .filter(|&h| h > 0)
        .unwrap_or(DEFAULT_RETENTION_HOURS)
        * 3600
}

fn now_epoch() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

/// 期限切れの投稿を取り除く。読み出し・書き込みの両経路から呼ぶ。
fn purge_expired_locked(posts: &mut Vec<Post>) {
    let cutoff = now_epoch() - retention_seconds();
    posts.retain(|p| p.created_at_epoch >= cutoff);
}

#[derive(Debug, Deserialize)]
pub struct NewPostForm {
    pub category: String,
    pub name: String,
    pub message: String,
    #[serde(default)]
    pub meet_link: String,
    /// フォーム送信後、同じ言語のページへ戻すためのhiddenフィールド。
    #[serde(default)]
    pub lang: String,
}

/// 投稿を追加する。カテゴリ不明・本文空・文字数超過の場合はエラー文字列を返す。
pub fn add_post(form: &NewPostForm) -> Result<(), &'static str> {
    let category = Category::parse(form.category.trim()).ok_or("unknown category")?;
    let name = form.name.trim();
    let message = form.message.trim();
    if name.is_empty() || message.is_empty() {
        return Err("name and message must not be empty");
    }
    if name.chars().count() > MAX_NAME_LEN || message.chars().count() > MAX_MESSAGE_LEN {
        return Err("name or message too long");
    }
    let meet_link = form.meet_link.trim();
    if meet_link.chars().count() > MAX_LINK_LEN {
        return Err("link too long");
    }
    let meet_link = if meet_link.is_empty() { None } else { Some(meet_link.to_string()) };

    let mut posts = POSTS.lock().expect("board posts mutex poisoned");
    purge_expired_locked(&mut posts);

    posts.push(Post {
        category,
        name: name.to_string(),
        message: message.to_string(),
        meet_link,
        created_at_epoch: now_epoch(),
    });

    // カテゴリごとの上限を超えたら、古いものから間引く。
    let mut count_in_category = posts.iter().filter(|p| p.category == category).count();
    if count_in_category > MAX_POSTS_PER_CATEGORY {
        if let Some(oldest_idx) = posts
            .iter()
            .enumerate()
            .filter(|(_, p)| p.category == category)
            .min_by_key(|(_, p)| p.created_at_epoch)
            .map(|(i, _)| i)
        {
            posts.remove(oldest_idx);
            count_in_category -= 1;
        }
    }
    let _ = count_in_category;
    Ok(())
}

/// 指定カテゴリの投稿を新しい順に返す(期限切れは呼び出し時にパージ済み)。
pub fn list_posts(category: Category) -> Vec<Post> {
    let mut posts = POSTS.lock().expect("board posts mutex poisoned");
    purge_expired_locked(&mut posts);
    let mut result: Vec<Post> = posts.iter().filter(|p| p.category == category).cloned().collect();
    result.sort_by(|a, b| b.created_at_epoch.cmp(&a.created_at_epoch));
    result
}

/// 現在の保持期間(時間単位)。ページ表示・テストで参照する。
pub fn retention_hours() -> i64 {
    retention_seconds() / 3600
}

/// サーバー起動時に定期実行するパージ処理(`main.rs::spawn_periodic_tasks`
/// から呼ぶ)。掲示板アクセスが無い間もメモリを解放しておくための保険。
pub fn purge_now() {
    let mut posts = POSTS.lock().expect("board posts mutex poisoned");
    purge_expired_locked(&mut posts);
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// 投稿1件をHTMLの`<li>`断片として描画する。
pub fn render_post(p: &Post) -> String {
    let link_html = match &p.meet_link {
        Some(link) if link.starts_with("http://") || link.starts_with("https://") => {
            format!(r#" — <a href="{0}" target="_blank" rel="noopener noreferrer">{0}</a>"#, html_escape(link))
        }
        Some(link) => format!(" — {}", html_escape(link)),
        None => String::new(),
    };
    format!(
        r#"<li><strong>{}</strong>: {}{}</li>"#,
        html_escape(&p.name),
        html_escape(&p.message),
        link_html
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // テストは共有static `POSTS`を操作するため、Rustのデフォルト並列
    // テスト実行だとお互いに干渉する。全テストがこのロックを取得してから
    // 実行することで直列化する(他の場所でも使われている既存パターン)。
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn clear_board() -> std::sync::MutexGuard<'static, ()> {
        let guard = TEST_LOCK.lock().unwrap();
        POSTS.lock().unwrap().clear();
        guard
    }

    #[test]
    fn add_and_list_posts_round_trip() {
        let _guard = clear_board();
        let form = NewPostForm {
            category: "negotiation".to_string(),
            name: "Alice".to_string(),
            message: "Looking for a supplier".to_string(),
            meet_link: String::new(),
            lang: "en".to_string(),
        };
        add_post(&form).unwrap();
        let posts = list_posts(Category::Negotiation);
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].name, "Alice");
        assert!(list_posts(Category::TvChat).is_empty());
    }

    #[test]
    fn rejects_unknown_category_and_empty_fields() {
        let _guard = clear_board();
        let bad_category = NewPostForm {
            category: "bogus".to_string(),
            name: "Bob".to_string(),
            message: "hi".to_string(),
            meet_link: String::new(),
            lang: "en".to_string(),
        };
        assert!(add_post(&bad_category).is_err());

        let empty_message = NewPostForm {
            category: "tvchat".to_string(),
            name: "Bob".to_string(),
            message: "   ".to_string(),
            meet_link: String::new(),
            lang: "en".to_string(),
        };
        assert!(add_post(&empty_message).is_err());
    }

    #[test]
    fn purge_removes_posts_older_than_retention() {
        let _guard = clear_board();
        {
            let mut posts = POSTS.lock().unwrap();
            posts.push(Post {
                category: Category::TvChat,
                name: "Old".to_string(),
                message: "stale post".to_string(),
                meet_link: None,
                created_at_epoch: now_epoch() - retention_seconds() - 10,
            });
            posts.push(Post {
                category: Category::TvChat,
                name: "Fresh".to_string(),
                message: "recent post".to_string(),
                meet_link: None,
                created_at_epoch: now_epoch(),
            });
        }
        let posts = list_posts(Category::TvChat);
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].name, "Fresh");
    }

    #[test]
    fn render_post_escapes_html_and_links_meet_urls() {
        let p = Post {
            category: Category::TvChat,
            name: "<script>".to_string(),
            message: "hello & welcome".to_string(),
            meet_link: Some("https://meet.google.com/abc-defg-hij".to_string()),
            created_at_epoch: now_epoch(),
        };
        let html = render_post(&p);
        assert!(html.contains("&lt;script&gt;"));
        assert!(html.contains("hello &amp; welcome"));
        assert!(html.contains(r#"<a href="https://meet.google.com/abc-defg-hij""#));
    }
}
