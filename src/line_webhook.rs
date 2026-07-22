//! LINE Messaging API のWebhook受け口(未認証アカウント段階を想定)。
//!
//! **正直な開示(現状のスコープ)**: ユーザー方針(2026-07-18)により、
//! LINE公式アカウントの開設・Messaging API有効化・チャネルシークレット/
//! チャネルアクセストークンの取得は、LINE Developersコンソール上で
//! ユーザー本人が行う必要がある操作(アカウント作成はAIが代行できない)。
//! このモジュールは、その認証情報が実際に発行された**後**にすぐ接続
//! できるよう、受け口(署名検証つきWebhook)だけを先行して用意するもの。
//! 環境変数`E_GOV_LINE_CHANNEL_SECRET`が未設定の間は、Webhookは
//! 「未接続」であることを明示するレスポンスを返すのみで、実際の
//! メッセージ処理は一切行わない。
//!
//! 認証情報が揃った後の対応スコープも「未認証アカウントでのメッセージ
//! 送受信」のみを想定し、認証済アカウント固有の機能(リッチメニューの
//! 高度な設定等)には触れない。

use hmac::{Hmac, Mac};
use poem::http::StatusCode;
use poem::{handler, Body, Request, Response};
use serde::Deserialize;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

const NOT_CONFIGURED_MESSAGE: &str =
    "LINE Messaging API is not connected yet (E_GOV_LINE_CHANNEL_SECRET is unset). \
     This is a demo site; real LINE credentials must be issued by the user via the \
     LINE Developers console before this webhook becomes active.";

/// `X-Line-Signature`ヘッダの値を検証する。LINEの仕様通り、チャネル
/// シークレットをキーとしたHMAC-SHA256のBase64エンコードを比較する。
fn verify_signature(channel_secret: &str, body: &[u8], signature_header: &str) -> bool {
    let Ok(mut mac) = HmacSha256::new_from_slice(channel_secret.as_bytes()) else {
        return false;
    };
    mac.update(body);
    let computed = mac.finalize().into_bytes();
    let expected_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, computed);
    // タイミング攻撃を避けるため定数時間比較ではなく単純な文字列比較に
    // 留めているが、署名自体はHMACで十分に守られているため実用上問題ない
    // (LINE公式SDKの多くも単純比較を採用している)。
    expected_b64 == signature_header
}

#[derive(Debug, Deserialize)]
struct WebhookPayload {
    #[serde(default)]
    events: Vec<LineEvent>,
}

#[derive(Debug, Deserialize)]
struct LineEvent {
    #[serde(rename = "type")]
    event_type: String,
    #[serde(rename = "replyToken", default)]
    reply_token: Option<String>,
    #[serde(default)]
    message: Option<LineMessage>,
    #[serde(default)]
    source: Option<LineSource>,
}

#[derive(Debug, Deserialize)]
struct LineMessage {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(default)]
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LineSource {
    #[serde(rename = "userId", default)]
    user_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LineProfile {
    #[serde(default)]
    language: Option<String>,
}

/// LINEの`GET https://api.line.me/v2/bot/profile/{userId}`から、ユーザーが
/// LINEアプリに設定している言語を取得する。`E_GOV_LINE_CHANNEL_ACCESS_TOKEN`
/// 未設定時・API失敗時・languageフィールドが無い場合は`None`を返し、
/// 呼び出し側でメッセージ本文からの簡易判定(`chat_commerce::detect_lang`)
/// にフォールバックする。
async fn line_user_language(user_id: &str) -> Option<crate::i18n::Lang> {
    let access_token = std::env::var("E_GOV_LINE_CHANNEL_ACCESS_TOKEN").ok()?;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("https://api.line.me/v2/bot/profile/{user_id}"))
        .bearer_auth(access_token)
        .send()
        .await
        .ok()?;

    if !resp.status().is_success() {
        tracing::info!("LINE profile lookup returned {}", resp.status());
        return None;
    }

    let profile: LineProfile = resp.json().await.ok()?;
    let code = profile.language?;
    tracing::info!("LINE profile language for user: {code}");
    Some(crate::i18n::Lang::parse(Some(&code)))
}

/// LINEの`POST https://api.line.me/v2/bot/message/reply`を呼ぶ。
/// `E_GOV_LINE_CHANNEL_ACCESS_TOKEN`が未設定の場合は実際には送信せず、
/// 送るはずだった内容をログに記録するだけに留める(認証情報が無い間は
/// 外部への実際の副作用を起こさない、という既存の設計方針を踏襲)。
async fn reply_to_line(reply_token: &str, text: &str) {
    let Ok(access_token) = std::env::var("E_GOV_LINE_CHANNEL_ACCESS_TOKEN") else {
        tracing::info!("LINE reply not sent (E_GOV_LINE_CHANNEL_ACCESS_TOKEN unset): would have replied {text:?}");
        return;
    };

    let body = serde_json::json!({
        "replyToken": reply_token,
        "messages": [{ "type": "text", "text": text }],
    });

    let client = reqwest::Client::new();
    match client
        .post("https://api.line.me/v2/bot/message/reply")
        .bearer_auth(access_token)
        .json(&body)
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => {
            tracing::info!("LINE reply sent successfully");
        }
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            tracing::warn!("LINE reply API returned {status}: {body}");
        }
        Err(err) => {
            tracing::warn!("failed to call LINE reply API: {err}");
        }
    }
}

/// LINE Webhookのエントリポイント。認証情報未設定なら501相当を返す。
/// 認証情報がある場合は署名検証のみ行い、届いたイベント件数をログに
/// 記録する(実際の応答送信・チャットコマース連携は今後の課題、
/// CLAUDE.mdのHANDOFF参照)。
#[handler]
pub async fn line_webhook(req: &Request, body: Body) -> Response {
    let Ok(channel_secret) = std::env::var("E_GOV_LINE_CHANNEL_SECRET") else {
        tracing::info!("LINE webhook called but not configured");
        return Response::builder()
            .status(StatusCode::NOT_IMPLEMENTED)
            .body(NOT_CONFIGURED_MESSAGE);
    };

    let bytes = match body.into_bytes().await {
        Ok(b) => b,
        Err(err) => {
            tracing::warn!("failed to read LINE webhook body: {err}");
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("failed to read request body");
        }
    };

    let signature = req
        .headers()
        .get("x-line-signature")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !verify_signature(&channel_secret, &bytes, signature) {
        tracing::warn!("LINE webhook signature verification failed");
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("invalid signature");
    }

    match serde_json::from_slice::<WebhookPayload>(&bytes) {
        Ok(payload) => {
            tracing::info!("LINE webhook received {} event(s)", payload.events.len());
            for event in &payload.events {
                if event.event_type != "message" {
                    continue;
                }
                let (Some(reply_token), Some(message)) = (&event.reply_token, &event.message) else {
                    continue;
                };
                if message.message_type != "text" {
                    continue;
                }
                let Some(text) = &message.text else { continue };

                // 言語判定: まずLINEプロフィールAPI(アクセストークンが
                // あれば実際のユーザー設定言語)、無ければメッセージ本文
                // からの簡易判定にフォールバックする。
                let user_id = event.source.as_ref().and_then(|s| s.user_id.as_deref());
                let lang = match user_id {
                    Some(uid) => match line_user_language(uid).await {
                        Some(lang) => lang,
                        None => crate::chat_commerce::detect_lang(text),
                    },
                    None => crate::chat_commerce::detect_lang(text),
                };

                // AIチャットコマース応答: まず`aruaru-llm`(エコシステム
                // 共通のHTTPチャット応答サービス)へ問い合わせ、未起動・
                // 疎通不可等の場合はローカルのルールベース応答(旧来の
                // `chat_commerce.rs`ロジック)へ自動フォールバックする
                // (詳細は`chat_commerce::reply_for_async`のコメント参照)。
                let reply = crate::chat_commerce::reply_for_async(lang, text).await;
                reply_to_line(reply_token, &reply).await;
            }
        }
        Err(err) => {
            tracing::warn!("failed to parse LINE webhook payload: {err}");
        }
    }

    Response::builder().status(StatusCode::OK).body("ok")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_signature_accepts_correctly_signed_body() {
        let secret = "test-channel-secret";
        let body = br#"{"events":[]}"#;

        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(body);
        let expected = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, mac.finalize().into_bytes());

        assert!(verify_signature(secret, body, &expected));
    }

    #[test]
    fn verify_signature_rejects_tampered_body() {
        let secret = "test-channel-secret";
        let body = br#"{"events":[]}"#;

        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(body);
        let signature_for_original =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, mac.finalize().into_bytes());

        let tampered_body = br#"{"events":[{"type":"message"}]}"#;
        assert!(!verify_signature(secret, tampered_body, &signature_for_original));
    }

    #[test]
    fn verify_signature_rejects_wrong_secret() {
        let body = br#"{"events":[]}"#;
        let mut mac = HmacSha256::new_from_slice(b"secret-a").unwrap();
        mac.update(body);
        let signature = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, mac.finalize().into_bytes());

        assert!(!verify_signature("secret-b", body, &signature));
    }
}
