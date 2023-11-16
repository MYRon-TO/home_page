use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct SeriesDb {
    name: String,
    create_time: DateTime<Utc>,
    info_path: String,
}

impl SeriesDb {
    pub fn new(name: String, create_time: DateTime<Utc>, info_path: String) -> SeriesDb {
        SeriesDb {
            name,
            create_time,
            info_path,
        }
    }
}
