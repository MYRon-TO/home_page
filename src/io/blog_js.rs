use super::*;

use tokio::{
    fs::File,
    io::AsyncReadExt,
};

#[derive(Debug, Deserialize)]
pub struct BlogJs {
    title: String,
    desc: String,
    has_cover: bool,
}

/// 读取博客的json文件
pub async fn get_blog_json(path: &str) -> BlogJs {
    // 读取文件
    let mut file = File::open(path).await.unwrap();

    // 读取文件内容
    let mut data = Vec::new();
    file.read_to_end(&mut data).await.unwrap();

    let blog: BlogJs = serde_json::from_reader(&data[..]).unwrap();
    blog
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_blog_json() {
        let blog = get_blog_json("assets/blogs/消息与信道/info.json").await;
        assert_eq!(blog.title, "消息与信道");
        assert_eq!(blog.desc, "这是一篇计算机网络的，有关消息与信道的笔记");
        assert_eq!(blog.has_cover, true);
    }
}
