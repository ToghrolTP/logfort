use sqlx::{MySqlPool, Pool, MySql};

pub async fn create_pool() -> Result<Pool<MySql>, sqlx::Error> {
    let pool = MySqlPool::connect("mysql://logfort:6150062929@localhost:3306/logfort_db").await;

    pool
}
