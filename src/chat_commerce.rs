//! AIチャットコマースの応答ロジック(LINE Webhook・将来のWeb版チャット
//! 双方から共有される想定)。
//!
//! **正直な開示(現状のスコープ)**: 「AI」チャットコマースと銘打っている
//! ものの、現時点では外部LLM APIを一切呼び出さない**ルールベースの
//! キーワードマッチング**による応答に留まる。理由は`marketing.rs`と同様、
//! 規制・コスト・応答内容の予測可能性の観点から、まず決定的でテスト
//! しやすい実装から始めるため。将来、実際のLLM連携(与信情報等の機微な
//! 判断はLLMに丸投げせず、あくまで案内・絞り込みに用途を限定する設計と
//! すべき)に置き換える場合は、この関数のシグネチャ(文字列入力→文字列
//! 出力)を保ったまま内部実装だけ差し替えられるようにしてある。

/// 各カテゴリのキーワードと、案内する内容・遷移先ページ。
struct Intent {
    keywords: &'static [&'static str],
    reply: &'static str,
}

const INTENTS: &[Intent] = &[
    Intent {
        keywords: &["申請", "手続き", "行政", "役所", "マイナンバー", "government", "application"],
        reply: "eガバメント(デジタルガバメント)についてのご案内ですね。\
ペーパーレスでのオンライン申請、コンビニ端末(Loppi/Famiポート等)での手続き、\
金額に応じた段階的な本人確認に対応しています。詳しくは https://e-gov.info/gov をご覧ください。",
    },
    Intent {
        keywords: &["買いたい", "欲しい", "注文", "商品", "buy", "want", "order", "product"],
        reply: "オンライン貿易プラットフォームでのお買い物ですね。\
食料品・家電・自動車・オーディオ機器まで幅広く取り扱っています(現在は実在庫を伴わないサンプル運用です)。\
具体的にどんな商品をお探しですか? 詳しくは https://e-gov.info/trade をご覧ください。",
    },
    Intent {
        keywords: &["仕入れ", "与信", "掛け", "売掛", "請求書", "credit", "invoice"],
        reply: "AI与信調査・掛け仕入れ・売掛保証についてのご質問ですね。\
与信スコアに応じた後払い仕入れ、電子請求書の重複調査、売掛債権の保証に対応予定です\
(現時点では設計方針の段階で、実際の与信審査機能はまだ搭載していません)。\
詳しくは https://e-gov.info/credit をご覧ください。",
    },
    Intent {
        keywords: &["不動産", "土地", "間取り", "工務店", "賃貸", "real estate", "land", "house"],
        reply: "不動産投資・AI工務店についてのご質問ですね。\
検索した土地情報をもとにAIが間取りをご提案する機能を構想しています\
(電子契約は正式な許可が下りるまで未実装のサンプル・デモ段階です)。\
詳しくは https://e-gov.info/realestate をご覧ください。",
    },
];

const FALLBACK_REPLY: &str = "e-gov.infoへようこそ。\
「申請したい」「買いたい」「仕入れたい」「土地を探したい」のように\
教えていただければ、該当するページをご案内します。\
(本メッセージはルールベースの応答です。実際のLLM連携は未実装、詳しくは e-gov.info/research をご覧ください)";

/// LINEでの全ての返信の先頭に必ず付ける、英語・日本語併記のサンプル
/// 注意書き(ユーザー指示、2026-07-18: WEBサイトだけでなくLINE友だに
/// なっても毎回表示すること)。
pub const SAMPLE_BANNER: &str = "⚠️ THIS IS STILL A SAMPLE / DEMONSTRATION ONLY — NOT A LIVE SERVICE\n\
⚠️ これはまだサンプル・デモンストレーションです(実際のサービスではありません)\n\n";

/// ユーザーのメッセージ本文から、キーワードマッチングで案内文を選ぶ。
/// 大文字・小文字を無視し、複数キーワードにマッチした場合は最初に定義
/// された`INTENTS`の順序を優先する(単純化のためスコアリングはしない)。
/// 返信の先頭には必ず`SAMPLE_BANNER`(英日併記のサンプル注意書き)を
/// 付ける——WEBサイトのバナーと同様、LINE経由でも毎回表示するため。
pub fn reply_for(user_text: &str) -> String {
    let lower = user_text.to_lowercase();
    for intent in INTENTS {
        if intent.keywords.iter().any(|kw| lower.contains(&kw.to_lowercase())) {
            return format!("{SAMPLE_BANNER}{}", intent.reply);
        }
    }
    format!("{SAMPLE_BANNER}{FALLBACK_REPLY}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_government_keyword() {
        assert!(reply_for("マイナンバーカードの申請をしたい").contains("eガバメント"));
    }

    #[test]
    fn matches_trade_keyword_case_insensitively() {
        assert!(reply_for("I want to BUY a speaker").contains("オンライン貿易"));
    }

    #[test]
    fn matches_credit_keyword() {
        assert!(reply_for("掛け仕入れについて教えて").contains("与信"));
    }

    #[test]
    fn matches_realestate_keyword() {
        assert!(reply_for("土地を探しています").contains("不動産"));
    }

    #[test]
    fn falls_back_for_unmatched_text() {
        assert_eq!(reply_for("こんにちは"), format!("{SAMPLE_BANNER}{FALLBACK_REPLY}"));
    }

    #[test]
    fn every_reply_starts_with_the_bilingual_sample_banner() {
        for text in ["申請したい", "buy a car", "掛け仕入れ", "土地を探す", "何でもない文章"] {
            let reply = reply_for(text);
            assert!(reply.starts_with(SAMPLE_BANNER), "reply for {text:?} did not start with the sample banner: {reply}");
        }
    }
}
