//! デジタルガバメントの自動調査機能: 日本語・英語の両方でGoogle検索
//! リンクを組み立て、GitHubは実際のGitHub Search REST APIを叩いて
//! 関連リポジトリを取得する。
//!
//! **正直な開示(スコープの限界)**: Google検索結果そのものを自動取得する
//! (スクレイピングする)処理は、Google Custom Search JSON API等の
//! 有償/要APIキーの手段が本来必要であり、本プロジェクトには未設定。
//! そのためGoogle側は「検索結果ページへのリンクを組み立てて記録する」
//! ところまでを自動化範囲とし、実際に結果を取得・解析するのは
//! GitHub側(GitHub Search API、APIキー不要で利用可能)のみとする。
//! この非対称性は意図的なものであり、隠さずここに明記する。

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// 調査対象トピック(日本語・英語ペア)。デジタルガバメント・オンライン貿易
/// プラットフォームの中核テーマに沿って選定。
const TOPICS: &[(&str, &str)] = &[
    ("デジタルガバメント 電子政府 コンビニ端末", "digital government e-government kiosk terminal"),
    ("マイナンバーカード スマホ NFC 公的個人認証", "national ID card smartphone NFC authentication"),
    ("AIチャットコマース 通販", "AI chat commerce online marketplace"),
    ("AI与信調査 掛け仕入れ 売掛保証", "AI credit scoring trade credit accounts receivable insurance"),
    ("電子請求書 重複 インボイス制度", "electronic invoice duplicate detection"),
    ("AI工務店 間取り提案 不動産", "AI construction chatbot floor plan real estate"),
];

fn percent_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 3);
    for byte in input.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*byte as char);
            }
            _ => out.push_str(&format!("%{:02X}", byte)),
        }
    }
    out
}

fn google_search_url(query: &str) -> String {
    format!("https://www.google.com/search?q={}", percent_encode(query))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubHit {
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub stargazers_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicReport {
    pub topic_ja: String,
    pub topic_en: String,
    pub google_search_url_ja: String,
    pub google_search_url_en: String,
    pub github_total_count: u64,
    pub github_top_hits: Vec<GithubHit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchReport {
    pub generated_at_unix: u64,
    pub topics: Vec<TopicReport>,
}

#[derive(Debug, Deserialize)]
struct GithubSearchResponse {
    total_count: u64,
    items: Vec<GithubItem>,
}

#[derive(Debug, Deserialize)]
struct GithubItem {
    full_name: String,
    html_url: String,
    description: Option<String>,
    stargazers_count: u64,
}

/// GitHub Search API (`/search/repositories`) を叩いて上位3件を取得する。
/// APIキー不要(未認証)だが、GitHubのAPI利用規約でUser-Agentヘッダが
/// 必須のため付与する。未認証だとレート制限が10req/minと厳しいため、
/// 呼び出し間に短いウェイトを挟む(`run_research_all`側で実施)。
async fn github_search(query: &str) -> anyhow::Result<(u64, Vec<GithubHit>)> {
    let client = reqwest::Client::builder()
        .user_agent("e-gov-info-research-bot (https://github.com/aon-co-jp/e-gov)")
        .timeout(Duration::from_secs(15))
        .build()?;

    let url = format!(
        "https://api.github.com/search/repositories?q={}&sort=stars&order=desc&per_page=3",
        percent_encode(query)
    );

    let resp = client
        .get(&url)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?
        .error_for_status()?;

    let parsed: GithubSearchResponse = resp.json().await?;

    let hits = parsed
        .items
        .into_iter()
        .map(|item| GithubHit {
            full_name: item.full_name,
            html_url: item.html_url,
            description: item.description,
            stargazers_count: item.stargazers_count,
        })
        .collect();

    Ok((parsed.total_count, hits))
}

/// 全トピックについて、日本語・英語それぞれのGoogle検索リンクを組み立て、
/// GitHubは英語クエリで実際に検索して結果を取得する
/// (GitHub側の検索は英語クエリの方がヒット率が高いため英語のみで実施、
/// Google側は日本語・英語の両方のリンクを用意する——ユーザー指示
/// 「英語と日本語で...検索」を、取得可否の実情に応じて非対称に実装した
/// 判断であり、この理由をコメントとして明記する)。
pub async fn run_research_all() -> anyhow::Result<ResearchReport> {
    let mut topics = Vec::with_capacity(TOPICS.len());

    for (topic_ja, topic_en) in TOPICS {
        tracing::info!("researching topic: {topic_ja} / {topic_en}");

        let (total_count, hits) = match github_search(topic_en).await {
            Ok(result) => result,
            Err(err) => {
                tracing::warn!("github search failed for '{topic_en}': {err}");
                (0, Vec::new())
            }
        };

        topics.push(TopicReport {
            topic_ja: topic_ja.to_string(),
            topic_en: topic_en.to_string(),
            google_search_url_ja: google_search_url(topic_ja),
            google_search_url_en: google_search_url(topic_en),
            github_total_count: total_count,
            github_top_hits: hits,
        });

        // GitHub未認証APIのレート制限(10req/min)に配慮した間隔。
        tokio::time::sleep(Duration::from_secs(7)).await;
    }

    let generated_at_unix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    Ok(ResearchReport { generated_at_unix, topics })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_encode_handles_spaces_and_unicode() {
        let encoded = percent_encode("digital government 電子政府");
        assert!(encoded.contains("digital%20government"));
        assert!(!encoded.contains(' '));
    }

    #[test]
    fn google_search_url_is_well_formed() {
        let url = google_search_url("test query");
        assert_eq!(url, "https://www.google.com/search?q=test%20query");
    }

    #[test]
    fn topics_list_is_non_empty_and_bilingual() {
        assert!(!TOPICS.is_empty());
        for (ja, en) in TOPICS {
            assert!(!ja.is_empty());
            assert!(!en.is_empty());
        }
    }
}
