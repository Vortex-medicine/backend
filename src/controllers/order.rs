use crate::constants::telegram_notifier::{TELEGRAM_BOT_TOKEN, TELEGRAM_CHAT_ID};
use crate::models::order::Order;
use crate::utils::telegram_notifier::send_telegram_notification;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

pub async fn create_order(Json(order): Json<Order>) -> impl IntoResponse {
    println!("{:#?}", order);

    let order_json_str = match serde_json::to_string_pretty(&order) {
        Ok(value) => format!("{}{}", "New order: ", value),
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message":
                        format!(
                            "{}{}",
                            "error when converting order to json string: ", error
                        )
                })),
            )
                .into_response()
        }
    };

    let token = dotenv::var(TELEGRAM_BOT_TOKEN).unwrap();
    let chat_id = dotenv::var(TELEGRAM_CHAT_ID).unwrap();

    match send_telegram_notification(&order_json_str, &token, &chat_id).await {
        Ok(_) => (StatusCode::CREATED, Json(order)).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": format!("error when sending telegram notification: {}", error)
            })),
        )
            .into_response(),
    }
}
