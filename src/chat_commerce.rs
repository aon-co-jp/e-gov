//! AIチャットコマースの応答ロジック(LINE Webhook・将来のWeb版チャット
//! 双方から共有される想定)。多言語対応(2026-07-18、`i18n::Lang`の13言語)。
//!
//! **正直な開示(現状のスコープ)**: 「AI」チャットコマースと銘打っている
//! ものの、現時点では外部LLM APIを一切呼び出さない**ルールベースの
//! キーワードマッチング**による応答に留まる。理由は`marketing.rs`と同様、
//! 規制・コスト・応答内容の予測可能性の観点から、まず決定的でテスト
//! しやすい実装から始めるため。将来、実際のLLM連携(与信情報等の機微な
//! 判断はLLMに丸投げせず、あくまで案内・絞り込みに用途を限定する設計と
//! すべき)に置き換える場合は、この関数のシグネチャを保ったまま内部実装
//! だけ差し替えられるようにしてある。
//!
//! **言語判定も同様にヒューリスティック(スクリプト/文字種ベース)であり、
//! 本格的な言語識別ライブラリ(fastText言語判定等)は使っていない**。
//! 呼び出し側(`line_webhook.rs`)は、LINEのプロフィールAPIから取得できる
//! ユーザーの言語設定を優先し、それが得られない場合(アクセストークン
//! 未設定・API失敗等)にのみ、このモジュールの`detect_lang`にメッセージ
//! 本文を渡して簡易判定する、という2段構成にする想定。

use crate::i18n::{self, Lang};

/// `aruaru-llm`(エコシステム共通のAIチャットコマース応答サービス、
/// [aon-co-jp/aruaru-llm](https://github.com/aon-co-jp/aruaru-llm))の
/// 既定リッスンアドレス(`aruaru-llm/src/main.rs`の`bind_addr`と同じ値)。
/// 環境変数`E_GOV_ARUARU_LLM_URL`未設定時はこのローカルアドレスを使う。
/// **注意**: これはURLのデフォルト値であり、`aruaru-llm`プロセス自体は
/// 別途起動しておく必要がある(未起動・疎通不可の場合は下記の通り
/// ローカルのルールベース応答へ自動フォールバックする)。
const DEFAULT_ARUARU_LLM_URL: &str = "http://127.0.0.1:4600";

#[derive(Debug, serde::Deserialize)]
struct AruaruLlmChatResponse {
    reply: String,
    #[serde(default)]
    engine: Option<String>,
}

/// `aruaru-llm`の`POST /v1/chat`へ実際にHTTP問い合わせを行う。
/// 疎通不可・タイムアウト・非2xx・パース失敗など、いずれの失敗経路でも
/// `None`を返す(呼び出し側`reply_for_async`がローカルのルールベース応答
/// へフォールバックする、「グレースフルデグラデーション、サイレント
/// 失敗はしない」というこのエコシステムの方針に従い、必ず`tracing::warn!`
/// でログを残す)。
async fn try_llm_reply(user_text: &str) -> Option<String> {
    let base = std::env::var("E_GOV_ARUARU_LLM_URL").unwrap_or_else(|_| DEFAULT_ARUARU_LLM_URL.to_string());
    let url = format!("{base}/v1/chat");

    let client = match reqwest::Client::builder().timeout(std::time::Duration::from_secs(3)).build() {
        Ok(c) => c,
        Err(err) => {
            tracing::warn!("failed to build reqwest client for aruaru-llm: {err}, falling back to local rule-based reply");
            return None;
        }
    };

    let resp = match client
        .post(&url)
        .json(&serde_json::json!({ "message": user_text, "tenant": "e-gov.info" }))
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            tracing::warn!("aruaru-llm unreachable at {url}: {err}, falling back to local rule-based reply");
            return None;
        }
    };

    if !resp.status().is_success() {
        tracing::warn!("aruaru-llm returned non-2xx status {} from {url}, falling back to local rule-based reply", resp.status());
        return None;
    }

    match resp.json::<AruaruLlmChatResponse>().await {
        Ok(parsed) => {
            tracing::info!("aruaru-llm replied (engine={:?})", parsed.engine);
            Some(parsed.reply)
        }
        Err(err) => {
            tracing::warn!("failed to parse aruaru-llm response from {url}: {err}, falling back to local rule-based reply");
            None
        }
    }
}

/// 各カテゴリの多言語キーワードと、`i18n`の該当ページ本文を返す関数。
struct Intent {
    keywords: &'static [&'static str],
    text_for: fn(Lang) -> i18n::PageText,
}

const INTENTS: &[Intent] = &[
    Intent {
        keywords: &[
            "申請", "手続き", "行政", "役所", "マイナンバー", "government", "application", "apply",
            "数字政府", "申请", "數位政府", "申請", "신청", "정부", "richiesta", "governo", "demande",
            "gouvernement", "antrag", "regierung", "طلب", "حكومة", "درخواست", "دولت", "заявка",
            "государство", "уряд",
        ],
        text_for: i18n::gov_text,
    },
    Intent {
        keywords: &[
            "買いたい", "欲しい", "注文", "商品", "buy", "want", "order", "product", "购买", "商品",
            "購買", "구매", "주문", "acquisto", "achat", "commande", "kauf", "bestellung", "شراء",
            "منتج", "خرید", "سفارش", "покупка", "заказ", "купівля", "замовлення",
        ],
        text_for: i18n::trade_text,
    },
    Intent {
        keywords: &[
            "仕入れ", "与信", "掛け", "売掛", "請求書", "credit", "invoice", "信用", "赊购", "賒購",
            "신용", "외상매입", "credito", "crédit", "kredit", "ائتمان", "فاتورة", "اعتبار", "صورت‌حساب",
            "кредит", "счёт", "рахунок",
        ],
        text_for: i18n::credit_text,
    },
    Intent {
        keywords: &[
            "不動産", "土地", "間取り", "工務店", "賃貸", "real estate", "land", "house", "不动产",
            "土地", "부동산", "토지", "immobiliare", "terreno", "immobilier", "terrain", "immobilien",
            "grundstück", "عقار", "أرض", "املاک", "زمین", "недвижимость", "земля", "нерухомість",
        ],
        text_for: i18n::realestate_text,
    },
];

fn fallback_reply(lang: Lang) -> String {
    match lang {
        Lang::En | Lang::EnGb => "Welcome to e-gov.info. Try telling me things like \"apply\", \"buy\", \"trade credit\", or \"land search\" and I'll point you to the right page. (This is a rule-based reply — real LLM integration is not implemented yet, see e-gov.info/research)".to_string(),
        Lang::Ja => "e-gov.infoへようこそ。「申請したい」「買いたい」「仕入れたい」「土地を探したい」のように教えていただければ、該当するページをご案内します。(本メッセージはルールベースの応答です。実際のLLM連携は未実装、詳しくは e-gov.info/research をご覧ください)".to_string(),
        Lang::ZhCn => "欢迎使用e-gov.info。请告诉我「申请」「购买」「赊购」「寻找土地」等关键词，我会为您指引相应页面。(本消息为规则型回复，尚未接入真正的LLM，详情请见 e-gov.info/research)".to_string(),
        Lang::ZhTw => "歡迎使用e-gov.info。請告訴我「申請」「購買」「賒購」「尋找土地」等關鍵詞，我會為您指引對應頁面。(本訊息為規則型回覆，尚未串接真正的LLM，詳情請見 e-gov.info/research)".to_string(),
        Lang::Ko => "e-gov.info에 오신 것을 환영합니다. \"신청\", \"구매\", \"외상매입\", \"토지 검색\"과 같이 말씀해 주시면 해당 페이지를 안내해드립니다. (본 메시지는 규칙 기반 응답이며 실제 LLM 연동은 아직 구현되지 않았습니다. 자세한 내용은 e-gov.info/research 참조)".to_string(),
        Lang::It => "Benvenuto su e-gov.info. Prova a dirmi cose come \"richiesta\", \"acquisto\", \"credito commerciale\" o \"ricerca terreno\" e ti indirizzerò alla pagina giusta. (Questa è una risposta basata su regole — l'integrazione LLM reale non è ancora implementata, vedi e-gov.info/research)".to_string(),
        Lang::Fr => "Bienvenue sur e-gov.info. Essayez de me dire des choses comme « demande », « achat », « crédit commercial » ou « recherche de terrain » et je vous orienterai vers la bonne page. (Ceci est une réponse basée sur des règles — l'intégration IA réelle n'est pas encore implémentée, voir e-gov.info/research)".to_string(),
        Lang::De => "Willkommen bei e-gov.info. Sagen Sie mir zum Beispiel „Antrag“, „Kauf“, „Handelskredit“ oder „Grundstückssuche“, und ich verweise Sie auf die passende Seite. (Dies ist eine regelbasierte Antwort — echte LLM-Integration ist noch nicht umgesetzt, siehe e-gov.info/research)".to_string(),
        Lang::Ar => "مرحباً بك في e-gov.info. أخبرني بكلمات مثل \"طلب\" أو \"شراء\" أو \"ائتمان تجاري\" أو \"البحث عن أرض\" وسأوجهك إلى الصفحة المناسبة. (هذا رد قائم على القواعد — التكامل الفعلي مع الذكاء الاصطناعي لم يُنفَّذ بعد، راجع e-gov.info/research)".to_string(),
        Lang::Fa => "به e-gov.info خوش آمدید. کافی است چیزهایی مانند «درخواست»، «خرید»، «اعتبار تجاری» یا «جست‌وجوی زمین» بگویید تا شما را به صفحهٔ مناسب راهنمایی کنم. (این پاسخ مبتنی بر قاعده است — یکپارچه‌سازی واقعی هوش مصنوعی هنوز پیاده‌سازی نشده، به e-gov.info/research مراجعه کنید)".to_string(),
        Lang::Ru => "Добро пожаловать на e-gov.info. Просто напишите «заявка», «покупка», «торговый кредит» или «поиск участка», и я направлю вас на нужную страницу. (Это ответ на основе правил — реальная интеграция ИИ ещё не реализована, см. e-gov.info/research)".to_string(),
        Lang::Uk => "Ласкаво просимо до e-gov.info. Просто напишіть «заявка», «купівля», «торговий кредит» або «пошук ділянки», і я направлю вас на потрібну сторінку. (Це відповідь на основі правил — реальна інтеграція ШІ ще не реалізована, див. e-gov.info/research)".to_string(),
    }
}

/// メッセージ本文のUnicodeスクリプト/特徴的な単語からユーザーの言語を
/// 推定する簡易ヒューリスティック(本格的な言語識別ライブラリは不使用)。
/// 判定できない場合は既定言語の英語を返す。
pub fn detect_lang(text: &str) -> Lang {
    let has_range = |lo: u32, hi: u32| text.chars().any(|c| (c as u32) >= lo && (c as u32) <= hi);

    // 日本語: ひらがな・カタカナの有無(漢字だけの中国語と区別できる)。
    if has_range(0x3040, 0x30FF) {
        return Lang::Ja;
    }
    // 韓国語: ハングル。
    if has_range(0xAC00, 0xD7A3) {
        return Lang::Ko;
    }
    // アラビア文字圏: ペルシャ語固有の文字(پ/چ/ژ/گ)があればペルシャ語、
    // なければアラビア語。
    if has_range(0x0600, 0x06FF) {
        let persian_only = ['پ', 'چ', 'ژ', 'گ'];
        if text.chars().any(|c| persian_only.contains(&c)) {
            return Lang::Fa;
        }
        return Lang::Ar;
    }
    // キリル文字圏: ウクライナ語固有の文字(і/ї/є/ґ)があればウクライナ語、
    // なければロシア語。
    if has_range(0x0400, 0x04FF) {
        let ukrainian_only = ['і', 'ї', 'є', 'ґ', 'І', 'Ї', 'Є', 'Ґ'];
        if text.chars().any(|c| ukrainian_only.contains(&c)) {
            return Lang::Uk;
        }
        return Lang::Ru;
    }
    // 漢字(ひらがな/カタカナ無し) → 中国語。簡体字/繁体字の判定は、
    // 繁体字特有の文字(們/國/來/這等)の有無で行う。
    if has_range(0x4E00, 0x9FFF) {
        let traditional_markers = ['們', '國', '來', '這', '對', '學'];
        if text.chars().any(|c| traditional_markers.contains(&c)) {
            return Lang::ZhTw;
        }
        return Lang::ZhCn;
    }
    // ラテン文字圏: 特徴的な単語で伊/仏/独を判定、それ以外は英語既定。
    let lower = text.to_lowercase();
    let word_present = |words: &[&str]| words.iter().any(|w| lower.contains(w));
    if word_present(&["è", "perché", "grazie", " il ", " che ", "richiesta"]) {
        return Lang::It;
    }
    if word_present(&["é", "è", "ç", " le ", " je ", " demande", "merci"]) {
        return Lang::Fr;
    }
    if word_present(&["ä", "ö", "ü", "ß", " der ", " und ", "antrag"]) {
        return Lang::De;
    }
    Lang::En
}

/// ユーザーのメッセージ本文から、キーワードマッチングで案内文を選ぶ。
/// `lang`は返信本文の言語(呼び出し側がLINEプロフィールAPIまたは
/// `detect_lang`で決定して渡す)。返信の先頭には必ず、そのページの
/// バナー文言(`i18n::common(lang).banner_local`+英語)を付ける。
pub fn reply_for(lang: Lang, user_text: &str) -> String {
    let common = i18n::common(lang);
    let banner = if common.banner_local == common.banner_en {
        format!("{}\n\n", common.banner_en)
    } else {
        format!("{}\n{}\n\n", common.banner_en, common.banner_local)
    };

    let lower = user_text.to_lowercase();
    for intent in INTENTS {
        if intent.keywords.iter().any(|kw| lower.contains(&kw.to_lowercase())) {
            let text = (intent.text_for)(lang);
            return format!("{banner}{}\n{}", text.h1, text.body);
        }
    }
    format!("{banner}{}", fallback_reply(lang))
}

/// `reply_for`のHTTP版。まず`aruaru-llm`(エコシステム共通のAIチャット
/// コマース応答サービス、意味的類似度分類ベース)へ問い合わせ、成功すれば
/// その応答をバナー付きで返す。`aruaru-llm`が未起動・疎通不可・タイムアウト・
/// 非2xx・レスポンス形式不正のいずれかの場合は、`try_llm_reply`が既に
/// `tracing::warn!`でログを残した上で`None`を返してくるので、ここでは
/// 静かに(エラーにせず)ローカルのルールベース`reply_for`へフォールバック
/// する。呼び出し側(`line_webhook.rs`)はこの関数だけを呼べばよい。
///
/// **正直な開示**: `aruaru-llm`側の定型応答文は現状すべて日本語である
/// (`aruaru-llm/src/scoring.rs`参照、多言語化は未対応)。そのため
/// `lang`が日本語以外の場合でも、`aruaru-llm`到達時は本文が日本語のまま
/// 返る点に注意(バナー部分のみは引き続き`lang`に応じて翻訳される)。
pub async fn reply_for_async(lang: Lang, user_text: &str) -> String {
    if let Some(llm_reply) = try_llm_reply(user_text).await {
        let common = i18n::common(lang);
        let banner = if common.banner_local == common.banner_en {
            format!("{}\n\n", common.banner_en)
        } else {
            format!("{}\n{}\n\n", common.banner_en, common.banner_local)
        };
        return format!("{banner}{llm_reply}");
    }
    reply_for(lang, user_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_government_keyword_in_japanese() {
        let reply = reply_for(Lang::Ja, "マイナンバーカードの申請をしたい");
        assert!(reply.contains("eガバメント") || reply.contains("デジタルガバメント"));
    }

    #[test]
    fn matches_trade_keyword_case_insensitively_in_english() {
        let reply = reply_for(Lang::En, "I want to BUY a speaker");
        assert!(reply.to_lowercase().contains("trade"));
    }

    #[test]
    fn matches_credit_keyword_in_chinese() {
        let reply = reply_for(Lang::ZhCn, "我想赊购一批货物");
        assert!(reply.contains("信用"));
    }

    #[test]
    fn matches_realestate_keyword_in_arabic() {
        let reply = reply_for(Lang::Ar, "أبحث عن أرض للبناء");
        assert!(reply.contains("العقار"));
    }

    #[test]
    fn falls_back_for_unmatched_text() {
        let reply = reply_for(Lang::En, "hello there");
        assert!(reply.contains("Welcome"));
    }

    #[test]
    fn every_reply_starts_with_the_localized_banner() {
        for lang in Lang::ALL {
            let reply = reply_for(*lang, "unmatched text");
            assert!(reply.contains("SAMPLE"), "reply for {:?} missing sample banner: {reply}", lang.code());
        }
    }

    #[test]
    fn detects_japanese_from_kana() {
        assert_eq!(detect_lang("これは日本語のテストです"), Lang::Ja);
    }

    #[test]
    fn detects_korean_from_hangul() {
        assert_eq!(detect_lang("이것은 한국어 테스트입니다"), Lang::Ko);
    }

    #[test]
    fn detects_simplified_vs_traditional_chinese() {
        assert_eq!(detect_lang("这是简体中文测试"), Lang::ZhCn);
        assert_eq!(detect_lang("這是繁體中文測試，我們國家"), Lang::ZhTw);
    }

    #[test]
    fn detects_arabic_vs_persian() {
        assert_eq!(detect_lang("هذا اختبار باللغة العربية"), Lang::Ar);
        assert_eq!(detect_lang("این پیام به زبان فارسی است"), Lang::Fa);
    }

    #[test]
    fn detects_russian_vs_ukrainian() {
        assert_eq!(detect_lang("это тест на русском языке"), Lang::Ru);
        assert_eq!(detect_lang("це тест українською мовою"), Lang::Uk);
    }

    #[test]
    fn defaults_to_english_for_plain_latin_text() {
        assert_eq!(detect_lang("hello world"), Lang::En);
    }
}
