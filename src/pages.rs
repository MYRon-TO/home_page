//! 这里是页面的处理函数
pub mod list;
use axum::http::StatusCode;

// todo: 404 page
pub async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}
