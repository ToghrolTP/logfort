use askama::Template;

pub trait WithSiteContext {
    fn site_name(&self) -> &'static str {
        "LogforT"
    }
}

// Home
#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;
impl WithSiteContext for HomeTemplate {}

// Contact Info
#[derive(Template)]
#[template(path = "contact-info.html")]
pub struct ContactInfoTemplate;
impl WithSiteContext for ContactInfoTemplate {}

// About Me
#[derive(Template)]
#[template(path = "about_me.html")]
pub struct AboutMeTemplate;
impl WithSiteContext for AboutMeTemplate {}
