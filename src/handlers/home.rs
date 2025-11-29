use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::templates::{
    home::HomeTemplate, 
    contact_info::ContactInfoTemplate
};

pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate {
        site_name: "logfort".to_string(),
    };

    Html(template.render().unwrap())
}

pub async fn contact_info() -> impl IntoResponse {
    let template = ContactInfoTemplate {
        site_name: "logfort".to_string(),
    };

    Html(template.render().unwrap())
}
