use std::net::SocketAddr;// 用于设置监听地址
use axum::{
    // http::StatusCode,
    routing::get,
    Router,
};

use backend;

#[tokio::main]
async fn main() {
    backend::run::init().await;

    // 设置路由
    let app = Router::new()
        .route("/", get(backend::pages::hello_world))
        .route("/list", get(backend::list));

    // 设置监听地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
