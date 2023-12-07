use super::*;

use tokio::{fs::File, io::AsyncReadExt};

/// ## 博客的json文件结构
/// BlogJS:{
///    title: String,
///    desc: String,
///    has_cover: bool,
/// }
#[derive(Debug, Deserialize)]
pub struct SeriesJs {
    title: String,
    desc: String,
    has_cover: bool,
}

impl SeriesJs {
  pub fn get_title(&self) -> String {
    self.title.clone()
  }
  pub fn get_desc(&self) -> String {
    self.desc.clone()
  }
  pub fn get_has_cover(&self) -> bool {
    self.has_cover
  }
}

/// ### 读取博客的json文件
/// path: &str 博客的json文件路径
/// return: Result<BlogJs, std::io::Error>
///
pub async fn get_series_json(path: &str) -> Result<SeriesJs, std::io::Error> {
    // 读取文件
    let mut file = File::open(path).await?;

    let mut data = Vec::new();
    file.read_to_end(&mut data).await?;

    let info: SeriesJs = serde_json::from_reader(&data[..])?;
    return Ok(info);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_series_json() {
        let series = get_series_json("assets/series/计算机网络/info.json")
            .await
            .unwrap();
        assert_eq!(series.title, "计算机网络");
        assert_eq!(series.desc, "这里放的是计算机网络的笔记");
        assert_eq!(series.has_cover, true);
    }
}
