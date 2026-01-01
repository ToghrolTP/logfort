use crate::{models::posts::Post, templates::{WithSiteContext}};
use askama::Template;

#[derive(Template)]
#[template(path = "blog/blog.html")]
pub struct BlogTemplate {
    pub posts: Vec<Post>,
}
impl WithSiteContext for BlogTemplate {}

#[derive(Template)]
#[template(path = "blog/post_details.html")]
pub struct PostDetailsTemplates {
    pub post: Post,
}
impl WithSiteContext for PostDetailsTemplates {}
