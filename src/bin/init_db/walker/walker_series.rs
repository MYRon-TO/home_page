use super::*;
use backend::io::series_js;

use tokio::sync::Mutex;

pub async fn walk_series(db: Arc<Mutex<Database>>) -> Result<(), Box<dyn std::error::Error>> {
    // 获取数据库连接
    let mut db = db.lock().await;

    let walker = WalkDir::new("assets/series/").into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
      let entry = entry?;
      if entry.file_name() == "info.json" {

        // println!("{}", entry.path().display());

        let info_path = entry.path().display().to_string();
        let js = series_js::get_series_json(&info_path).await?;
        let name = js.get_title();

        // 写入数据库
        db.write_series(&info_path, &name).await?;

      }
    }
    Ok(())
}
