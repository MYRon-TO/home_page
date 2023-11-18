// 模块树
pub mod db;
pub mod io;

pub mod run{
  use crate::io::config_toml::Config;
  use crate::db::Database;

  pub async fn init(){
    let config: Config = Config::new();
    let _db = Database::new(config.database.get()).await;

    // let answer = db.get_all_series().await;
    // println!("{:?}", answer);

  }
}
