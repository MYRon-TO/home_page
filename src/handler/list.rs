use std::path::PathBuf;

use crate::db::data::{blog_series_db::BlogSeriesDb, series_db::SeriesDb, DbData};
use crate::io::{blog_js::get_blog_json, series_js::get_series_json};
use crate::run::AppState;
use askama::Template;
use axum::debug_handler;
use axum::extract::{Path, State};
use walkdir::{DirEntry, WalkDir};

// use crate::io::series_js::get_series_json;

mod list_handler;
mod list_item;

use list_item::*;
// use list_handler::*;

pub enum ListType {
    Series,
    Tags,
    All,
    Nil,
}

/// 页面list的模板
/// list是一个列表页面，
/// 依据不同的参数，返回不同的列表
#[derive(Template)]
#[template(path = "list.html")]
pub struct ListTemplate {
    items: List,
    list_type: ListType,
}

impl ListTemplate {
    pub fn new(list_type: ListType) -> Self {
        ListTemplate {
            items: List { content: vec![] },
            list_type,
        }
    }
}

pub struct List {
    pub content: Vec<ListItem>,
}

impl List {
    /// append a item to list
    pub async fn append(&mut self, item: ListItem) {
        self.content.push(item);
    }

    pub async fn deal_db_date(date: DbData) -> ListItem {
        match date {
            DbData::Series(series) => List::make_series_item(series).await,
            DbData::BlogSeries(blog_series) => List::make_blog_series_item(blog_series).await,
        }
    }

    /// ## get_series_item
    /// ### args
    /// **data**: data from database which type is SeriesDb
    async fn make_series_item(data: SeriesDb) -> ListItem {
        // 读取info.json
        let path = data.get_info_path();
        let series = get_series_json(path.as_str())
            .await
            .expect("get series json failed");
        // todo: return an error page here if failed

        let title = series.get_title();
        let desc = series.get_desc();
        let cover = if series.get_has_cover() {
            let cover = get_cover(path.clone()).unwrap_or("".to_string());
            format!(
                "background-image: url({})",
                cover
            )
        } else {
            "".to_string()
        };
        let link = "todo: put a link here".to_string();

        ListItem::new_series(title, cover, link, desc)
    }

    async fn make_blog_series_item(data: BlogSeriesDb) -> ListItem {
        let path = data.get_info_path();
        let blog: crate::io::blog_js::BlogJs = get_blog_json(path.as_str())
            .await
            .expect("get blog json failed");

        let title = blog.get_title();
        let desc = blog.get_desc();
        let cover = if blog.get_has_cover() {
            let cover = get_cover(path.clone()).unwrap_or("".to_string());
            format!(
                "background-image: url({})",
                cover
            )
        } else {
            "".to_string()
        };
        let link = "todo: put a link here".to_string();

        ListItem::new_series(title, cover, link, desc)
    }
}

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn get_cover(path: String)-> Option<String> {
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
    return None
}

/// ## handler for list page
/// which_type: what kind of list
/// 你应该传入[all, series, tags]
/// #### bug: 穷尽时返回all
/// todo: 想办法让他返回fallback()
/// 试试返回状态码
#[debug_handler]
pub async fn list(
    Path(list_type): Path<String>,
    State(app_state): State<AppState>,
) -> ListTemplate {
    match list_type.as_str() {
        "series" => list_handler::list_series(app_state).await,
        "tags" => list_handler::list_tags(app_state).await,
        "all" => list_handler::list_all(app_state).await,
        _ => list_handler::list_nil().await,
    }
}
