extern crate uu;
// use uu::*;
use axum::{response::Html, routing::get, Json, Router};
// use http::{Request, Response};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/j", get(json_handler))
        .route("/ii", get(ii))
        .route("/list", get(json_handler2));

    // run it
    // let domain = "0.0.0.0:3000";
    let domain = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(domain)
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<String> {    
    Html(format!("<h1>Hello, World!</h1><body>res: {:#?}</body>", "sss"))
}

async fn ii() -> Html<&'static str> {    
    Html(include_str!("./index.html"))
}

async fn json_handler() -> Json<serde_json::Value> {
    let db_id = "48a7e844b86c47a7b87fab295956d4b3";
    let url = format!("https://api.notion.com/v1/databases/{}", db_id);

    let resp = send(url);
    // println!("{:#?}", resp.await.unwrap());
    let sss = serde_json::from_str::<serde_json::Value>(resp.await.unwrap().as_str());
    Json(sss.unwrap())
}

async fn json_handler2() -> Json<serde_json::Value> {
    let db_id = "48a7e844b86c47a7b87fab295956d4b3";
    let url = format!("https://api.notion.com/v1/databases/{}/query", db_id);

    let resp = send_post(url);
    // println!("{:#?}", resp.await.unwrap());
    let sss = serde_json::from_str::<serde_json::Value>(resp.await.unwrap().as_str());
    let results = &sss.unwrap()["results"];
    Json(results.clone())
}

use std::error::Error;
async fn send_post(url: String) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let resp = client.post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json; charset=UTF-8")
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", "secret"))
        .header("Notion-Version", "2022-02-22")
        .send()
        .await?
        .text()
        .await?;
    
    Ok(resp)
}

async fn send(url: String) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let resp = client.get(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json; charset=UTF-8")
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", "secret"))
        .header("Notion-Version", "2022-02-22")
        .send()
        .await?
        .text()
        .await?;
    
    Ok(resp)
}