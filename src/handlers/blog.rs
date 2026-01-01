use crate::{error::AppResult, models::posts::Post, templates::blog::{BlogTemplate, PostDetailsTemplates}};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use sqlx::{MySql, Pool, query_as};

pub async fn blog(State(pool): State<Pool<MySql>>) -> AppResult<impl IntoResponse> {
    let posts = query_as::<_, Post>(
        r#"
        SELECT * FROM posts;
    "#,
    )
    .fetch_all(&pool)
    .await?;

    let template = BlogTemplate { posts: posts };

    Ok(Html(template.render().unwrap()))
}

pub async fn post_details(
    State(pool): State<Pool<MySql>>,
    Path(id): Path<i32>,
) -> AppResult<impl IntoResponse> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?;

    match post {
        Some(post) => {
            let template = PostDetailsTemplates { post };
            Ok(Html(template.render().unwrap()))
        },
        None => {
            Ok(Html("<h1>404: Post not found in the archives</h1>".to_string()))
        }
    }
}
