// use super::*;

#[derive(Debug)]
pub struct TagBlogDb {
    name: String,
}

impl TagBlogDb {
    pub fn new(name: String) -> Self {
        TagBlogDb {
            name,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

