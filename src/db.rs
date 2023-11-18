pub mod data;
use crate::db::data::SeriesDb;
use std::vec;

// sql
use sqlx::{
    mysql::{
        // MySql,
        MySqlPool,
        MySqlPoolOptions,
    },
    Row,
    types::chrono::{DateTime, Utc},
};

// date

pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub async fn new(db: String) -> Database {
        // 创建连接池
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(db.as_str())
            .await
            .expect("连接数据库失败");

        Database { pool }
    }

    // async fn execute_query(&self, query: &str) -> Result<(), sqlx::Error> {
    //     match sqlx::query(query).execute(&self.pool).await {
    //         Ok(_) => Ok(()),
    //         Err(e) => Err(e),
    //     }
    // }

    async fn select(&self, query: &str) -> Result<Vec<sqlx::mysql::MySqlRow>, sqlx::Error> {
        match sqlx::query(query).fetch_all(&self.pool).await {
            Ok(rows) => Ok(rows),
            Err(e) => Err(e),
        }
    }

    // pub async fn get_all_series(&self) -> Result<Vec<sqlx::mysql::MySqlRow>, sqlx::Error> {
    pub async fn get_all_series(&self) -> Vec<SeriesDb> {
        let query = "SELECT * FROM series";

        let mut data: Vec<SeriesDb> = vec![];

        if let Ok(rows) = self.select(query).await {
            for row in rows {
                let name: String = row.get::<String, _>("name");
                let create_time: DateTime<Utc> = row.get::<DateTime<Utc>, _>("create_time");
                let info_path: String = row.get::<String, _>("info_path");
                data.push(SeriesDb::new(name, create_time, info_path));
            }
        }
        return data;
    }
}
