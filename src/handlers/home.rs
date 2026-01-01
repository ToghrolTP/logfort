use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::templates::home::{AboutMeTemplate, ContactInfoTemplate, HomeTemplate};

pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate;

    Html(template.render().unwrap())
}

pub async fn contact_info() -> impl IntoResponse {
    let template = ContactInfoTemplate;

    Html(template.render().unwrap())
}

pub async fn about_me() -> impl IntoResponse {
    let template = AboutMeTemplate;

    Html(template.render().unwrap())
}
