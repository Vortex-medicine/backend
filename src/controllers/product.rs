use crate::models::product::ProductId;
use crate::repositories::product::ProductRepository;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use serde_json::json;
use std::fmt::Debug;
use std::sync::Arc;

pub async fn get_all_products<R, P>(State(db): State<Arc<R>>) -> impl IntoResponse
where
    P: Debug + Serialize,
    R: ProductRepository<Product = P>,
{
    match db.get_all_products().await {
        Ok(products) => {
            (StatusCode::OK, Json(json!({ "products": dbg!(products) }))).into_response()
        }
        Err(error) => {
            dbg!(error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "error when getting all products"
                })),
            )
                .into_response()
        }
    }
}

pub async fn get_product_by_id<R, P>(
    State(db): State<Arc<R>>,
    Path(product_id): Path<ProductId>,
) -> impl IntoResponse
where
    P: Debug + Serialize,
    R: ProductRepository<Product = P>,
{
    match db.get_product_by_id(&product_id).await {
        Ok(product) => (StatusCode::OK, Json(product)).into_response(),
        Err(error) => {
            dbg!(error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "error when getting product by id"
                })),
            )
                .into_response()
        }
    }
}
