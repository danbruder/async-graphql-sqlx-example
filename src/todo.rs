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
    pub async fn insert(pool: &SqlitePool, body: &str) -> Result<Todo> {
        let id = Uuid::new_v4().to_string();
        sqlx::query!("INSERT INTO todo VALUES ($1, $2, $3)", id, body, false)
            .execute(pool)
            .await?;

        Ok(Todo {
            id,
            body: body.to_string(),
            complete: false,
        })
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Todo>> {
        let todos = sqlx::query_as!(Todo, "SELECT * FROM todo")
            .fetch_all(pool)
            .await?;

        Ok(todos)
    }

    pub async fn update(
        pool: &SqlitePool,
        id: &str,
        body: &str,
        complete: bool,
    ) -> Result<Option<Todo>> {
        sqlx::query!(
            "UPDATE todo SET body=$1, complete=$2 WHERE id=$3",
            body,
            complete,
            id
        )
        .execute(pool)
        .await?;

        let todo = sqlx::query_as!(Todo, "SELECT * FROM todo WHERE id=$1", id)
            .fetch_optional(pool)
            .await?;

        Ok(todo)
    }

    pub async fn toggle_complete(pool: &SqlitePool, id: &str) -> Result<Option<Todo>> {
        sqlx::query!("UPDATE todo SET complete=NOT complete WHERE id=$1", id)
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
