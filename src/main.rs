use axum::{routing::get, Router};

mod root;
use root::*;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root).post(root_post).delete(root_delete));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}