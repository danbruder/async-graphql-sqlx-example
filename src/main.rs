mod graphql;
mod result;
mod todo;
mod web;

use dotenv::dotenv;
use result::Result;
use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let pool = SqlitePool::new("sqlite://./db.sqlite3").await.unwrap();

    sqlx::query!("CREATE TABLE IF NOT EXISTS todo (id TEXT PRIMARY KEY NOT NULL, body TEXT NOT NULL, complete BOOLEAN NOT NULL) ")
            .execute(&pool)
            .await?;

    web::start(pool).await;
    Ok(())
}
