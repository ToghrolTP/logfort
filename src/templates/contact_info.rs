use askama::Template;


#[derive(Template)]
#[template(path="contact-info.html")]
pub struct ContactInfoTemplate {
    pub site_name: String,
}
