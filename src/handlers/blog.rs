use crate::{error::AppResult, models::posts::Posts, templates::blog::BlogTemplate};
use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use sqlx::{MySql, Pool, query_as};

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
