use sqlx::{MySql, Pool};

pub async fn run_migrations(_pool: &Pool<MySql>) -> Result<(), sqlx::Error>{
    // I will add migrations here

    Ok(())
}
