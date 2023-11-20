use askama::Template;
use axum::debug_handler;
use axum::extract::{Path, State};
use crate::db::data::SeriesDb;
use crate::run::AppState;

use crate::io::series_js::get_series_json;

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
          items: List{ items: vec![] },
          list_type
        }
    }

}

pub struct List{
    pub items: Vec<ListItem>,
}

impl List{
    /// append a item to list
    pub async fn append(&mut self, item: ListItem) {
        self.items.push(item);
    }

    pub async fn deal_db_date(date: SeriesDb) -> ListItem {
      // todo: 读取info.json
      let path = date.get_info_path();
      let series = get_series_json(path.as_str()).await.expect("get series json failed");
      // return an error page here if failed

      let title = series.get_title();
      let desc = series.get_desc();
      let cover = if series.get_has_cover() {
          format!("background-image: url({})", "assets/series/计算机网络/cover.png")
      } else {
          "".to_string()
      };
      let link = "todo: put a link here".to_string();

      ListItem::new(title, cover, link, desc)

    }
}


#[derive(Clone)]
pub struct ListItem {
    pub title: String,
    pub cover: String,
    pub link: String,
    pub desc: String,
}

impl ListItem {
    pub fn new(title: String, cover: String, link: String, desc: String) -> Self {
        ListItem {
            title,
            cover,
            link,
            desc,
        }
    }
}

/// ## handler for list page
/// which_type: what kind of list
/// 你应该传入[all, series, tags]
/// #### bug: 穷尽时返回all
/// todo: 想办法让他返回fallback()
/// 试试返回状态码
#[debug_handler]
pub async fn list(Path(list_type): Path<String>, State(app_state): State<AppState>) -> ListTemplate {
    match list_type.as_str() {
        "series" => list_series( app_state ).await,
        "tags"   => list_tags  ( app_state ).await,
        "all"    => list_all   ( app_state ).await,
        _        => list_nil   ().await,
    }
}

async fn list_all(app_state: AppState) -> ListTemplate {
  let rows = app_state.database.lock().await.get_all_series().await;
  let mut list = ListTemplate::new(ListType::All);
  for row in rows {
    let item = List::deal_db_date(row).await;
    list.items.append(item).await;
  }

  list
}

async fn list_nil() -> ListTemplate {
  ListTemplate{
    items: List{
      items :vec![]
    },
    list_type: ListType::Nil,
  }
}

async fn list_series(app_state: AppState) -> ListTemplate {
  let rows = app_state.database.lock().await.get_all_series().await;
  let mut list = ListTemplate::new(ListType::Series);
  for row in rows {
    let item = List::deal_db_date(row).await;
    list.items.append(item).await;
  }

  list
}

async fn list_tags(app_state: AppState) -> ListTemplate {
  let rows = app_state.database.lock().await.get_all_series().await;
  let mut list = ListTemplate::new(ListType::Tags);
  for row in rows {
    let item = List::deal_db_date(row).await;
    list.items.append(item).await;
  }

  list
}
