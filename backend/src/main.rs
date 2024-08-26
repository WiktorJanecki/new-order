use axum::{routing::get, Router};

mod controllers;
mod error;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ping", get(|| async { "pong" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
