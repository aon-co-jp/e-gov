//! GitHub(`aon-co-jp/e-gov`)の README.md / CLAUDE.md / PORTING.md を
//! 自動取得し、GitHub風にレンダリングして表示する機能。
//! `aruaru-tokyo`の複数リポジトリ切替版を、単一リポジトリ固定に
//! 簡略化した実装(移植手順は `PORTING.md` 参照)。

const GITHUB_ORG: &str = "aon-co-jp";
const GITHUB_REPO: &str = "e-gov";
pub const GITHUB_REPO_URL: &str = "https://github.com/aon-co-jp/e-gov";

const REPO_FILES: &[(&str, &str, bool)] = &[
    ("README.md", "README(概要)", true),
    ("CLAUDE.md", "CLAUDE.md(開発方針・設計方針)", false),
    ("PORTING.md", "PORTING.md(お引越し可能ファイル)", false),
];

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// READMEなど各Markdownを`//!`付きrustdocコメント形式の`.rs`風テキストへ変換する
/// (readme-to-rs構想の簡易実装、`aruaru-tokyo`と同じロジック)。
fn markdown_to_rs(markdown: &str) -> String {
    markdown
        .lines()
        .map(|line| if line.is_empty() { "//!".to_string() } else { format!("//! {line}") })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

/// MarkdownをGitHub風の見た目でレンダリングしたHTMLへ変換する。
fn markdown_to_github_style_html(markdown: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(markdown, options);
    let mut html_out = String::new();
    html::push_html(&mut html_out, parser);
    html_out
}

async fn fetch_repo_file(client: &reqwest::Client, filename: &str) -> Option<String> {
    for branch in ["main", "master"] {
        let url = format!("https://raw.githubusercontent.com/{GITHUB_ORG}/{GITHUB_REPO}/{branch}/{filename}");
        if let Ok(resp) = client
            .get(&url)
            .header("User-Agent", "e-gov-info-readme-viewer/0.1")
            .timeout(std::time::Duration::from_secs(6))
            .send()
            .await
        {
            if resp.status().is_success() {
                if let Ok(body) = resp.text().await {
                    return Some(body);
                }
            }
        }
    }
    None
}

/// `README.md`/`CLAUDE.md`/`PORTING.md`をそれぞれ取得し、GitHub風表示と
/// `.rs`形式表示を切替タブで見られるHTMLブロックを組み立てる。
pub async fn render_repo_viewer() -> String {
    let client = reqwest::Client::new();
    let mut out = String::new();
    out.push_str(&format!(
        "<p class=\"repo-link\"><a href=\"{GITHUB_REPO_URL}\" target=\"_blank\" rel=\"noopener\">🔗 {GITHUB_ORG}/{GITHUB_REPO} をGitHubで開く</a></p>\n"
    ));

    for (idx, (filename, label, required)) in REPO_FILES.iter().enumerate() {
        let markdown = fetch_repo_file(&client, filename).await;
        out.push_str("<div class=\"repo-file-block\">\n");
        out.push_str(&format!("  <h3>{}</h3>\n", html_escape(label)));
        match markdown {
            Some(md) => {
                let gh_html = markdown_to_github_style_html(&md);
                let rs = html_escape(&markdown_to_rs(&md));
                let tab_id = format!("egov-{idx}");
                out.push_str(&format!(
                    r#"  <div class="view-toggle" data-tab="{tab_id}">
    <button type="button" class="view-toggle-btn active" data-view="gh">GitHub風表示</button>
    <button type="button" class="view-toggle-btn" data-view="rs">.rs形式</button>
  </div>
  <div class="markdown-body" id="gh-{tab_id}">{gh_html}</div>
  <pre class="rs-output hidden" id="rs-{tab_id}">{rs}</pre>
"#
                ));
            }
            None => {
                let msg = if *required {
                    format!("❌ {filename} を取得できませんでした({GITHUB_ORG}/{GITHUB_REPO})。")
                } else {
                    format!("❌ {filename} はこのリポジトリにはありません。")
                };
                out.push_str(&format!("  <p class=\"rs-error\">{}</p>\n", html_escape(&msg)));
            }
        }
        out.push_str("</div>\n");
    }
    out
}

/// 表示に必要なCSS(`page_shell`の`<style>`に追記して使う)。
pub const VIEW_TOGGLE_CSS: &str = r#"
.repo-link { margin: 0 0 1rem; font-size: 0.9rem; }
.repo-file-block { margin-top: 1.5rem; }
.repo-file-block h3 { font-size: 0.9rem; margin: 0 0 0.4rem; color: #777; }
.view-toggle { display: flex; gap: 0.4rem; margin-bottom: 0.5rem; }
.view-toggle-btn { background: transparent; color: #777; border: 1px solid #ccc; border-radius: 999px; padding: 0.3rem 0.9rem; font-size: 0.78rem; font-weight: 600; cursor: pointer; }
.view-toggle-btn.active { background: #2f6fed; color: #fff; border-color: #2f6fed; }
.hidden { display: none !important; }
.markdown-body { background: #fafafa; border: 1px solid #eee; border-radius: 0.6rem; padding: 1.5rem 2rem; overflow-x: auto; max-height: 70vh; overflow-y: auto; }
.markdown-body img { max-width: 100%; }
.markdown-body h1, .markdown-body h2, .markdown-body h3 { border-bottom: 1px solid #eee; padding-bottom: 0.3rem; }
.markdown-body code { background: rgba(0,0,0,0.06); padding: 0.1rem 0.35rem; border-radius: 0.3rem; font-family: monospace; font-size: 0.85em; }
.markdown-body pre { background: #1e1e1e; color: #d4d4d4; padding: 0.8rem 1rem; border-radius: 0.5rem; overflow-x: auto; }
.markdown-body pre code { background: none; padding: 0; }
.markdown-body table { border-collapse: collapse; width: 100%; font-size: 0.88rem; }
.markdown-body th, .markdown-body td { border: 1px solid #eee; padding: 0.4rem 0.6rem; }
.markdown-body blockquote { border-left: 3px solid #ff7a45; margin: 0; padding: 0.2rem 1rem; color: #777; }
.rs-output { margin-top: 1.25rem; background: #1e1e1e; color: #d4d4d4; border-radius: 0.6rem; padding: 1.25rem 1.5rem; overflow-x: auto; font-family: monospace; font-size: 0.82rem; line-height: 1.5; max-height: 70vh; }
.rs-error { color: #c0392b; font-size: 0.85rem; margin-top: 1rem; }
"#;

/// 切替タブを配線するJS(`page_shell`の`<script>`に追記して使う)。
pub const VIEW_TOGGLE_JS: &str = r#"
document.querySelectorAll('.view-toggle').forEach(toggle => {
  const tabId = toggle.getAttribute('data-tab');
  const ghEl = document.getElementById('gh-' + tabId);
  const rsEl = document.getElementById('rs-' + tabId);
  toggle.querySelectorAll('.view-toggle-btn').forEach(b => {
    b.addEventListener('click', () => {
      toggle.querySelectorAll('.view-toggle-btn').forEach(x => x.classList.remove('active'));
      b.classList.add('active');
      const view = b.getAttribute('data-view');
      if (view === 'gh') { ghEl.classList.remove('hidden'); rsEl.classList.add('hidden'); }
      else { ghEl.classList.add('hidden'); rsEl.classList.remove('hidden'); }
    });
  });
});
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_to_rs_prefixes_every_line() {
        let out = markdown_to_rs("# Title\n\nBody text");
        assert_eq!(out, "//! # Title\n//!\n//! Body text\n");
    }

    #[test]
    fn markdown_to_github_style_html_renders_heading() {
        let out = markdown_to_github_style_html("# Hello");
        assert!(out.contains("<h1>"));
        assert!(out.contains("Hello"));
    }

    #[test]
    fn html_escape_escapes_all_special_chars() {
        assert_eq!(html_escape("<a>&\"'"), "&lt;a&gt;&amp;&quot;&#39;");
    }
}
