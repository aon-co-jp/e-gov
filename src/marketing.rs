//! デジタルガバメントの自動オンラインマーケティング機能: 定期的に
//! 広報用の文章(告知ドラフト)を自動生成し、記録する。
//!
//! **正直な開示(スコープの限界)**: LINE公式アカウント・X(Twitter)・
//! その他SNSへの実際の自動投稿には各プラットフォームのAPIキー/トークンが
//! 必要であり、本プロジェクトには未設定。そのため現時点では
//! 「投稿ドラフトを自動生成してファイルに記録する」ところまでを実装範囲とし、
//! 外部プラットフォームへの実際の自動投稿は今後の課題として明記する
//! (`CLAUDE.md`のHANDOFF参照)。テンプレートベースの生成であり、外部LLM
//! APIも呼び出さない(与信調査ページ同様、規制・コスト面から現時点では
//! シンプルな実装に留める)。

use serde::{Deserialize, Serialize};

/// ページごとの見出し・訴求ポイント(告知文の材料)。
const CAMPAIGNS: &[(&str, &str, &str)] = &[
    (
        "digital-government",
        "eガバメント",
        "コンビニ端末・LINE・WEBの3つの入口から、ペーパーレスで行政手続きが完結。金額に応じた段階的本人確認で安心。",
    ),
    (
        "ai-chat-commerce",
        "オンライン貿易プラットフォーム",
        "AIとの対話で欲しい商品が見つかる。個人から貿易商社まで、既存大手より低い手数料で出店可能。",
    ),
    (
        "credit-and-guarantee",
        "AI与信調査・売掛保証",
        "AIが与信スコアを算出し、掛け仕入れの後払いを実現。電子請求書の重複調査・売掛保証で安心して取引。",
    ),
    (
        "ai-builder",
        "AI工務店・不動産",
        "検索した土地情報から、AIが間取りを提案。投機的な資金流入を助長しない、地に足のついた不動産取引。",
    ),
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketingDraft {
    pub campaign_id: String,
    pub headline: String,
    pub body: String,
    pub generated_at_unix: u64,
}

/// 1件の告知ドラフトを組み立てる(単純なテンプレート埋め込み、LLM不使用)。
fn build_draft(campaign_id: &str, headline: &str, pitch: &str, generated_at_unix: u64) -> MarketingDraft {
    let body = format!(
        "【{headline}】{pitch} 詳しくは e-gov.info をご覧ください。",
    );
    MarketingDraft {
        campaign_id: campaign_id.to_string(),
        headline: headline.to_string(),
        body,
        generated_at_unix,
    }
}

/// 全キャンペーンの告知ドラフトを生成する。「常に・定期的に・自動で」の
/// 定期実行部分は `main.rs` 側の `tokio::time::interval` ループが担い、
/// この関数自体は1回分の生成ロジックに専念する(単一責任、テスト容易性)。
pub fn run_marketing_all() -> Vec<MarketingDraft> {
    let generated_at_unix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    CAMPAIGNS
        .iter()
        .map(|(id, headline, pitch)| build_draft(id, headline, pitch, generated_at_unix))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_marketing_all_produces_one_draft_per_campaign() {
        let drafts = run_marketing_all();
        assert_eq!(drafts.len(), CAMPAIGNS.len());
    }

    #[test]
    fn draft_body_contains_headline_and_site_mention() {
        let drafts = run_marketing_all();
        for draft in &drafts {
            assert!(draft.body.contains(&draft.headline));
            assert!(draft.body.contains("e-gov.info"));
        }
    }

    #[test]
    fn campaign_ids_are_unique() {
        let drafts = run_marketing_all();
        let mut ids: Vec<&str> = drafts.iter().map(|d| d.campaign_id.as_str()).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), drafts.len());
    }
}
