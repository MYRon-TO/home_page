use super::*;

#[derive(Debug)]
pub struct SeriesDb {
    name: String,
    create_time: DateTime<Utc>,
    info_path: String,
}

impl SeriesDb {
    pub fn new(name: String, create_time: DateTime<Utc>, info_path: String) -> Self {
        SeriesDb {
            name,
            create_time,
            info_path,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_create_time(&self) -> DateTime<Utc> {
        self.create_time.clone()
    }

    pub fn get_info_path(&self) -> String {
        self.info_path.clone()
    }
}

