use anyhow::Result;
use serde_json::json;

pub async fn send_telegram_notification(message: &str, token: &str, chat_id: &str) -> Result<()> {
    let payload = json!({
        "text": message,
        "chat_id": chat_id
    });

    let client = reqwest::Client::new();
    client
        .post(format!(
            "https://api.telegram.org/bot{token}/sendMessage",
            token = token
        ))
        .json(&payload)
        .send()
        .await?;

    Ok(())
}
