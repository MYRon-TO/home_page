use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;

use super::*;

// todo!();

#[derive(Template)]
#[template(path = "blog.html")]
pub struct BlogTemplate {
    pub item: Blog,
}

pub struct Blog {
    name: String,
    tags: Vec<String>,
    series: String,
    cover: String,
    update_time: DateTime<Utc>,
}

impl Blog {
    pub fn get_cover(&self) -> String {
        // format!(
        //   "<img src=\"{}\"/>",
        //   self.cover
        // )
        if self.cover == "" {
            return "/assets/images/default_cover.png".to_string();
        }
        self.cover.to_string()
    }

    pub fn get_background(&self) -> String {
        if self.cover == "" {
            return "background: url(/assets/images/default_cover.png) no-repeat center center fixed;"
                .to_string();
        }
        format!(
            "background: url({}) no-repeat center center fixed;",
            self.cover
        )
    }

    pub fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_series(&self) -> String {
        self.series.clone()
    }

    pub fn get_update_time(&self) -> String {
        self.update_time.format("%Y/%m/%d").to_string()
    }

    pub async fn deal_with_db(db: DbData) -> Blog {
        if let DbData::BlogSeries(blog_series) = db {
            let name = blog_series.get_name();
            let info_path = blog_series.get_info_path();
            let blog_info = get_blog_json(&info_path)
                .await
                .expect("get blog json error");

            let tags = blog_info.get_tag().to_vec();
            let cover = get_cover(info_path).unwrap_or("".to_string());
            let series = blog_series.get_series();
            let update_time = blog_series.get_update_time();

            Blog {
                name,
                tags,
                cover,
                series,
                update_time,
            }
        } else {
            Blog {
                name: "".to_string(),
                tags: vec!["".to_string()],
                cover: "".to_string(),
                series: "".to_string(),
                update_time: Utc::now(),
            }
        }
    }
}

#[debug_handler]
pub async fn blog(
    Path(blog_name): Path<String>,
    State(app_state): State<AppState>,
) -> BlogTemplate {
    let blog_data = if let Some(row) = app_state
        .database
        .lock()
        .await
        .get_blog_from_name(&blog_name)
        .await
    {
        Blog::deal_with_db(row).await
    } else {
        Blog {
            name: blog_name,
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            cover: "".to_string(),
            series: "".to_string(),
            update_time: Utc::now(),
        }
    };

    BlogTemplate { item: blog_data }
}
