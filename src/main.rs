#![allow(unused)]

use std::net::SocketAddr;
use axum::response::{Html, IntoResponse};
use axum::{Router, ServiceExt};
use axum::extract::{Path, Query};
use axum::routing::get;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .merge(routes_hello());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> Listening on {addr} \n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello/:name", get(handler_hello2))
}
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

// eg: `hello?name=Nishat`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// eg: `hello/Nishat`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello <strong>{name}</strong>"))
}
