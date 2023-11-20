use askama::Template;
use axum::extract::{Path, State};
use crate::db::data::SeriesDb;
use crate::run::AppState;

use crate::io::series_js::get_series_json;

/// 页面list的模板
/// list是一个列表页面，
/// 依据不同的参数，返回不同的列表
#[derive(Template)]
#[template(path = "list.html")]
pub struct ListTemplate {
    items: Vec<ListItem>,
}

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

impl ListTemplate {
    pub fn new() -> Self {
        ListTemplate { items: vec![] }
    }

    /// append a item to list
    pub fn append(&mut self, item: ListItem) {
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

/// ## handler for list page
/// which_type: what kind of list
/// 你应该传入[all, series, tags]
/// #### bug: 穷尽时返回all
/// todo: 想办法让他返回fallback()
pub async fn list(Path(list_type): Path<String>, State(app_state): State<AppState>) -> ListTemplate {
    match list_type.as_str() {
        "series" => list_series( app_state ).await,
        "tags"   => list_tags  ( app_state ).await,
        _        => list_all   ( app_state ).await,
    }
}

async fn list_all(app_state: AppState) -> ListTemplate {
  let rows = app_state.database.lock().unwrap().get_all_series().await;
  let mut list = ListTemplate::new();
  for row in rows {
    let item = ListTemplate::deal_db_date(row).await;
    list.append(item);
  }

  list
}

async fn list_series(_app_state: AppState) -> ListTemplate {
    ListTemplate {
        items: vec![ListItem {
            title: "TiTlE".to_string(),
            link: "lInK".to_string(),
            cover: "cOvEr".to_string(),
            desc: "dEsCrIpTiOn".to_string(),
        }],
    }
}

async fn list_tags(_app_state: AppState) -> ListTemplate {
    ListTemplate {
        items: vec![ListItem {
            title: "TiTlE".to_string(),
            link: "lInK".to_string(),
            cover: "cOvEr".to_string(),
            desc: "dEsCrIpTiOn".to_string(),
        }],
    }
}
