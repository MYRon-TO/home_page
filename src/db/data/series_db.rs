use super::*;

#[derive(Debug)]
pub struct SeriesDb {
    name: String,
    update_time: DateTime<Utc>,
    info_path: String,
}

impl SeriesDb {
    pub fn new(name: String, update_time: DateTime<Utc>, info_path: String) -> Self {
        SeriesDb {
            name,
            update_time,
            info_path,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_update_time(&self) -> DateTime<Utc> {
        self.update_time.clone()
    }

    pub fn get_info_path(&self) -> String {
        self.info_path.clone()
    }
}

