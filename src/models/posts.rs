use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub content: String,
    pub status: String,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
