use std::sync::Arc;
use tokio::sync::Mutex;

mod walker;
mod db;
use db::Database;
use backend::io::config_toml::Config;

#[tokio::main]
pub async fn main() {

    let config: Config = Config::new();
    let db = Arc::new(Mutex::new(
        Database::new(config.get_manager()).await,
    ));

    db.lock().await.drop_it().await.unwrap();
    db.lock().await.create_tabes().await.unwrap();

    walker::walker_series::walk_series(Arc::clone(&db))
        .await
        .unwrap();
    walker::walker_blogs::walk_blogs(Arc::clone(&db))
        .await
        .unwrap();
}
