use crate::result::Result;
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: String,
    pub body: String,
    pub complete: bool,
}

impl Todo {
    pub async fn insert(pool: &SqlitePool, body: &str, complete: bool) -> Result<Todo> {
        let id = Uuid::new_v4().to_string();
        sqlx::query!("INSERT INTO todo VALUES ($1, $2, $3)", id, body, complete)
            .execute(pool)
            .await?;

        Ok(Todo {
            id,
            body: body.to_string(),
            complete,
        })
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Todo>> {
        let todos = sqlx::query_as!(Todo, "SELECT * FROM todo")
            .fetch_all(pool)
            .await?;

        Ok(todos)
    }

    pub async fn update(pool: &SqlitePool, id: &str, body: &str) -> Result<Option<Todo>> {
        sqlx::query!("UPDATE todo SET body=$1 WHERE id=$2", body, id)
            .execute(pool)
            .await?;

        let todo = sqlx::query_as!(Todo, "SELECT * FROM todo WHERE id=$1", id)
            .fetch_optional(pool)
            .await?;

        Ok(todo)
    }

    pub async fn set_complete(pool: &SqlitePool, id: &str, complete: bool) -> Result<Option<Todo>> {
        sqlx::query!("UPDATE todo SET complete=$1 WHERE id=$2", complete, id)
            .execute(pool)
            .await?;

        let todo = sqlx::query_as!(Todo, "SELECT * FROM todo WHERE id=$1", id)
            .fetch_optional(pool)
            .await?;

        Ok(todo)
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<()> {
        sqlx::query!("DELETE FROM todo WHERE id=$1", id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
