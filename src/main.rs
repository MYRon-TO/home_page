use axum::{
    // http::StatusCode,
    routing::get,
    Router,
};
use backend::run::AppState;
use std::net::SocketAddr; // 用于设置监听地址

use tower_http::{services::ServeDir, trace::TraceLayer};

use backend;
use backend::handler;

#[tokio::main]
async fn main() {

    let app_state = AppState::init().await;

    // 设置路由
    // 路由目录：root：/doc/
    let app = Router::new()
        // 设置静态文件目录
        .nest_service("/", ServeDir::new("./doc"))
        .route("/list/:type", get(handler::list::list))
        .with_state(app_state)// 设置共享状态
        .layer(TraceLayer::new_for_http()) // 打印请求日志? 不知道是干嘛的
        .fallback(handler::fallback);


    // 设置监听地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}
