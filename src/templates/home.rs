use askama::Template;

#[derive(Template)]
#[template(path="home.html")]
pub struct HomeTemplate {
    pub site_name: String,
}
