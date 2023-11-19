use std::net::SocketAddr;// 用于设置监听地址
use axum::{
    // http::StatusCode,
    routing::get,
    Router,
};

use tower_http::{trace::TraceLayer, services::ServeDir};

use backend;

#[tokio::main]
async fn main() {
    backend::run::init().await;

    // 设置路由
    let app = Router::new()
        // 设置静态文件目录
        .nest_service("/", ServeDir::new("./doc"))
        .route("/list", get(backend::list))
        .layer(TraceLayer::new_for_http());

    // 设置监听地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));


    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    tokio::join!(

    )
}
