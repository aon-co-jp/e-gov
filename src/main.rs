//! e-gov.info — Rust + Poem 版TOPページ。
//! aruaru-tokyo-server / karu.tokyo と同じ技術スタック・実装方針を踏襲する:
//! DB非依存・1バイナリ完結・サーバーサイド文字列組み立てHTML(テンプレート
//! エンジン不使用)。
//!
//! 2本柱: (1) eガバメント(デジタルガバメント、コンビニ端末+LINE+WEB、
//! 段階的本人確認) (2) オンライン貿易プラットフォーム(AIチャットコマース、
//! 不動産投資・電子契約・AI工務店)。詳細な設計思想は CLAUDE.md 参照。
//!
//! v0.1.0時点では紹介ページのみで、申請フォーム・決済・本人確認等の実機能は
//! 未実装(CLAUDE.md の HANDOFF に今後の調査タスクを記載)。
//!
//! **本プロジェクトは正式な許可が下りるまでのサンプル・デモンストレー
//! ションサイトである**(ユーザー指示、2026-07-18)。電子公証・電子契約
//! (法的拘束力のある売買・賃貸契約の締結)は現段階では一切実装しない。
//!
//! 多言語対応の基本言語セット(README-<言語>.md方式、詳細はCLAUDE.md参照):
//! 日本語・英語(米/英)・中国語・台湾語・韓国語・伊・仏・独・アラビア語・
//! ペルシャ語・ロシア語・ウクライナ語。サイト本体も`?lang=<コード>`
//! クエリパラメータでこの13言語を切り替え可能(`src/i18n.rs`)、
//! **既定言語は英語**。アラビア語・ペルシャ語は`dir="rtl"`も自動設定。

mod chat_commerce;
mod github_viewer;
mod i18n;
mod line_webhook;
mod marketing;
mod research;

use i18n::Lang;
use poem::listener::TcpListener;
use poem::web::{Html, Query};
use poem::{get, handler, post, Route, Server};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LangQuery {
    lang: Option<String>,
}

const GITHUB_REPO_URL: &str = "https://github.com/aon-co-jp/e-gov";

/// 定期実行の既定間隔(環境変数`E_GOV_RESEARCH_INTERVAL_HOURS`/
/// `E_GOV_MARKETING_INTERVAL_HOURS`で上書き可能)。
const DEFAULT_RESEARCH_INTERVAL_HOURS: u64 = 24;
const DEFAULT_MARKETING_INTERVAL_HOURS: u64 = 12;

fn interval_hours_from_env(var_name: &str, default_hours: u64) -> u64 {
    std::env::var(var_name)
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|&h| h > 0)
        .unwrap_or(default_hours)
}

/// 生成物(調査レポート/マーケティングドラフト)の書き出し先ディレクトリ。
fn data_dir() -> std::path::PathBuf {
    std::env::var("E_GOV_DATA_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
}

async fn research_pass_and_save() {
    match research::run_research_all().await {
        Ok(report) => {
            let path = data_dir().join("research-report.json");
            match serde_json::to_vec_pretty(&report) {
                Ok(bytes) => {
                    if let Err(err) = std::fs::write(&path, bytes) {
                        tracing::warn!("failed to write {}: {err}", path.display());
                    } else {
                        tracing::info!("research report written to {}", path.display());
                    }
                }
                Err(err) => tracing::warn!("failed to serialize research report: {err}"),
            }
        }
        Err(err) => tracing::warn!("research pass failed: {err}"),
    }
}

fn marketing_pass_and_save() {
    let drafts = marketing::run_marketing_all();
    let path = data_dir().join("marketing-drafts.json");
    match serde_json::to_vec_pretty(&drafts) {
        Ok(bytes) => {
            if let Err(err) = std::fs::write(&path, bytes) {
                tracing::warn!("failed to write {}: {err}", path.display());
            } else {
                tracing::info!("marketing drafts written to {}", path.display());
            }
        }
        Err(err) => tracing::warn!("failed to serialize marketing drafts: {err}"),
    }
}

/// 「常に・コンスタントに・定期的に・自動で」を満たす、サーバー内蔵の
/// 定期実行ループ。手動トリガー(`--research-all`/`--marketing-all`)とは
/// 別に、サーバー起動時に自動でバックグラウンド開始する。
fn spawn_periodic_tasks() {
    let research_interval = interval_hours_from_env("E_GOV_RESEARCH_INTERVAL_HOURS", DEFAULT_RESEARCH_INTERVAL_HOURS);
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(research_interval * 3600));
        loop {
            ticker.tick().await;
            tracing::info!("periodic research pass starting (every {research_interval}h)");
            research_pass_and_save().await;
        }
    });

    let marketing_interval = interval_hours_from_env("E_GOV_MARKETING_INTERVAL_HOURS", DEFAULT_MARKETING_INTERVAL_HOURS);
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(marketing_interval * 3600));
        loop {
            ticker.tick().await;
            tracing::info!("periodic marketing pass starting (every {marketing_interval}h)");
            marketing_pass_and_save();
        }
    });
}

/// 現在のページを保ったまま言語だけ切り替えるナビ(`?lang=xx`)。
fn render_lang_switcher(current_path: &str) -> String {
    i18n::Lang::ALL
        .iter()
        .map(|l| format!(r#"<a href="{current_path}?lang={}">{}</a>"#, l.code(), l.native_name()))
        .collect::<Vec<_>>()
        .join(" ・ ")
}

fn page_shell(lang: Lang, current_path: &str, title: &str, body: &str) -> String {
    let c = i18n::common(lang);
    let dir = if lang.is_rtl() { "rtl" } else { "ltr" };
    let html_lang = lang.html_lang();
    let lang_switcher = render_lang_switcher(current_path);

    // 英語自体がバナーの言語(既定言語)の場合、二重表示を避けて1行だけにする。
    let banner_local_html = if c.banner_local == c.banner_en {
        String::new()
    } else {
        format!(r#"<span class="local">{}</span>"#, c.banner_local)
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="{html_lang}" dir="{dir}">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<style>
body {{ font-family: -apple-system, "Hiragino Sans", "Yu Gothic", sans-serif; max-width: 780px; margin: 2rem auto; padding: 0 1rem; line-height: 1.7; color: #222; }}
h1 {{ font-size: 1.6rem; }}
h2 {{ font-size: 1.2rem; margin-top: 2rem; border-bottom: 2px solid #eee; padding-bottom: 0.3rem; }}
a {{ color: #222; }}
a:visited {{ color: #222; }}
nav a {{ margin-right: 1rem; }}
ul.linklist li {{ margin-bottom: 0.5rem; }}
footer {{ margin-top: 3rem; font-size: 0.85rem; color: #777; }}
.badge {{ display: inline-block; background: #eee; border-radius: 4px; padding: 0.1rem 0.5rem; font-size: 0.8rem; margin-left: 0.5rem; }}
.sample-banner {{ background: #b91c1c; color: #fff; text-align: center; font-weight: 700; padding: 0.9rem 1rem; margin: -2rem -1rem 1.5rem; font-size: 1.05rem; line-height: 1.5; }}
.sample-banner .en {{ display: block; font-size: 1.15rem; }}
.sample-banner .local {{ display: block; margin-top: 0.2rem; }}
.lang-switcher {{ font-size: 0.78rem; color: #777; margin-bottom: 0.75rem; line-height: 1.8; }}
.lang-switcher a {{ color: #777; }}
{view_toggle_css}
</style>
</head>
<body>
<div class="sample-banner">
<span class="en">{banner_en}</span>
{banner_local_html}
</div>
<div class="lang-switcher">{lang_switcher}</div>
<nav><a href="/">{nav_top}</a> <a href="/gov">{nav_gov}</a> <a href="/trade">{nav_trade}</a> <a href="/credit">{nav_credit}</a> <a href="/realestate">{nav_realestate}</a> <a href="/research">{nav_research}</a></nav>
{body}
<footer><p>{footer} <a href="{GITHUB_REPO_URL}">GitHub (aon-co-jp/e-gov)</a></p></footer>
<script>{view_toggle_js}</script>
</body>
</html>"#,
        banner_en = c.banner_en,
        nav_top = c.nav_top,
        nav_gov = c.nav_gov,
        nav_trade = c.nav_trade,
        nav_credit = c.nav_credit,
        nav_realestate = c.nav_realestate,
        nav_research = c.nav_research,
        footer = c.footer,
        view_toggle_css = github_viewer::VIEW_TOGGLE_CSS,
        view_toggle_js = github_viewer::VIEW_TOGGLE_JS,
    )
}

#[handler]
async fn index(Query(q): Query<LangQuery>) -> Html<String> {
    let lang = Lang::parse(q.lang.as_deref());
    let t = i18n::index_text(lang);
    let c = i18n::common(lang);
    let repo_viewer = github_viewer::render_repo_viewer().await;
    let body = format!(
        r#"
<h1>{h1} <span class="badge">v0.1.0</span></h1>
<p>{intro}</p>
<ul class="linklist">
<li><a href="/gov">{nav_gov}</a></li>
<li><a href="/trade">{nav_trade}</a></li>
<li><a href="/credit">{nav_credit}</a></li>
<li><a href="/realestate">{nav_realestate}</a></li>
</ul>

<h2>📄 README/CLAUDE.md/PORTING.md</h2>
<p>{repo_note}</p>
{repo_viewer}
"#,
        h1 = t.h1,
        intro = t.body,
        nav_gov = c.nav_gov,
        nav_trade = c.nav_trade,
        nav_credit = c.nav_credit,
        nav_realestate = c.nav_realestate,
        repo_note = "aon-co-jp/e-gov (live from GitHub)",
    );
    Html(page_shell(lang, "/", t.title, &body))
}

fn simple_page(lang: Lang, path: &str, t: i18n::PageText) -> Html<String> {
    let body = format!("<h1>{}</h1>\n<p>{}</p>\n", t.h1, t.body);
    Html(page_shell(lang, path, t.title, &body))
}

#[handler]
fn gov(Query(q): Query<LangQuery>) -> Html<String> {
    let lang = Lang::parse(q.lang.as_deref());
    simple_page(lang, "/gov", i18n::gov_text(lang))
}

#[handler]
fn trade(Query(q): Query<LangQuery>) -> Html<String> {
    let lang = Lang::parse(q.lang.as_deref());
    simple_page(lang, "/trade", i18n::trade_text(lang))
}

#[handler]
fn credit(Query(q): Query<LangQuery>) -> Html<String> {
    let lang = Lang::parse(q.lang.as_deref());
    simple_page(lang, "/credit", i18n::credit_text(lang))
}

#[handler]
fn realestate(Query(q): Query<LangQuery>) -> Html<String> {
    let lang = Lang::parse(q.lang.as_deref());
    simple_page(lang, "/realestate", i18n::realestate_text(lang))
}

#[handler]
fn research_page(Query(q): Query<LangQuery>) -> Html<String> {
    let lang = Lang::parse(q.lang.as_deref());
    simple_page(lang, "/research", i18n::research_text(lang))
}

#[handler]
fn healthz() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--research-all") {
        research_pass_and_save().await;
        return Ok(());
    }
    if args.iter().any(|a| a == "--marketing-all") {
        marketing_pass_and_save();
        return Ok(());
    }

    let app = Route::new()
        .at("/", get(index))
        .at("/gov", get(gov))
        .at("/trade", get(trade))
        .at("/credit", get(credit))
        .at("/realestate", get(realestate))
        .at("/research", get(research_page))
        .at("/webhook/line", post(line_webhook::line_webhook))
        .at("/healthz", get(healthz));

    spawn_periodic_tasks();

    let bind_addr = "127.0.0.1:4500";
    tracing::info!("e-gov-server listening on {bind_addr}");
    Server::new(TcpListener::bind(bind_addr)).run(app).await
}
