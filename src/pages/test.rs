use askama::Template;
#[derive(Template)]
#[template(path = "test.html")]

pub struct TestTemplate<'a> {
    pub name: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template() {
        let test = TestTemplate { name: "world" };
        assert_eq!(test.render().unwrap(), "<h1>Hello, world!</h1>");
    }
}
