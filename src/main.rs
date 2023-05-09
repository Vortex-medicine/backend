use axum::http::{HeaderValue, Method};
use axum::{http, routing::post, Router};
use tower_http::cors::CorsLayer;

use crate::controllers::order::create_order;

mod constants;
mod controllers;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    let app = Router::new().route("/orders", post(create_order)).layer(
        CorsLayer::new()
            .allow_origin("http://localhost:3001".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::POST])
            .allow_headers([http::header::CONTENT_TYPE]),
    );

    axum::Server::bind(&"0.0.0.0:3008".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
