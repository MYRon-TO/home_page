// sql
use sqlx::mysql::{
    // MySql,
    MySqlPool,
    MySqlPoolOptions,
};
use sqlx:: Row;

// date
// use chrono::NaiveDateTime;

pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub async fn new() -> Database {
        // 创建连接池
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect("mysql://BlueBird:%23ff0000Berry@localhost/yuru")
            .await
            .expect("连接数据库失败");

        Database { pool }
    }

    async fn execute_query(&self, query: &str) -> Result<(), sqlx::Error> {
        match sqlx::query(query).execute(&self.pool).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn select(&self, query: &str) -> Result<Vec<sqlx::mysql::MySqlRow>, sqlx::Error> {
        match sqlx::query(query).fetch_all(&self.pool).await {
            Ok(rows) => Ok(rows),
            Err(e) => {
                Err(e)
            }
        }
    }

    // pub async fn get_all_serise(&self) -> Result<Vec<sqlx::mysql::MySqlRow>, sqlx::Error> {
    pub async fn get_all_serise(&self) {
        let query = "SELECT * FROM serise";

        if let Ok(rows) = self.select(query).await {
            for row in rows {
                let name: String = row.get::<String,_>("name");
                let create_time: String = row.get::<String,_>("create_time");
                let info_path: String = row.get::<String,_>("info_path");
            }
        }
    }
}
