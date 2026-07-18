# PORTING.md — e-gov.info を他プロジェクトへお引越しする際のガイド

このファイル1枚で、本リポジトリの実装パターンを他プロジェクトへ移設できる
ことを目指す(`aruaru-tokyo`/`karu.tokyo`と同じ慣行)。

## 1. GitHub README/CLAUDE.md/PORTING.md 自動クロール・GitHub風表示

`src/github_viewer.rs` に、指定した1つのGitHubリポジトリの
`README.md`/`CLAUDE.md`/`PORTING.md`を`raw.githubusercontent.com`経由で
取得し、`pulldown-cmark`でGitHub風にレンダリングして表示する機能一式を
まとめている。`aruaru-tokyo`の複数リポジトリ切替版(`fetch_repo_file`/
`markdown_to_github_style_html`/`markdown_to_rs`)を単一リポジトリ固定
(`GITHUB_ORG`/`GITHUB_REPO`定数)に簡略化したもの。

移植手順:
1. `src/github_viewer.rs`をそのままコピーする。
2. `GITHUB_ORG`/`GITHUB_REPO`定数を移設先の値に書き換える。
3. `Cargo.toml`に`reqwest`(json, rustls-tls)・`pulldown-cmark`・
   `serde_json`を追加する。
4. 呼び出し側(`main.rs`)で`github_viewer::render_repo_viewer().await`を
   呼び、返ってきたHTML文字列をページに埋め込む。CSSも
   `github_viewer::VIEW_TOGGLE_CSS`定数をコピーする。

## 2. 定期自動調査(日本語・英語、Google/GitHub)

`src/research.rs`。GitHub Search APIは実際に叩いて結果を取得し、Googleは
検索リンクの自動生成に留める(APIキー不要な範囲で自動化する設計判断)。
`TOPICS`定数の日英ペアを書き換えるだけで、他プロジェクトのテーマにも
転用できる。

## 3. 定期自動マーケティングドラフト生成

`src/marketing.rs`。テンプレートベースの告知文生成(外部LLM APIは不使用)。
`CAMPAIGNS`定数を書き換えるだけで転用可能。

## 4. サーバー内蔵の定期実行ループ

`main.rs`の`spawn_periodic_tasks()`。`tokio::time::interval`で
起動時に自動でバックグラウンドループを開始する(VPSのcron/systemd timerに
頼らず、アプリ自身が「常に・定期的に・自動で」実行する設計)。環境変数で
間隔を変更可能。

## 5. LINE Messaging API Webhook(署名検証つき)

`src/line_webhook.rs`。`E_GOV_LINE_CHANNEL_SECRET`環境変数が未設定なら
501を返し「未接続」であることを正直に開示する設計。HMAC-SHA256による
署名検証ロジックはLINE公式SDKと同じ仕組みで実装。

## 注意事項

- 秘密情報(チャネルシークレット・アクセストークン等)は`.env`
  (`.gitignore`対象)またはVPSのsystemd `Environment=`経由で渡すこと。
  コードやコミット履歴に直接書き込まないこと。
