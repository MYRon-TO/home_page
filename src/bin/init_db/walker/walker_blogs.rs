use super::*;
use backend::io::blog_js;

use tokio::sync::Mutex;

pub async fn walk_blogs(db: Arc<Mutex<Database>>) -> Result<(), Box<dyn std::error::Error>> {
    // 获取数据库连接
    let mut db = db.lock().await;

    // 遍历博客目录
    let walker = WalkDir::new("assets/blogs/").into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
      let entry = entry?;
      // 查找info.json
      if entry.file_name() == "info.json" {

        // println!("{}", entry.path().display());

        let info_path = entry.path().display().to_string();
        let js = blog_js::get_blog_json(&info_path).await?; // 读取博客json文件
        let series = js.get_series().to_string();
        let tags = js.get_tag().to_vec();

        // 写入数据库
        db.write_blogs_series(&info_path, &series).await?;
        let blog_id = db.get_blog_id(&info_path).await?;
        db.write_tag_blog(blog_id, &tags).await?;

      }
    }
    Ok(())
}

