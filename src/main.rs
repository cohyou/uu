extern crate uu;
// use uu::*;
use axum::{response::Html, routing::get, Router};
// use http::{Request, Response};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    let resp = send();
    println!("{:#?}", resp.await.unwrap());
    Html("<h1>Hello, World!</h1>")
}

use std::error::Error;
async fn send() -> Result<String, Box<dyn Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .text()
        .await?;
    Ok(resp)
}