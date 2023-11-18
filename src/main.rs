use axum::{
    // http::StatusCode,
    routing::get,
    Router,
};

use backend;



use std::{net::SocketAddr, thread::sleep};

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn hi_world() -> &'static str {
    "hi"
}

#[tokio::main]
async fn main() {
    backend::run::init().await;

    // 设置路由
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/hi/", get(hi_world)); // localhost:3000/

    // 设置监听地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
