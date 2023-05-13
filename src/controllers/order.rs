use crate::constants::telegram_notifier::{TELEGRAM_BOT_TOKEN, TELEGRAM_CHAT_ID};
use crate::models::order::Order;
use crate::repositories::mongo::MongoRepository;
use crate::repositories::order::OrderRepository;
use crate::utils::telegram_notifier::send_telegram_notification;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::sync::Arc;

pub async fn create_order(
    State(db): State<Arc<MongoRepository>>,
    Json(order): Json<Order>,
) -> impl IntoResponse {
    println!("{:#?}", order);

    let created_order = match db.create_order(order.clone()).await {
        Ok(value) => value,
        Err(error) => {
            dbg!(error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "error when creating order"
                })),
            )
                .into_response();
        }
    };

    dbg!(&created_order);

    let order_json_str = match serde_json::to_string_pretty(&created_order) {
        Ok(value) => format!("{}{}", "New order: ", value),
        Err(error) => {
            dbg!(error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "error when converting order to json string"
                })),
            )
                .into_response();
        }
    };

    let response_order = Order {
        id: created_order.id,
        creation_time: created_order.creation_time,
        ..order
    };

    let token = dotenv::var(TELEGRAM_BOT_TOKEN).unwrap();
    let chat_id = dotenv::var(TELEGRAM_CHAT_ID).unwrap();

    match send_telegram_notification(&order_json_str, &token, &chat_id).await {
        Ok(_) => (StatusCode::CREATED, Json(response_order)).into_response(),
        Err(error) => {
            dbg!(error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "error when sending telegram notification"
                })),
            )
                .into_response()
        }
    }
}
