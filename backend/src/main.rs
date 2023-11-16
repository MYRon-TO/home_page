use axum::{
    // http::StatusCode,
    routing::get,
    Router,
};

use backend::db;

use std::net::SocketAddr;

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    // 设置路由
    let app = Router::new()
        .route("/", get(hello_world));  // localhost:3000/

    let db = db::Database::new().await;
    let answer = db.get_all_series().await;
    println!("{:?}", answer);


    // 设置监听地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
