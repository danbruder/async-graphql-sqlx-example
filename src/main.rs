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

    web::start(pool).await;
    Ok(())
}
