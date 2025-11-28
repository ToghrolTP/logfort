use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::templates::home::HomeTemplate;

pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate {
        site_name: "logfort".to_string(),
    };

    Html(template.render().unwrap())
}
