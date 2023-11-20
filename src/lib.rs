// 模块树
pub mod db;
pub mod io;
pub mod pages;

pub mod run {
    use crate::db::Database;
    use crate::io::config_toml::Config;
    use std::sync::{Arc, Mutex}; // 用于共享状态

    #[derive(Clone)]
    pub struct AppState {
        pub database: Arc<Mutex<Database>>,
    }

    impl AppState {
        pub async fn init() -> Self {
            let config: Config = Config::new();
            let db = Database::new(config.database.get()).await;

            Self {
                database: Arc::new(Mutex::new(db)),
            }
        }
    }
}
