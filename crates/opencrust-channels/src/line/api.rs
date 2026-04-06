use reqwest::Client;

pub const LINE_API_BASE: &str = "https://api.line.me/v2/bot";
/// Separate hostname used for downloading message content (images, files, etc.).
pub const LINE_DATA_API_BASE: &str = "https://api-data.line.me/v2/bot";

/// Download the binary content of a LINE message (image, file, audio, video).
///
/// Uses the data API: `GET {data_base_url}/message/{message_id}/content`.
/// Returns the raw bytes.
pub async fn download_content(
    client: &Client,
    channel_access_token: &str,
    message_id: &str,
    data_base_url: &str,
) -> Result<Vec<u8>, String> {
    let resp = client
        .get(format!("{data_base_url}/message/{message_id}/content"))
        .bearer_auth(channel_access_token)
        .send()
        .await
        .map_err(|e| format!("line download_content request failed: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("line download_content error {status}: {body}"));
    }

    resp.bytes()
        .await
        .map(|b| b.to_vec())
        .map_err(|e| format!("line download_content read failed: {e}"))
}

/// Send a reply using a reply token (free, expires in 30 seconds, one use).
pub async fn reply(
    client: &Client,
    channel_access_token: &str,
    reply_token: &str,
    text: &str,
    base_url: &str,
) -> Result<(), String> {
    let body = serde_json::json!({
        "replyToken": reply_token,
        "messages": [{"type": "text", "text": text}]
    });

    let resp = client
        .post(format!("{base_url}/message/reply"))
        .bearer_auth(channel_access_token)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("line reply request failed: {e}"))?;

    if resp.status().is_success() {
        Ok(())
    } else {
        let status = resp.status();
        let body_text = resp.text().await.unwrap_or_default();
        Err(format!("line reply failed ({status}): {body_text}"))
    }
}

/// Send a push message to a user ID (paid tier, works at any time).
pub async fn push(
    client: &Client,
    channel_access_token: &str,
    user_id: &str,
    text: &str,
    base_url: &str,
) -> Result<(), String> {
    let body = serde_json::json!({
        "to": user_id,
        "messages": [{"type": "text", "text": text}]
    });

    let resp = client
        .post(format!("{base_url}/message/push"))
        .bearer_auth(channel_access_token)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("line push request failed: {e}"))?;

    if resp.status().is_success() {
        Ok(())
    } else {
        let status = resp.status();
        let body_text = resp.text().await.unwrap_or_default();
        Err(format!("line push failed ({status}): {body_text}"))
    }
}
