use askama::Template;

mod test; // just for test **remove it when fish**

#[derive(Template)]
#[template(path = "index.html")]
