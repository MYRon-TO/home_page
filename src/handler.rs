//! 这里是页面的处理函数
pub mod blog;
pub mod list;

use crate::db::data::{blog_series_db::BlogSeriesDb, series_db::SeriesDb, DbData};
use crate::io::{blog_js::get_blog_json, series_js::get_series_json};
use crate::run::AppState;

use askama::Template;
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
};

use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn get_cover(path: String) -> Option<String> {
    let path = PathBuf::from(path.as_str());
    let path_dir = path.parent()?;
    let walker = WalkDir::new(path_dir).max_depth(1).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        let name = entry.path().file_stem()?.to_str()?;
        if name == "cover" {
            return Some(format!("/{}", entry.path().display().to_string()));
        }
    }
    return None;
}

// todo: 404 page
pub async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

