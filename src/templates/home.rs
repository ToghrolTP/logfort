use askama::Template;

pub trait WithSiteContext {
    fn site_name(&self) -> &'static str {
        "LogforT"
    }
}

#[derive(Template)]
#[template(path="home.html")]
pub struct HomeTemplate;

impl WithSiteContext for HomeTemplate {}

#[derive(Template)]
#[template(path="contact-info.html")]
pub struct ContactInfoTemplate;

impl WithSiteContext for ContactInfoTemplate {}

#[derive(Template)]
#[template(path="about_me.html")]
pub struct AboutMeTemplate;
impl WithSiteContext for AboutMeTemplate {}
