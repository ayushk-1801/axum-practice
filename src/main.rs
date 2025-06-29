use axum::{
    Router,
    routing::{get, post},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/", post(root_post));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() {
    println!("[GET] /");
}

async fn root_post() {
    println!("[POST] /");
}
