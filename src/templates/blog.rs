use crate::{models::posts::Posts, templates::home::WithSiteContext};
use askama::Template;

#[derive(Template)]
#[template(path = "blog/blog.html")]
pub struct BlogTemplate {
    pub posts: Vec<Posts>,
}
impl WithSiteContext for BlogTemplate {}
