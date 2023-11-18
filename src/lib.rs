// 模块树
pub mod db;
pub mod io;
pub mod pages;

pub mod run {
    use crate::db::Database;
    use crate::io::config_toml::Config;

    pub async fn init() {
        let config: Config = Config::new();
        let _db = Database::new(config.database.get()).await;

        // let answer = db.get_all_series().await;
        // println!("{:?}", answer);
    }
}

use askama::Template;

#[derive(Template)]
#[template(path = "list.html")]
pub struct ListTemplate<'a> {
    title: &'a str,
    cover: &'a str,
    description: &'a str,
}

pub async fn list() -> ListTemplate<'static> {
    ListTemplate {
        title: "TiTlE",
        cover: "cOvEr",
        description: "dEsCrIpTiOn",
    }
}
