use crate::templates::home::WithSiteContext;
use askama::Template;

#[derive(Template)]
#[template(path = "blog/blog.html")]
pub struct BlogTemplate;
impl WithSiteContext for BlogTemplate {}
