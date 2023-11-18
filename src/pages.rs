use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(source = "<h1>hello {{ name }}</h1>",ext = "html",)]
struct HelloTemplate<'a> {
    name: &'a str,
}

pub async fn hello_world() ->Html<String> {
  Html(HelloTemplate { name: "world" }.render().unwrap())
}
