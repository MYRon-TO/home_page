use sqlx::{mysql::MySqlPoolOptions, Row};
use sqlx_mysql::MySqlPool;

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

    pub async fn drop_it(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sql_1 = "drop table if exists tag_blog;";
        let sql_2 = "drop table if exists blog_series;";
        let sql_3 = "drop table if exists series;";

        self.do_sql(sql_1.to_string()).await?;
        self.do_sql(sql_2.to_string()).await?;
        self.do_sql(sql_3.to_string()).await?;

        Ok(())
    }

    pub async fn create_tabes(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sql_1 = r#"
CREATE TABLE IF NOT EXISTS series (
    name VARCHAR(255) PRIMARY KEY,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    info_path VARCHAR(255) NOT NULL
    );
       "#;

        let sql_2 = r#"
CREATE TABLE IF NOT EXISTS blog_series (
    id INT AUTO_INCREMENT PRIMARY KEY,
    info_path VARCHAR(255) NOT NULL,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    series VARCHAR(255),
    FOREIGN KEY (series) REFERENCES series(name)
      ON DELETE CASCADE
      ON UPDATE CASCADE
    );
        "#;

        let sql_3 = r#"
CREATE TABLE IF NOT EXISTS tag_blog (
    tag VARCHAR(255) NOT NULL,
    blog_id INT NOT NULL,
    PRIMARY KEY (blog_id, tag),
    FOREIGN KEY (blog_id) REFERENCES blog_series(id)
      ON DELETE CASCADE
      ON UPDATE CASCADE
    );
        "#;

        self.do_sql(sql_1.to_string()).await?;
        self.do_sql(sql_2.to_string()).await?;
        self.do_sql(sql_3.to_string()).await?;

        Ok(())
    }

    pub async fn write_series(
        &mut self,
        info_path: &String,
        name: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sql = format!(
            "INSERT INTO series (info_path, name) VALUES ('{}', '{}')",
            info_path, name
        );

        self.do_sql(sql).await?;

        Ok(())
    }

    /// ## 写入博客信息
    pub async fn write_blogs_series(
        &mut self,
        info_path: &String,
        series: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sql = format!(
            "INSERT INTO blog_series (info_path, series) VALUES ('{}', '{}')",
            info_path, series
        );

        self.do_sql(sql).await?;

        Ok(())
    }

    /// ## 写入标签信息
    pub async fn write_tag_blog(
        &mut self,
        blog_id: i32,
        tags: &Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for tag in tags {
            let sql = format!(
                "INSERT INTO tag_blog (blog_id, tag) VALUES ('{}', '{}')",
                blog_id, tag
            );
            self.do_sql(sql).await?;
        }

        Ok(())
    }

    /// ## 获取博客id
    pub async fn get_blog_id(
        &mut self,
        info_path: &String,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sql = format!(
            "SELECT id FROM blog_series WHERE info_path = '{}'",
            info_path
        );

        let id = sqlx::query(sql.as_str())
            .fetch_one(&self.pool)
            .await?
            .get("id");

        Ok(id)
    }

    /// ## 执行sql语句
    async fn do_sql(&mut self, sql: String) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query(sql.as_str()).execute(&self.pool).await?;

        Ok(())
    }
}
