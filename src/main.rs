use axum::{routing::get, Router};
use std::env;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Servidor escuchando en http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
