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

use poem::listener::TcpListener;
use poem::web::Html;
use poem::{get, handler, Route, Server};

const GITHUB_REPO_URL: &str = "https://github.com/aon-co-jp/e-gov";

fn page_shell(title: &str, body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="ja">
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
</style>
</head>
<body>
<nav><a href="/">TOP</a> <a href="/gov">eガバメント</a> <a href="/trade">オンライン貿易</a> <a href="/credit">与信・売掛保証</a> <a href="/realestate">不動産・AI工務店</a></nav>
{body}
<footer><p>e-gov.info — デジタルガバメント × オンライン貿易プラットフォーム(構想段階)。 <a href="{GITHUB_REPO_URL}">GitHub (aon-co-jp/e-gov)</a></p></footer>
</body>
</html>"#
    )
}

#[handler]
fn index() -> Html<String> {
    let body = r#"
<h1>e-gov.info <span class="badge">構想段階 v0.1.0</span></h1>
<p>行政のデジタル化と、個人〜貿易商社まで対応するオンライン貿易・不動産
プラットフォームを、LINEアプリ・WEBサイト・コンビニ端末という複数の入り口
から利用できる形で統合する構想です。</p>
<h2>2本柱</h2>
<ul class="linklist">
<li><a href="/gov">eガバメント(デジタルガバメント)</a> — ペーパーレス・
オンライン申請。コンビニ端末(Loppi/Famiポート等)・LINE・WEBの複数入口、
金額・重要度に応じた段階的本人確認。</li>
<li><a href="/trade">オンライン貿易プラットフォーム</a> — AIチャット
コマースによる総合通販。個人〜貿易商社まで対応、既存大手より低い手数料。</li>
<li><a href="/credit">AI与信調査・掛け仕入れ・売掛保証</a> — 与信スコアに
応じた後払い仕入れ、電子請求書の重複調査、売掛債権の保証。</li>
<li><a href="/realestate">不動産投資・電子契約・AI工務店</a> — 土地情報
からAIが間取りを提案。投機的資金の過剰流入を助長しない設計方針。</li>
</ul>
<p>詳細な設計思想は <code>CLAUDE.md</code> を参照してください。</p>
"#;
    Html(page_shell("e-gov.info — デジタルガバメント×オンライン貿易", body))
}

#[handler]
fn gov() -> Html<String> {
    let body = r#"
<h1>eガバメント(デジタルガバメント)</h1>
<h2>入り口の多様化</h2>
<ul class="linklist">
<li>LINEアプリ</li>
<li>WEBサイト</li>
<li>コンビニのマルチメディア端末(Loppi/Famiポート/セブン-イレブンの
マルチコピー機等) — WEBサイトやスマホの操作が分からない方向け</li>
</ul>
<h2>段階的本人確認(金額・重要度に応じて認証強度を変える)</h2>
<ul class="linklist">
<li><strong>軽微な照会・低額手続き</strong>: 電話のナンバーディスプレイに
よる簡易確認(ナンバーディスプレイが無い電話には折り返し電話で確認)</li>
<li><strong>中程度の金額・手続き</strong>: 登録メールへのワンタイム
パスワード(OTP)</li>
<li><strong>高額な取引・重要な法的手続き</strong>: マイナンバーカードの
スマートフォンNFCスキャンによる公的個人認証</li>
</ul>
<p>ペーパーレス・オンライン申請を基本方針とし、複数の省庁・自治体に
またがる手続きの入力を1回で済ませるワンストップ化を目指します。</p>
"#;
    Html(page_shell("eガバメント — e-gov.info", body))
}

#[handler]
fn trade() -> Html<String> {
    let body = r#"
<h1>オンライン貿易プラットフォーム(AIチャットコマース)</h1>
<p>LINEアプリ・WEBサイトの両方から、AIとの対話で商品を発見・注文できる
総合通販です。</p>
<h2>取扱商材(予定)</h2>
<p>食料品・加工食品・非常食・日用雑貨・衣料品・医療品・サプリメント・
家具・家電製品・スポーツ用品・新車/中古車(自動車・バイク)・
オーディオ機器(アンプ・スピーカー)</p>
<h2>出店者層</h2>
<p>個人(フリマ的な出品)〜貿易商社まで対応するマーケットプレイス型。
既存の大手プラットフォームより出店・出品手数料を大幅に引き下げます。</p>
<h2>初期フェーズの方針</h2>
<p>実際の在庫仕入れは行わず、アドセンス(広告収益)によるサンプル的な
運用から開始し、AIチャットコマースのUXを検証します。</p>
"#;
    Html(page_shell("オンライン貿易プラットフォーム — e-gov.info", body))
}

#[handler]
fn credit() -> Html<String> {
    let body = r#"
<h1>AI与信調査・掛け仕入れ・電子請求書重複調査・売掛保証</h1>
<h2>AI与信調査・掛け仕入れ</h2>
<p>出店者・仕入れ希望者の取引実績等をAIが分析し、与信スコア(パーセン
テージ)を算出します。スコアに応じて代金の後払い(例: 3ヶ月後払いなどの
支払いサイト)による掛け仕入れを許可し、与信担保・保険加入(取引信用
保険相当)と組み合わせてリスクを補完します。</p>
<h2>電子請求書の重複調査</h2>
<p>発行しようとする電子請求書が、インボイス制度の登録番号や請求内容の
面で他社の請求書と重複・衝突していないかをAIがオンラインで調査し、
二重請求・不正請求のリスクを事前に検知します。</p>
<h2>売掛保証</h2>
<p>出店者が発行した売掛債権(未回収の請求)を保証する仕組みを搭載し、
代金未回収リスクを過度に負わずに掛け取引できるようにします。</p>
<p><em>与信・保険・請求書関連機能は貸金業法・割賦販売法・保険業法等の
規制対象となるため、実装前に法令・実在APIの調査が必要です(詳細は
CLAUDE.md参照、現時点では設計方針のみで未実装)。</em></p>
"#;
    Html(page_shell("与信・売掛保証 — e-gov.info", body))
}

#[handler]
fn realestate() -> Html<String> {
    let body = r#"
<h1>不動産投資・電子契約・AI工務店</h1>
<p>土地・不動産の売買・賃貸契約を電子契約で完結します。</p>
<h2>AI工務店</h2>
<p>ユーザーが検索した土地情報をもとに、AIが「この様な間取りはいかが
でしょうか」と提案するAIチャットコマース形式の対話型間取り提案を
予定しています。</p>
<h2>税制に関する設計方針</h2>
<p>海外の不動産投資優遇税制の事例では、節税目的の投資マネーが不動産に
過剰流入し、地価・不動産価格が高騰して実需層の生活を圧迫する副作用が
指摘されています。本プラットフォームは、手数料体系・レコメンド
ロジック側で投機的資金の過剰流入を助長しないことを設計原則とします。</p>
"#;
    Html(page_shell("不動産・AI工務店 — e-gov.info", body))
}

#[handler]
fn healthz() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let app = Route::new()
        .at("/", get(index))
        .at("/gov", get(gov))
        .at("/trade", get(trade))
        .at("/credit", get(credit))
        .at("/realestate", get(realestate))
        .at("/healthz", get(healthz));

    let bind_addr = "127.0.0.1:4500";
    tracing::info!("e-gov-server listening on {bind_addr}");
    Server::new(TcpListener::bind(bind_addr)).run(app).await
}
