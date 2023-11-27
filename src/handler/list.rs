// use crate::io::series_js::get_series_json;

mod list_handler;
mod list_item;

use super::*;
use list_item::*;

pub enum ListType {
    Series,
    Tags,
    Blog,
    Nil,
}

pub enum QuestBlog {
    All,
    Series(String),
    Tags(String),
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

    /// true -> empty
    pub fn is_empty(&self) -> bool {
      self.content.is_empty()
    }

    pub async fn deal_db_date(date: DbData) -> ListItem {
        match date {
            DbData::Series(series) => List::make_series_item(series).await,
            DbData::BlogSeries(blog_series) => List::make_blog_series_item(blog_series).await,
            DbData::TagBlog(tag_blog) => List::make_tag_item(tag_blog).await,
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
            format!("background-image: url({})", cover)
        } else {
            "".to_string()
        };
        // let link = "todo: put a link here".to_string();
        let link = format!("/list/series/{}", title);

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
            format!("background-image: url({})", cover)
        } else {
            "".to_string()
        };
        // let link = "todo: put a link here".to_string();
        let link = format!("/blog/{}", title);

        ListItem::new_series(title, cover, link, desc)
    }

    async fn make_tag_item(data: crate::db::data::tag_blog_db::TagBlogDb) -> ListItem {
        let title = data.get_name();
        let link = format!("/list/tags/{}", title);

        ListItem::new_tags(title, link)
    }

}

/// ## handler for list page
/// which_type: what kind of list
/// 你应该传入[blog, series, tags]
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
        "blog" => list_handler::list_blog(app_state, QuestBlog::All).await,
        _ => list_handler::list_nil().await,
    }
}

#[debug_handler]
pub async fn list_blog(
    Path((list_type, blog_name)): Path<(String, String)>,
    State(app_state): State<AppState>,
) -> ListTemplate {
    match list_type.as_str() {
        "series" => list_handler::list_blog(app_state, QuestBlog::Series(blog_name)).await,
        "tags" => list_handler::list_blog(app_state, QuestBlog::Tags(blog_name)).await,
        "blog" => list_handler::list_blog(app_state, QuestBlog::All).await,
        _ => list_handler::list_nil().await,
    }
}
