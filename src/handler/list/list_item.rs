#[derive(Clone)]
pub enum ListItem {
    Series(SeriesListItem),
    Tags(TagsListItem),
    Blogs(BlogsListItem),
}

impl ListItem {
    pub fn new_series(title: String, cover: String, link: String, desc: String) -> Self {
        ListItem::Series(SeriesListItem::new(title, cover, link, desc))
    }

    pub fn new_blogs(title: String, cover: String, link: String, desc: String) -> Self {
        ListItem::Blogs(BlogsListItem::new(title, cover, link, desc))
    }

    pub fn new_tags(title: String, link: String) -> Self {
        ListItem::Tags(TagsListItem::new(title, link))
    }

    pub fn get_title(&self) -> String {
        match self {
            ListItem::Series(series) => series.title.clone(),
            ListItem::Blogs(blogs) => blogs.title.clone(),
            ListItem::Tags(tags) => tags.title.clone(),
        }
    }

    pub fn get_cover(&self) -> String {
        match self {
            ListItem::Series(series) => series.cover.clone(),
            ListItem::Blogs(blogs) => blogs.cover.clone(),
            ListItem::Tags(_) => String::new(),
        }
    }

    pub fn get_link(&self) -> String {
        match self {
            ListItem::Series(series) => series.link.clone(),
            ListItem::Blogs(blogs) => blogs.link.clone(),
            ListItem::Tags(tags) => tags.link.clone(),
        }
    }

    pub fn get_desc(&self) -> String {
        match self {
            ListItem::Series(series) => series.desc.clone(),
            ListItem::Blogs(blogs) => blogs.desc.clone(),
            ListItem::Tags(_) => String::new(),
        }
    }
}

#[derive(Clone)]
pub struct BlogsListItem {
    pub title: String,
    pub desc: String,
    pub cover: String,
    pub link: String,
}

#[derive(Clone)]
pub struct TagsListItem {
    pub title: String,
    pub link: String,
}

#[derive(Clone)]
pub struct SeriesListItem {
    pub title: String,
    pub desc: String,
    pub cover: String,
    pub link: String,
}

impl SeriesListItem {
    pub fn new(title: String, cover: String, link: String, desc: String) -> Self {
        SeriesListItem {
            title,
            cover,
            link,
            desc,
        }
    }
}

impl BlogsListItem {
    pub fn new(title: String, cover: String, link: String, desc: String) -> Self {
        BlogsListItem {
            title,
            cover,
            link,
            desc,
        }
    }
}

impl TagsListItem {
    pub fn new(title: String, link: String) -> Self {
        TagsListItem {
            title,
            link,
        }
    }
}
