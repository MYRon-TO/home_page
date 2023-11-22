use super::*;

#[derive(Debug)]
pub struct BlogSeriesDb {
    create_time: DateTime<Utc>,
    info_path: String,
    series: String,
}

impl BlogSeriesDb {
  pub fn new(info_path: String, create_time: DateTime<Utc>, series: String) -> Self{
    BlogSeriesDb{
      info_path,
      create_time,
      series,
    }
  }

  pub fn get_create_time(&self) -> DateTime<Utc> {
    self.create_time.clone()
  }

  pub fn get_info_path(&self) -> String {
    self.info_path.clone()
  }

  pub fn get_series(&self) -> String {
    self.series.clone()
  }
}
