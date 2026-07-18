//! サイト本体の多言語切り替え(クエリパラメータ`?lang=`方式)。
//!
//! 対応言語はCLAUDE.mdで定めた基本13言語セットと同じ
//! (README-<言語>.mdと同じ翻訳内容を、ページ表示用に再利用・要約)。
//! **既定言語は英語**(ユーザー指示: 「実際のサイトは最初は、英語を
//! 基本として」)。日本語を含む他12言語は`?lang=`クエリパラメータで
//! 明示的に選択する。
//!
//! **正直な開示(スコープの限界)**: TOPページ(`/`)は各言語で本文の
//! 詳細度をなるべく揃えているが、サブページ(gov/trade/credit/
//! realestate/research)は簡潔な要約文で統一しており、日本語版の
//! 冒頭に書かれていたような箇条書きの詳細さまでは13言語すべてに
//! 展開していない(翻訳量が現実的な範囲に収まるようにする設計判断)。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    En,
    EnGb,
    Ja,
    ZhCn,
    ZhTw,
    Ko,
    It,
    Fr,
    De,
    Ar,
    Fa,
    Ru,
    Uk,
}

impl Lang {
    pub const ALL: &'static [Lang] = &[
        Lang::En, Lang::EnGb, Lang::Ja, Lang::ZhCn, Lang::ZhTw, Lang::Ko, Lang::It, Lang::Fr, Lang::De, Lang::Ar,
        Lang::Fa, Lang::Ru, Lang::Uk,
    ];

    /// URLクエリパラメータで使うコード(`?lang=xx`)。
    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::EnGb => "en-gb",
            Lang::Ja => "ja",
            Lang::ZhCn => "zh-cn",
            Lang::ZhTw => "zh-tw",
            Lang::Ko => "ko",
            Lang::It => "it",
            Lang::Fr => "fr",
            Lang::De => "de",
            Lang::Ar => "ar",
            Lang::Fa => "fa",
            Lang::Ru => "ru",
            Lang::Uk => "uk",
        }
    }

    /// `<html lang="...">`属性用(BCP 47)。
    pub fn html_lang(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::EnGb => "en-GB",
            Lang::Ja => "ja",
            Lang::ZhCn => "zh-CN",
            Lang::ZhTw => "zh-TW",
            Lang::Ko => "ko",
            Lang::It => "it",
            Lang::Fr => "fr",
            Lang::De => "de",
            Lang::Ar => "ar",
            Lang::Fa => "fa",
            Lang::Ru => "ru",
            Lang::Uk => "uk",
        }
    }

    /// 言語切替ナビに表示する現地語名。
    pub fn native_name(self) -> &'static str {
        match self {
            Lang::En => "English",
            Lang::EnGb => "English (UK)",
            Lang::Ja => "日本語",
            Lang::ZhCn => "简体中文",
            Lang::ZhTw => "繁體中文",
            Lang::Ko => "한국어",
            Lang::It => "Italiano",
            Lang::Fr => "Français",
            Lang::De => "Deutsch",
            Lang::Ar => "العربية",
            Lang::Fa => "فارسی",
            Lang::Ru => "Русский",
            Lang::Uk => "Українська",
        }
    }

    pub fn is_rtl(self) -> bool {
        matches!(self, Lang::Ar | Lang::Fa)
    }

    /// クエリパラメータの値から`Lang`を決定する。未知の値・未指定の
    /// 場合は既定言語の英語にフォールバックする。
    pub fn parse(code: Option<&str>) -> Lang {
        match code.map(str::to_lowercase).as_deref() {
            Some("ja") => Lang::Ja,
            Some("en-gb") | Some("en_gb") | Some("engb") => Lang::EnGb,
            Some("zh-cn") | Some("zh_cn") | Some("zhcn") | Some("zh") => Lang::ZhCn,
            Some("zh-tw") | Some("zh_tw") | Some("zhtw") => Lang::ZhTw,
            Some("ko") => Lang::Ko,
            Some("it") => Lang::It,
            Some("fr") => Lang::Fr,
            Some("de") => Lang::De,
            Some("ar") => Lang::Ar,
            Some("fa") => Lang::Fa,
            Some("ru") => Lang::Ru,
            Some("uk") => Lang::Uk,
            Some("en") => Lang::En,
            _ => Lang::En,
        }
    }
}

/// ナビゲーション・フッター等、全ページ共通の文言。
pub struct CommonStrings {
    pub nav_top: &'static str,
    pub nav_gov: &'static str,
    pub nav_trade: &'static str,
    pub nav_credit: &'static str,
    pub nav_realestate: &'static str,
    pub nav_research: &'static str,
    pub banner_en: &'static str,
    pub banner_local: &'static str,
    pub footer: &'static str,
}

pub fn common(lang: Lang) -> CommonStrings {
    const BANNER_EN: &str = "⚠️ THIS IS STILL A SAMPLE / DEMONSTRATION ONLY — NOT A LIVE SERVICE";

    match lang {
        Lang::En => CommonStrings {
            nav_top: "TOP", nav_gov: "Digital Gov", nav_trade: "Online Trade", nav_credit: "Credit & Guarantee",
            nav_realestate: "Real Estate", nav_research: "Auto Research", banner_en: BANNER_EN, banner_local: BANNER_EN,
            footer: "e-gov.info — Digital government x online trade platform (concept stage).",
        },
        Lang::EnGb => CommonStrings {
            nav_top: "TOP", nav_gov: "Digital Gov", nav_trade: "Online Trade", nav_credit: "Credit & Guarantee",
            nav_realestate: "Real Estate", nav_research: "Auto Research", banner_en: BANNER_EN, banner_local: BANNER_EN,
            footer: "e-gov.info — Digital government x online trade platform (concept stage).",
        },
        Lang::Ja => CommonStrings {
            nav_top: "TOP", nav_gov: "eガバメント", nav_trade: "オンライン貿易", nav_credit: "与信・売掛保証",
            nav_realestate: "不動産・AI工務店", nav_research: "自動調査・マーケティング", banner_en: BANNER_EN,
            banner_local: "⚠️ これはまだサンプル・デモンストレーションです(実際のサービスではありません)",
            footer: "e-gov.info — デジタルガバメント × オンライン貿易プラットフォーム(構想段階)。",
        },
        Lang::ZhCn => CommonStrings {
            nav_top: "首页", nav_gov: "数字政府", nav_trade: "在线贸易", nav_credit: "信用与担保",
            nav_realestate: "不动产", nav_research: "自动调查", banner_en: BANNER_EN,
            banner_local: "⚠️ 本项目目前仅为样品/演示，并非正式服务",
            footer: "e-gov.info — 数字政府 × 在线贸易平台(构想阶段)。",
        },
        Lang::ZhTw => CommonStrings {
            nav_top: "首頁", nav_gov: "數位政府", nav_trade: "線上貿易", nav_credit: "信用與保證",
            nav_realestate: "不動產", nav_research: "自動調查", banner_en: BANNER_EN,
            banner_local: "⚠️ 本專案目前僅為樣品/展示，並非正式服務",
            footer: "e-gov.info — 數位政府 × 線上貿易平台(構想階段)。",
        },
        Lang::Ko => CommonStrings {
            nav_top: "TOP", nav_gov: "디지털 정부", nav_trade: "온라인 무역", nav_credit: "신용・보증",
            nav_realestate: "부동산", nav_research: "자동 조사", banner_en: BANNER_EN,
            banner_local: "⚠️ 본 프로젝트는 아직 샘플/데모입니다. 실제 서비스가 아닙니다",
            footer: "e-gov.info — 디지털 정부 × 온라인 무역 플랫폼(구상 단계).",
        },
        Lang::It => CommonStrings {
            nav_top: "TOP", nav_gov: "Governo digitale", nav_trade: "Commercio online", nav_credit: "Credito e garanzia",
            nav_realestate: "Immobiliare", nav_research: "Ricerca automatica", banner_en: BANNER_EN,
            banner_local: "⚠️ QUESTO È ANCORA SOLO UN CAMPIONE / DIMOSTRAZIONE — NON UN SERVIZIO ATTIVO",
            footer: "e-gov.info — Governo digitale x piattaforma di commercio online (fase concettuale).",
        },
        Lang::Fr => CommonStrings {
            nav_top: "TOP", nav_gov: "Gouv. numérique", nav_trade: "Commerce en ligne", nav_credit: "Crédit et garantie",
            nav_realestate: "Immobilier", nav_research: "Recherche auto", banner_en: BANNER_EN,
            banner_local: "⚠️ CECI N'EST ENCORE QU'UN ÉCHANTILLON / UNE DÉMONSTRATION",
            footer: "e-gov.info — Gouvernement numérique x plateforme de commerce en ligne (phase conceptuelle).",
        },
        Lang::De => CommonStrings {
            nav_top: "TOP", nav_gov: "Digitalregierung", nav_trade: "Online-Handel", nav_credit: "Kredit & Garantie",
            nav_realestate: "Immobilien", nav_research: "Auto-Recherche", banner_en: BANNER_EN,
            banner_local: "⚠️ DIES IST NOCH LEDIGLICH EIN MUSTER / EINE DEMONSTRATION",
            footer: "e-gov.info — Digitale Regierung x Online-Handelsplattform (Konzeptphase).",
        },
        Lang::Ar => CommonStrings {
            nav_top: "الرئيسية", nav_gov: "الحكومة الرقمية", nav_trade: "التجارة الإلكترونية", nav_credit: "الائتمان والضمان",
            nav_realestate: "العقارات", nav_research: "بحث آلي", banner_en: BANNER_EN,
            banner_local: "⚠️ هذا لا يزال مجرد نموذج/عرض توضيحي فقط — وليس خدمة فعلية",
            footer: "e-gov.info — الحكومة الرقمية × منصة التجارة الإلكترونية (مرحلة المفهوم).",
        },
        Lang::Fa => CommonStrings {
            nav_top: "خانه", nav_gov: "دولت دیجیتال", nav_trade: "تجارت آنلاین", nav_credit: "اعتبار و ضمانت",
            nav_realestate: "املاک", nav_research: "پژوهش خودکار", banner_en: BANNER_EN,
            banner_local: "⚠️ این هنوز فقط یک نمونه/نسخهٔ نمایشی است — یک سرویس واقعی نیست",
            footer: "e-gov.info — دولت دیجیتال × پلتفرم تجارت آنلاین (مرحلهٔ مفهومی).",
        },
        Lang::Ru => CommonStrings {
            nav_top: "Главная", nav_gov: "Цифр. гос-во", nav_trade: "Онлайн-торговля", nav_credit: "Кредит и гарантия",
            nav_realestate: "Недвижимость", nav_research: "Авто-исследование", banner_en: BANNER_EN,
            banner_local: "⚠️ ЭТО ВСЁ ЕЩЁ ТОЛЬКО ОБРАЗЕЦ / ДЕМОНСТРАЦИЯ",
            footer: "e-gov.info — Цифровое государство x платформа онлайн-торговли (концептуальный этап).",
        },
        Lang::Uk => CommonStrings {
            nav_top: "Головна", nav_gov: "Цифр. уряд", nav_trade: "Онлайн-торгівля", nav_credit: "Кредит і гарантія",
            nav_realestate: "Нерухомість", nav_research: "Авто-дослідження", banner_en: BANNER_EN,
            banner_local: "⚠️ ЦЕ ВСЕ ЩЕ ЛИШЕ ЗРАЗОК / ДЕМОНСТРАЦІЯ",
            footer: "e-gov.info — Цифровий уряд x платформа онлайн-торгівлі (концептуальний етап).",
        },
    }
}

/// ページ本文(タイトル・見出し・要約文)。TOPページはやや詳しく、
/// サブページは簡潔な要約で統一する(スコープの限界、モジュール冒頭参照)。
pub struct PageText {
    pub title: &'static str,
    pub h1: &'static str,
    pub body: &'static str,
}

pub fn index_text(lang: Lang) -> PageText {
    match lang {
        Lang::En | Lang::EnGb => PageText {
            title: "e-gov.info — Digital Government x Online Trade",
            h1: "e-gov.info",
            body: "A project unifying digital government (paperless administrative services via LINE, web, and convenience-store terminals) and an online trade / real-estate platform (AI chat commerce, credit scoring, AI home-builder) for individuals through trading companies.",
        },
        Lang::Ja => PageText {
            title: "e-gov.info — デジタルガバメント×オンライン貿易",
            h1: "e-gov.info",
            body: "行政のデジタル化と、個人〜貿易商社まで対応するオンライン貿易・不動産プラットフォームを、LINEアプリ・WEBサイト・コンビニ端末という複数の入り口から利用できる形で統合する構想です。",
        },
        Lang::ZhCn => PageText {
            title: "e-gov.info — 数字政府 × 在线贸易",
            h1: "e-gov.info",
            body: "整合数字政府(通过LINE、网站、便利店终端实现无纸化行政手续)与面向个人到贸易公司的在线贸易/不动产平台(AI聊天商务、信用评分、AI建筑事务所)的项目。",
        },
        Lang::ZhTw => PageText {
            title: "e-gov.info — 數位政府 × 線上貿易",
            h1: "e-gov.info",
            body: "整合數位政府(透過LINE、網站、便利商店終端實現無紙化行政手續)與面向個人到貿易公司的線上貿易/不動產平台(AI聊天商務、信用評分、AI建築事務所)的專案。",
        },
        Lang::Ko => PageText {
            title: "e-gov.info — 디지털 정부 × 온라인 무역",
            h1: "e-gov.info",
            body: "디지털 정부(LINE・웹・편의점 단말기를 통한 무서류 행정)와 개인부터 무역상사까지 대응하는 온라인 무역/부동산 플랫폼(AI 챗커머스, 신용평가, AI 건축사무소)을 통합하는 프로젝트입니다.",
        },
        Lang::It => PageText {
            title: "e-gov.info — Governo digitale x commercio online",
            h1: "e-gov.info",
            body: "Un progetto che unifica il governo digitale (pratiche senza carta via LINE, web e terminali nei convenience store) e una piattaforma di commercio online/immobiliare (AI chat commerce, credit scoring, AI home-builder) per privati fino alle società commerciali.",
        },
        Lang::Fr => PageText {
            title: "e-gov.info — Gouvernement numérique x commerce en ligne",
            h1: "e-gov.info",
            body: "Un projet unifiant le gouvernement numérique (démarches sans papier via LINE, web et terminaux en supérette) et une plateforme de commerce en ligne/immobilier (commerce conversationnel par IA, notation de crédit, IA constructeur) pour les particuliers jusqu'aux sociétés commerciales.",
        },
        Lang::De => PageText {
            title: "e-gov.info — Digitalregierung x Online-Handel",
            h1: "e-gov.info",
            body: "Ein Projekt, das digitales Regierungshandeln (papierlose Verfahren über LINE, Web und Convenience-Store-Terminals) und eine Online-Handels-/Immobilienplattform (KI-Chat-Commerce, Bonitätsprüfung, KI-Bauunternehmen) für Privatpersonen bis hin zu Handelsgesellschaften vereint.",
        },
        Lang::Ar => PageText {
            title: "e-gov.info — الحكومة الرقمية × التجارة الإلكترونية",
            h1: "e-gov.info",
            body: "مشروع يوحّد الحكومة الرقمية (إجراءات بلا أوراق عبر LINE والويب وأجهزة متاجر الراحة) ومنصة تجارة إلكترونية/عقارية (تجارة دردشة بالذكاء الاصطناعي، تقييم ائتماني، مكتب بناء ذكي) للأفراد وصولاً إلى الشركات التجارية.",
        },
        Lang::Fa => PageText {
            title: "e-gov.info — دولت دیجیتال × تجارت آنلاین",
            h1: "e-gov.info",
            body: "پروژه‌ای که دولت دیجیتال (رویه‌های بدون کاغذ از طریق LINE، وب و پایانه‌های فروشگاهی) و پلتفرم تجارت آنلاین/املاک (تجارت گفتگومحور با هوش مصنوعی، اعتبارسنجی، شرکت‌سازی هوشمند) را برای افراد تا شرکت‌های بازرگانی یکپارچه می‌کند.",
        },
        Lang::Ru => PageText {
            title: "e-gov.info — Цифровое государство x онлайн-торговля",
            h1: "e-gov.info",
            body: "Проект, объединяющий цифровое государственное управление (безбумажные процедуры через LINE, веб и терминалы в магазинах у дома) и платформу онлайн-торговли/недвижимости (ИИ-чат-коммерция, кредитный скоринг, ИИ-застройщик) для частных лиц вплоть до торговых компаний.",
        },
        Lang::Uk => PageText {
            title: "e-gov.info — Цифрова держава x онлайн-торгівля",
            h1: "e-gov.info",
            body: "Проєкт, що об'єднує цифрове державне управління (безпаперові процедури через LINE, веб і термінали в магазинах біля дому) та платформу онлайн-торгівлі/нерухомості (ШІ-чат-комерція, кредитний скоринг, ШІ-забудовник) для приватних осіб аж до торгових компаній.",
        },
    }
}

pub fn gov_text(lang: Lang) -> PageText {
    match lang {
        Lang::En | Lang::EnGb => PageText {
            title: "Digital Government — e-gov.info",
            h1: "Digital Government",
            body: "Paperless applications via LINE, the web, and convenience-store terminals (Loppi, Fami Port, 7-Eleven). Identity verification scales with transaction value: caller-ID + callback for minor inquiries, email OTP for medium amounts, My Number card NFC scan for high-value/legal transactions.",
        },
        Lang::Ja => PageText {
            title: "eガバメント — e-gov.info",
            h1: "eガバメント(デジタルガバメント)",
            body: "LINE・WEB・コンビニ端末(Loppi/Famiポート/セブン-イレブン)からのペーパーレス申請。金額・重要度に応じた段階的本人確認(軽微: ナンバーディスプレイ+折り返し電話、中程度: メールOTP、高額・法的手続き: マイナンバーカードのスマホNFCスキャン)。",
        },
        Lang::ZhCn => PageText {
            title: "数字政府 — e-gov.info",
            h1: "数字政府",
            body: "通过LINE、网站、便利店终端(Loppi、Fami Port、7-Eleven)实现无纸化申请。身份验证强度随交易金额/重要程度分级(轻微：来电显示+回拨；中等：邮件OTP；高额/法律事务：My Number卡手机NFC扫描)。",
        },
        Lang::ZhTw => PageText {
            title: "數位政府 — e-gov.info",
            h1: "數位政府",
            body: "透過LINE、網站、便利商店終端(Loppi、Fami Port、7-Eleven)實現無紙化申請。身分驗證強度依交易金額/重要程度分級(輕微：來電顯示+回撥；中等：電子郵件OTP；高額/法律事務：My Number卡手機NFC掃描)。",
        },
        Lang::Ko => PageText {
            title: "디지털 정부 — e-gov.info",
            h1: "디지털 정부",
            body: "LINE・웹・편의점 단말기(Loppi/Fami Port/세븐일레븐)를 통한 무서류 신청. 거래 금액・중요도에 따른 단계별 본인확인(경미: 발신자표시+콜백, 중간: 이메일 OTP, 고액・법적 절차: 마이넘버카드 스마트폰 NFC 스캔).",
        },
        Lang::It => PageText {
            title: "Governo digitale — e-gov.info",
            h1: "Governo digitale",
            body: "Domande senza carta via LINE, web e terminali nei convenience store (Loppi, Fami Port, 7-Eleven). La verifica dell'identità scala con il valore della transazione: ID chiamante + richiamata per richieste minori, OTP via email per importi medi, scansione NFC della carta My Number per transazioni di alto valore/legali.",
        },
        Lang::Fr => PageText {
            title: "Gouvernement numérique — e-gov.info",
            h1: "Gouvernement numérique",
            body: "Démarches sans papier via LINE, le web et les terminaux en supérette (Loppi, Fami Port, 7-Eleven). La vérification d'identité s'ajuste à la valeur de la transaction : ID appelant + rappel pour les demandes mineures, OTP par e-mail pour les montants moyens, scan NFC de la carte My Number pour les transactions importantes/juridiques.",
        },
        Lang::De => PageText {
            title: "Digitalregierung — e-gov.info",
            h1: "Digitalregierung",
            body: "Papierlose Anträge über LINE, Web und Convenience-Store-Terminals (Loppi, Fami Port, 7-Eleven). Die Identitätsprüfung skaliert mit dem Transaktionswert: Anrufer-ID + Rückruf bei geringfügigen Anfragen, E-Mail-OTP bei mittleren Beträgen, NFC-Scan der My-Number-Karte bei hochwertigen/rechtlichen Transaktionen.",
        },
        Lang::Ar => PageText {
            title: "الحكومة الرقمية — e-gov.info",
            h1: "الحكومة الرقمية",
            body: "طلبات بلا أوراق عبر LINE والويب وأجهزة متاجر الراحة (Loppi، Fami Port، 7-Eleven). يتصاعد التحقق من الهوية مع قيمة المعاملة: تحديد هوية المتصل + معاودة الاتصال للاستفسارات البسيطة، رمز مرور بريدي للمبالغ المتوسطة، مسح NFC لبطاقة My Number للمعاملات عالية القيمة/القانونية.",
        },
        Lang::Fa => PageText {
            title: "دولت دیجیتال — e-gov.info",
            h1: "دولت دیجیتال",
            body: "درخواست‌های بدون کاغذ از طریق LINE، وب و پایانه‌های فروشگاهی (Loppi، Fami Port، 7-Eleven). شدت احراز هویت متناسب با ارزش تراکنش تغییر می‌کند: شناسه تماس‌گیرنده + تماس بازگشتی برای درخواست‌های جزئی، رمز یک‌بارمصرف ایمیلی برای مبالغ متوسط، اسکن NFC کارت My Number برای تراکنش‌های پرارزش/حقوقی.",
        },
        Lang::Ru => PageText {
            title: "Цифровое государство — e-gov.info",
            h1: "Цифровое государственное управление",
            body: "Безбумажные заявки через LINE, веб и терминалы в магазинах у дома (Loppi, Fami Port, 7-Eleven). Проверка личности масштабируется с суммой операции: ID звонящего + обратный звонок для мелких запросов, email-OTP для средних сумм, NFC-сканирование карты My Number для крупных/юридических операций.",
        },
        Lang::Uk => PageText {
            title: "Цифровий уряд — e-gov.info",
            h1: "Цифрове державне управління",
            body: "Безпаперові заявки через LINE, веб і термінали в магазинах біля дому (Loppi, Fami Port, 7-Eleven). Перевірка особи масштабується із сумою операції: ID абонента + зворотний дзвінок для дрібних запитів, email-OTP для середніх сум, NFC-сканування картки My Number для великих/юридичних операцій.",
        },
    }
}

pub fn trade_text(lang: Lang) -> PageText {
    match lang {
        Lang::En | Lang::EnGb => PageText {
            title: "Online Trade Platform — e-gov.info",
            h1: "Online Trade Platform (AI Chat Commerce)",
            body: "A general marketplace, from groceries to cars and audio equipment, discoverable via AI conversation on LINE and the web. For individuals through trading companies, with significantly lower fees than existing platforms. The initial phase runs without real inventory, monetized via ads as a UX demonstration.",
        },
        Lang::Ja => PageText {
            title: "オンライン貿易プラットフォーム — e-gov.info",
            h1: "オンライン貿易プラットフォーム(AIチャットコマース)",
            body: "食料品から自動車・オーディオ機器まで、LINE・WEBでAIとの対話により発見・注文できる総合通販。個人〜貿易商社対応、既存大手より手数料を大幅に引き下げ。初期は実在庫を伴わず、アドセンスによるサンプル運用でUXを検証。",
        },
        Lang::ZhCn => PageText {
            title: "在线贸易平台 — e-gov.info",
            h1: "在线贸易平台(AI聊天商务)",
            body: "从食品到汽车、音响设备，通过LINE和网站上与AI对话即可发现并下单的综合电商。面向个人到贸易公司，手续费远低于现有平台。初期不涉及实际库存，以广告收入方式验证用户体验。",
        },
        Lang::ZhTw => PageText {
            title: "線上貿易平台 — e-gov.info",
            h1: "線上貿易平台(AI聊天商務)",
            body: "從食品到汽車、音響設備，透過LINE和網站與AI對話即可發現並下單的綜合電商。面向個人到貿易公司，手續費遠低於現有平台。初期不涉及實際庫存，以廣告收益方式驗證使用者體驗。",
        },
        Lang::Ko => PageText {
            title: "온라인 무역 플랫폼 — e-gov.info",
            h1: "온라인 무역 플랫폼(AI 챗커머스)",
            body: "식품부터 자동차・오디오 기기까지, LINE과 웹에서 AI와의 대화로 발견・주문하는 종합 쇼핑몰. 개인부터 무역상사까지 대응, 기존 플랫폼보다 수수료 대폭 인하. 초기에는 실재고 없이 애드센스 수익으로 UX를 검증합니다.",
        },
        Lang::It => PageText {
            title: "Piattaforma di commercio online — e-gov.info",
            h1: "Piattaforma di commercio online (AI Chat Commerce)",
            body: "Un marketplace generale, dal cibo alle auto e apparecchiature audio, individuabile tramite conversazione con l'IA su LINE e sul web. Per privati fino alle società commerciali, con commissioni molto più basse. La fase iniziale opera senza inventario reale, monetizzata dalla pubblicità come dimostrazione UX.",
        },
        Lang::Fr => PageText {
            title: "Plateforme de commerce en ligne — e-gov.info",
            h1: "Plateforme de commerce en ligne (commerce conversationnel par IA)",
            body: "Une place de marché générale, de l'alimentaire aux voitures et au matériel audio, trouvable par conversation avec l'IA sur LINE et le web. Pour les particuliers jusqu'aux sociétés commerciales, avec des frais bien plus bas. La phase initiale fonctionne sans stock réel, monétisée par la publicité.",
        },
        Lang::De => PageText {
            title: "Online-Handelsplattform — e-gov.info",
            h1: "Online-Handelsplattform (KI-Chat-Commerce)",
            body: "Ein allgemeiner Marktplatz, von Lebensmitteln bis zu Autos und Audiogeräten, auffindbar per KI-Gespräch auf LINE und im Web. Für Privatpersonen bis hin zu Handelsgesellschaften, mit deutlich niedrigeren Gebühren. Die Anfangsphase läuft ohne echten Lagerbestand, monetarisiert durch Werbung.",
        },
        Lang::Ar => PageText {
            title: "منصة التجارة الإلكترونية — e-gov.info",
            h1: "منصة التجارة الإلكترونية (التجارة عبر الدردشة بالذكاء الاصطناعي)",
            body: "سوق عام، من المواد الغذائية إلى السيارات ومعدات الصوت، يمكن اكتشافه عبر محادثة مع الذكاء الاصطناعي على LINE والويب. للأفراد وصولاً إلى الشركات التجارية، برسوم أقل بكثير. تعمل المرحلة الأولية دون مخزون فعلي، وتموَّل عبر الإعلانات.",
        },
        Lang::Fa => PageText {
            title: "پلتفرم تجارت آنلاین — e-gov.info",
            h1: "پلتفرم تجارت آنلاین (تجارت گفتگومحور با هوش مصنوعی)",
            body: "بازاری عمومی، از مواد غذایی تا خودرو و تجهیزات صوتی، که از طریق گفتگو با هوش مصنوعی در LINE و وب قابل کشف است. برای افراد تا شرکت‌های بازرگانی، با کارمزدهای بسیار کمتر. مرحلهٔ اولیه بدون موجودی واقعی و با درآمد تبلیغاتی اجرا می‌شود.",
        },
        Lang::Ru => PageText {
            title: "Платформа онлайн-торговли — e-gov.info",
            h1: "Платформа онлайн-торговли (ИИ-чат-коммерция)",
            body: "Универсальный маркетплейс — от продуктов до автомобилей и аудиотехники, доступный через диалог с ИИ в LINE и вебе. Для частных лиц вплоть до торговых компаний, с гораздо более низкими комиссиями. Начальный этап работает без реальных запасов, монетизируется рекламой.",
        },
        Lang::Uk => PageText {
            title: "Платформа онлайн-торгівлі — e-gov.info",
            h1: "Платформа онлайн-торгівлі (ШІ-чат-комерція)",
            body: "Універсальний маркетплейс — від продуктів до автомобілів і аудіотехніки, доступний через діалог зі ШІ в LINE і вебі. Для приватних осіб аж до торгових компаній, зі значно нижчими комісіями. Початковий етап працює без реальних запасів, монетизується рекламою.",
        },
    }
}

pub fn credit_text(lang: Lang) -> PageText {
    match lang {
        Lang::En | Lang::EnGb => PageText {
            title: "Credit & Guarantee — e-gov.info",
            h1: "AI Credit Scoring, Trade Credit & Receivables Guarantee",
            body: "Deferred-payment purchasing based on an AI-computed credit score, duplicate e-invoice detection, and accounts-receivable guarantees. This is a demo — real credit screening/guarantee functionality is not implemented yet, pending review of relevant financial regulations.",
        },
        Lang::Ja => PageText {
            title: "与信・売掛保証 — e-gov.info",
            h1: "AI与信調査・掛け仕入れ・電子請求書重複調査・売掛保証",
            body: "AIが算出する与信スコアに応じた後払い仕入れ、電子請求書の重複調査、売掛債権の保証機能。本ページは正式な許可が下りるまでのサンプル・デモンストレーションです。実際に与信審査・保証が実行される機能は搭載していません。",
        },
        Lang::ZhCn => PageText {
            title: "信用与担保 — e-gov.info",
            h1: "AI信用调查・赊购・电子发票重复检测・应收账款担保",
            body: "根据AI计算的信用评分实现延期付款采购、电子发票重复检测、应收账款担保功能。本页面为正式批准前的样品/演示，尚未实现实际的信用审查/担保功能。",
        },
        Lang::ZhTw => PageText {
            title: "信用與保證 — e-gov.info",
            h1: "AI信用調查・賒購・電子發票重複檢測・應收帳款保證",
            body: "依AI計算的信用評分實現延期付款採購、電子發票重複檢測、應收帳款保證功能。本頁面為正式核准前的樣品/展示，尚未實作實際的信用審查/保證功能。",
        },
        Lang::Ko => PageText {
            title: "신용・보증 — e-gov.info",
            h1: "AI 신용조사・외상매입・전자세금계산서 중복조사・매출채권 보증",
            body: "AI가 산출한 신용점수에 따른 후불 매입, 전자세금계산서 중복 조사, 매출채권 보증 기능. 본 페이지는 정식 승인이 내려질 때까지의 샘플・데모이며, 실제 신용심사・보증 기능은 탑재되어 있지 않습니다.",
        },
        Lang::It => PageText {
            title: "Credito e garanzia — e-gov.info",
            h1: "Credit scoring AI, credito commerciale e garanzia sui crediti",
            body: "Acquisti a pagamento differito basati su un punteggio di credito calcolato dall'IA, rilevamento fatture duplicate e garanzie sui crediti. Questa è una demo — le funzioni reali di verifica/garanzia del credito non sono ancora implementate, in attesa di revisione normativa.",
        },
        Lang::Fr => PageText {
            title: "Crédit et garantie — e-gov.info",
            h1: "Notation de crédit par IA, crédit commercial et garantie de créances",
            body: "Achats à paiement différé fondés sur un score de crédit calculé par IA, détection de factures dupliquées et garanties de créances. Ceci est une démo — les fonctions réelles d'évaluation/garantie de crédit ne sont pas encore mises en œuvre, en attente d'un examen réglementaire.",
        },
        Lang::De => PageText {
            title: "Kredit & Garantie — e-gov.info",
            h1: "KI-Bonitätsprüfung, Handelskredit & Forderungsgarantie",
            body: "Kauf auf Ziel auf Basis eines KI-berechneten Bonitätswerts, Erkennung doppelter Rechnungen und Garantien für Forderungen. Dies ist eine Demo — echte Bonitätsprüfungs-/Garantiefunktionen sind noch nicht implementiert, vorbehaltlich regulatorischer Prüfung.",
        },
        Lang::Ar => PageText {
            title: "الائتمان والضمان — e-gov.info",
            h1: "تقييم ائتماني بالذكاء الاصطناعي، ائتمان تجاري وضمان الذمم المدينة",
            body: "شراء بالدفع الآجل استناداً إلى درجة ائتمان يحسبها الذكاء الاصطناعي، وكشف الفواتير المكررة، وضمانات الذمم المدينة. هذا عرض توضيحي — لم تُنفَّذ بعد وظائف التحقق/الضمان الائتماني الفعلية، بانتظار المراجعة التنظيمية.",
        },
        Lang::Fa => PageText {
            title: "اعتبار و ضمانت — e-gov.info",
            h1: "اعتبارسنجی با هوش مصنوعی، اعتبار تجاری و تضمین مطالبات",
            body: "خرید با پرداخت مؤجل بر پایهٔ امتیاز اعتباری محاسبه‌شده توسط هوش مصنوعی، تشخیص فاکتورهای تکراری و تضمین مطالبات. این یک نسخهٔ نمایشی است — عملکرد واقعی اعتبارسنجی/تضمین هنوز پیاده‌سازی نشده و در انتظار بررسی نظارتی است.",
        },
        Lang::Ru => PageText {
            title: "Кредит и гарантия — e-gov.info",
            h1: "ИИ-скоринг, торговый кредит и гарантия дебиторской задолженности",
            body: "Покупки с отсрочкой платежа на основе кредитного рейтинга, рассчитанного ИИ, обнаружение дублирующихся счетов и гарантии по дебиторской задолженности. Это демо — реальные функции проверки/гарантии кредита ещё не реализованы, в ожидании регуляторной проверки.",
        },
        Lang::Uk => PageText {
            title: "Кредит і гарантія — e-gov.info",
            h1: "ШІ-скоринг, торговий кредит і гарантія дебіторської заборгованості",
            body: "Покупки з відстрочкою платежу на основі кредитного рейтингу, розрахованого ШІ, виявлення дубльованих рахунків і гарантії за дебіторською заборгованістю. Це демо — реальні функції перевірки/гарантії кредиту ще не реалізовані, в очікуванні регуляторної перевірки.",
        },
    }
}

pub fn realestate_text(lang: Lang) -> PageText {
    match lang {
        Lang::En | Lang::EnGb => PageText {
            title: "Real Estate & AI Home-Builder — e-gov.info",
            h1: "Real Estate Investment, Electronic Contracts & AI Home-Builder",
            body: "An AI suggests floor plans based on searched land information. Design principle: do not fuel speculative capital inflow into real estate. This page is a demo pending formal approval — electronic notarization/legally binding contracts are not implemented yet.",
        },
        Lang::Ja => PageText {
            title: "不動産・AI工務店 — e-gov.info",
            h1: "不動産投資・電子契約・AI工務店",
            body: "検索した土地情報からAIが間取りを提案。投機的資金の過剰流入を助長しないことを設計原則とします。本ページは正式な許可が下りるまでのサンプル・デモンストレーションで、電子公証・電子契約は現段階では一切実装していません。",
        },
        Lang::ZhCn => PageText {
            title: "不动产・AI建筑事务所 — e-gov.info",
            h1: "不动产投资・电子合同・AI建筑事务所",
            body: "AI根据搜索到的土地信息提出户型方案。设计原则是不助长投机资金过度流入不动产。本页面为正式批准前的样品/演示，电子公证・电子合同现阶段尚未实现。",
        },
        Lang::ZhTw => PageText {
            title: "不動產・AI建築事務所 — e-gov.info",
            h1: "不動產投資・電子合約・AI建築事務所",
            body: "AI依搜尋到的土地資訊提出格局方案。設計原則是不助長投機資金過度流入不動產。本頁面為正式核准前的樣品/展示，電子公證・電子合約現階段尚未實作。",
        },
        Lang::Ko => PageText {
            title: "부동산・AI 건축사무소 — e-gov.info",
            h1: "부동산 투자・전자계약・AI 건축사무소",
            body: "검색한 토지 정보를 바탕으로 AI가 평면도를 제안합니다. 투기 자금의 과도한 유입을 조장하지 않는다는 설계 원칙. 본 페이지는 정식 승인이 내려질 때까지의 샘플・데모이며, 전자공증・전자계약은 현재 구현되어 있지 않습니다.",
        },
        Lang::It => PageText {
            title: "Immobiliare e AI Home-Builder — e-gov.info",
            h1: "Investimento immobiliare, contratti elettronici e AI home-builder",
            body: "Un'IA suggerisce piante in base alle informazioni sul terreno ricercato. Principio di design: non alimentare afflussi speculativi di capitale nel mercato immobiliare. Questa pagina è una demo in attesa di approvazione formale — autenticazione notarile/contratti vincolanti non sono ancora implementati.",
        },
        Lang::Fr => PageText {
            title: "Immobilier et IA constructeur — e-gov.info",
            h1: "Investissement immobilier, contrats électroniques et IA constructeur",
            body: "Une IA propose des plans d'aménagement à partir des informations sur le terrain recherché. Principe de conception : ne pas alimenter un afflux spéculatif de capitaux dans l'immobilier. Cette page est une démo en attente d'approbation formelle — notarisation électronique/contrats contraignants non encore mis en œuvre.",
        },
        Lang::De => PageText {
            title: "Immobilien & KI-Bauunternehmen — e-gov.info",
            h1: "Immobilieninvestition, elektronische Verträge & KI-Bauunternehmen",
            body: "Eine KI schlägt Grundrisse basierend auf recherchierten Grundstücksinformationen vor. Designprinzip: keinen spekulativen Kapitalzufluss in Immobilien befördern. Diese Seite ist eine Demo vorbehaltlich formeller Genehmigung — elektronische Beglaubigung/verbindliche Verträge sind noch nicht umgesetzt.",
        },
        Lang::Ar => PageText {
            title: "العقارات ومكتب البناء بالذكاء الاصطناعي — e-gov.info",
            h1: "الاستثمار العقاري، العقود الإلكترونية ومكتب البناء بالذكاء الاصطناعي",
            body: "يقترح الذكاء الاصطناعي مخططات أرضية بناءً على معلومات الأرض التي تم البحث عنها. مبدأ التصميم: عدم تغذية تدفق رؤوس أموال مضاربة إلى العقارات. هذه الصفحة عرض توضيحي بانتظار الموافقة الرسمية — التوثيق الإلكتروني/العقود الملزمة لم تُنفَّذ بعد.",
        },
        Lang::Fa => PageText {
            title: "املاک و شرکت‌سازی هوشمند — e-gov.info",
            h1: "سرمایه‌گذاری املاک، قراردادهای الکترونیکی و شرکت‌سازی هوشمند",
            body: "هوش مصنوعی بر اساس اطلاعات زمینی که جست‌وجو شده، طرح‌های نقشه پیشنهاد می‌دهد. اصل طراحی: عدم تشویق ورود سرمایهٔ سوداگرانه به املاک. این صفحه نسخهٔ نمایشی در انتظار تأیید رسمی است — تصدیق الکترونیکی/قراردادهای الزام‌آور هنوز پیاده‌سازی نشده‌اند.",
        },
        Lang::Ru => PageText {
            title: "Недвижимость и ИИ-застройщик — e-gov.info",
            h1: "Инвестиции в недвижимость, электронные договоры и ИИ-застройщик",
            body: "ИИ предлагает планировки на основе информации об искомом участке. Принцип проектирования: не поощрять избыточный приток спекулятивного капитала в недвижимость. Эта страница — демо в ожидании официального одобрения; электронное нотариальное заверение/обязывающие договоры пока не реализованы.",
        },
        Lang::Uk => PageText {
            title: "Нерухомість і ШІ-забудовник — e-gov.info",
            h1: "Інвестиції в нерухомість, електронні договори та ШІ-забудовник",
            body: "ШІ пропонує планування на основі інформації про шукану ділянку. Принцип проєктування: не заохочувати надмірний приплив спекулятивного капіталу в нерухомість. Ця сторінка — демо в очікуванні офіційного схвалення; електронне нотаріальне засвідчення/обов'язкові договори ще не реалізовані.",
        },
    }
}

pub fn research_text(lang: Lang) -> PageText {
    match lang {
        Lang::En | Lang::EnGb => PageText {
            title: "Automated Research & Marketing — e-gov.info",
            h1: "Automated Online Marketing & Bilingual Research",
            body: "Periodically researches digital-government topics in Japanese and English (real GitHub Search API calls; Google search links are generated, not scraped) and drafts marketing copy for each feature. Runs automatically on a schedule (default: research every 24h, marketing every 12h) and can be triggered manually with --research-all / --marketing-all.",
        },
        Lang::Ja => PageText {
            title: "自動調査・自動マーケティング — e-gov.info",
            h1: "デジタルガバメントの自動オンラインマーケティング・自動調査機能",
            body: "デジタルガバメント関連テーマを日本語・英語で定期的に自動調査(GitHubは実API、Googleはリンク生成)し、各機能の告知ドラフトも定期生成します。既定では調査24時間ごと・マーケティング12時間ごとに自動実行、手動実行は--research-all/--marketing-allで可能です。",
        },
        Lang::ZhCn => PageText {
            title: "自动调查与营销 — e-gov.info",
            h1: "数字政府的自动在线营销与自动调查功能",
            body: "以日语和英语定期自动调查数字政府相关主题(GitHub使用真实API，Google生成搜索链接)，并定期生成各功能的宣传文案。默认每24小时调查一次、每12小时营销一次，也可通过--research-all/--marketing-all手动执行。",
        },
        Lang::ZhTw => PageText {
            title: "自動調查與行銷 — e-gov.info",
            h1: "數位政府的自動線上行銷與自動調查功能",
            body: "以日語和英語定期自動調查數位政府相關主題(GitHub使用真實API，Google產生搜尋連結)，並定期產生各功能的宣傳文案。預設每24小時調查一次、每12小時行銷一次，也可透過--research-all/--marketing-all手動執行。",
        },
        Lang::Ko => PageText {
            title: "자동 조사・마케팅 — e-gov.info",
            h1: "디지털 정부의 자동 온라인 마케팅・자동 조사 기능",
            body: "디지털 정부 관련 주제를 일본어・영어로 정기적으로 자동 조사하고(GitHub는 실제 API, Google은 링크 생성) 각 기능의 홍보 문안도 정기 생성합니다. 기본값은 조사 24시간마다, 마케팅 12시간마다이며, --research-all/--marketing-all로 수동 실행도 가능합니다.",
        },
        Lang::It => PageText {
            title: "Ricerca e marketing automatizzati — e-gov.info",
            h1: "Marketing online automatizzato e ricerca bilingue",
            body: "Ricerca periodicamente argomenti sul governo digitale in giapponese e inglese (chiamate reali all'API GitHub Search; i link di ricerca Google sono generati, non estratti) e redige bozze di marketing per ogni funzione. Viene eseguito automaticamente su pianificazione (di default: ricerca ogni 24h, marketing ogni 12h) e può essere avviato manualmente con --research-all / --marketing-all.",
        },
        Lang::Fr => PageText {
            title: "Recherche et marketing automatisés — e-gov.info",
            h1: "Marketing en ligne automatisé et recherche bilingue",
            body: "Effectue périodiquement des recherches sur le gouvernement numérique en japonais et en anglais (appels réels à l'API GitHub Search ; les liens Google sont générés, pas extraits) et rédige des brouillons marketing pour chaque fonctionnalité. S'exécute automatiquement selon un calendrier (par défaut : recherche toutes les 24h, marketing toutes les 12h) et peut être déclenché manuellement avec --research-all / --marketing-all.",
        },
        Lang::De => PageText {
            title: "Automatisierte Recherche & Marketing — e-gov.info",
            h1: "Automatisiertes Online-Marketing & zweisprachige Recherche",
            body: "Recherchiert regelmäßig Themen zur Digitalregierung auf Japanisch und Englisch (echte GitHub-Search-API-Aufrufe; Google-Links werden generiert, nicht gescrapt) und entwirft Marketingtexte für jede Funktion. Läuft automatisch nach Zeitplan (Standard: Recherche alle 24h, Marketing alle 12h) und kann manuell mit --research-all / --marketing-all ausgelöst werden.",
        },
        Lang::Ar => PageText {
            title: "البحث والتسويق الآليان — e-gov.info",
            h1: "التسويق الآلي عبر الإنترنت والبحث الثنائي اللغة",
            body: "يجري بحثاً دورياً حول مواضيع الحكومة الرقمية باللغتين اليابانية والإنجليزية (استدعاءات فعلية لواجهة GitHub Search؛ روابط Google تُولَّد ولا تُستخرج بالزحف) ويُعِدّ مسودات تسويقية لكل ميزة. يعمل تلقائياً وفق جدول (افتراضياً: بحث كل 24 ساعة، تسويق كل 12 ساعة) ويمكن تشغيله يدوياً عبر --research-all / --marketing-all.",
        },
        Lang::Fa => PageText {
            title: "پژوهش و بازاریابی خودکار — e-gov.info",
            h1: "بازاریابی آنلاین خودکار و پژوهش دوزبانه",
            body: "به‌طور دوره‌ای موضوعات دولت دیجیتال را به ژاپنی و انگلیسی بررسی می‌کند (فراخوانی واقعی API جست‌وجوی گیت‌هاب؛ لینک‌های گوگل تولید می‌شوند، نه خزیده) و پیش‌نویس بازاریابی هر ویژگی را تهیه می‌کند. به‌طور خودکار طبق زمان‌بندی اجرا می‌شود (پیش‌فرض: پژوهش هر ۲۴ ساعت، بازاریابی هر ۱۲ ساعت) و با --research-all / --marketing-all قابل اجرای دستی است.",
        },
        Lang::Ru => PageText {
            title: "Автоматизированные исследования и маркетинг — e-gov.info",
            h1: "Автоматизированный онлайн-маркетинг и двуязычные исследования",
            body: "Периодически исследует темы цифрового государства на японском и английском (реальные вызовы GitHub Search API; ссылки Google генерируются, а не собираются скрапингом) и готовит маркетинговые черновики для каждой функции. Запускается автоматически по расписанию (по умолчанию: исследование каждые 24ч, маркетинг каждые 12ч), возможен ручной запуск через --research-all / --marketing-all.",
        },
        Lang::Uk => PageText {
            title: "Автоматизовані дослідження та маркетинг — e-gov.info",
            h1: "Автоматизований онлайн-маркетинг і двомовні дослідження",
            body: "Періодично досліджує теми цифрового уряду японською та англійською (реальні виклики GitHub Search API; посилання Google генеруються, а не збираються скрапінгом) і готує маркетингові чернетки для кожної функції. Запускається автоматично за розкладом (за замовчуванням: дослідження кожні 24 год, маркетинг кожні 12 год), можливий ручний запуск через --research-all / --marketing-all.",
        },
    }
}
