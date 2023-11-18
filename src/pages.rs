use askama::Template;

#[derive(Template)]
#[template(source = "<h1>hello {{ name }}</h1>",ext = "html",)]
pub struct HelloTemplate<'a> {
    name: &'a str,
}

pub async fn hello_world() ->HelloTemplate<'static> {
    HelloTemplate { name: "world" }
}

