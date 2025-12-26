use askama::Template;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use sqlx::{MySql, Pool, query_as};

use crate::error::AppResult;
use crate::{
    models::posts::Posts,
    templates::{
        blog::BlogTemplate,
        home::{AboutMeTemplate, ContactInfoTemplate, HomeTemplate},
    },
};

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

pub async fn blog(State(pool): State<Pool<MySql>>) -> AppResult<impl IntoResponse> {
    let posts = query_as::<_, Posts>(
        r#"
        SELECT * FROM posts;
    "#,
    )
    .fetch_all(&pool)
    .await?;

    let template = BlogTemplate { posts: posts };

    Ok(Html(template.render().unwrap()))
}
