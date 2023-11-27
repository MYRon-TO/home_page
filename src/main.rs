use axum::{
    // http::StatusCode,
    routing::get,
    Router,
};

use backend::run::AppState;
use tower::limit::ConcurrencyLimitLayer;
use std::net::SocketAddr;

use tower_http::services::ServeDir;

use backend;
use backend::handler;

#[tokio::main]
async fn main() {

    let app_state = AppState::init().await;

    let route_list = Router::new()
        // 好像应该穷举所有的路由来着,
        // 这样可以用fallback()来处理404,
        // 懒得改了
        // todo()!
        // .route("/blog", get(handler::list::list))
        // .route("/series", get(handler::list::list))
        .route("/:type", get(handler::list::list))
        .route("/:type/:name", get(handler::list::list_blog));

    // 设置路由
    // 路由目录：root：/doc/
    let app = Router::new()
        // 设置静态文件目录
        .nest_service("/", ServeDir::new("./doc"))
        // 设置路由
        .nest("/list", route_list)
        .route("/blog/:name", get(handler::blog::blog))
        // 设置共享状态
        .with_state(app_state)
        // 设置并发数
        .layer(ConcurrencyLimitLayer::new(8))
        .fallback(handler::fallback);


    // 设置监听地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}
