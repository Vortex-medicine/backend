use axum::http::{HeaderValue, Method};
use axum::routing::get;
use axum::{http, routing::post, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::controllers::order::create_order;
use crate::controllers::product::{get_all_products, get_product_by_id};
use crate::repositories::mongo::MongoRepository;

mod constants;
mod controllers;
mod models;
mod repositories;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "vortex-medicine=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = Arc::new(MongoRepository::init().await);

    let app = Router::new()
        .route("/orders", post(create_order))
        .route("/products", get(get_all_products))
        .route("/products/:product_id", get(get_product_by_id))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3001".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::POST])
                .allow_headers([http::header::CONTENT_TYPE]),
        )
        .with_state(db);

    axum::Server::bind(&"0.0.0.0:3008".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
