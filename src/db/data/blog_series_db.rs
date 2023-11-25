use super::*;

#[derive(Debug)]
pub struct BlogSeriesDb {
    update_time: DateTime<Utc>,
    info_path: String,
    series: String,
    name: String,
}

impl BlogSeriesDb {
  pub fn new(info_path: String, update_time: DateTime<Utc>, series: String, name: String) -> Self{
    BlogSeriesDb{
      info_path,
      update_time,
      series,
      name,
    }
  }

  pub fn get_update_time(&self) -> DateTime<Utc> {
    self.update_time.clone()
  }

  pub fn get_info_path(&self) -> String {
    self.info_path.clone()
  }

  pub fn get_series(&self) -> String {
    self.series.clone()
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }
}
