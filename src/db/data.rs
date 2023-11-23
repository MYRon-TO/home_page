use sqlx::types::chrono::{DateTime, Utc};

use self::blog_series_db::BlogSeriesDb;

pub mod series_db;
pub mod blog_series_db;
pub mod tag_blog_db;

#[derive(Debug)]
pub enum DbData{
  Series(series_db::SeriesDb),
  BlogSeries(BlogSeriesDb),
  TagBlog(tag_blog_db::TagBlogDb),
}

impl DbData {
  pub fn new_series(name: String, create_time: DateTime<Utc>, info_path: String) -> Self {
    DbData::Series(series_db::SeriesDb::new(name, create_time, info_path))
  }

  pub fn new_blog_series(name: String, create_time: DateTime<Utc>, info_path: String) -> Self {
    DbData::BlogSeries(BlogSeriesDb::new(name, create_time, info_path))
  }

  pub fn new_tag(name: String) -> Self {
    DbData::TagBlog(tag_blog_db::TagBlogDb::new(name))
  }
}
