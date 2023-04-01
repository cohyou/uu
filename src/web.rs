use axum::{routing::get, Router};
use std::net::SocketAddr;
use axum::response::Json;
use serde_json::{Value, json};
use tower_http::cors::{CorsLayer, Any};
use http::Method;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET])
        .allow_origin(Any)
        .allow_headers(Any);
    let app = Router::new()
        .route("/", get(root))
        .route("/json", get(json))
        .layer(cors);
    let addr = SocketAddr::from(([172, 31, 40, 87], 8127));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, UU!\n"
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42, "a": "bbb" }))
}
