use crate::todo::*;
use async_graphql::{Context, FieldResult, SimpleBroker, ID};
use futures::Stream;
use sqlx::sqlite::SqlitePool;
use tokio::stream::StreamExt;

#[async_graphql::Object]
impl Todo {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn body(&self) -> &str {
        &self.body
    }

    async fn complete(&self) -> &bool {
        &self.complete
    }
}

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn todos(&self, ctx: &Context<'_>) -> FieldResult<Vec<Todo>> {
        let pool = ctx.data::<SqlitePool>();
        let items = Todo::list(&pool).await?;
        Ok(items)
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn create_todo(
        &self,
        ctx: &Context<'_>,
        body: String,
        complete: bool,
    ) -> FieldResult<Todo> {
        let pool = ctx.data::<SqlitePool>();
        let item = Todo::insert(&pool, &body, complete).await?;

        SimpleBroker::publish(TodoChanged {
            mutation_type: MutationType::Created,
            id: item.clone().id.into(),
            item: Some(item.clone()),
        });

        Ok(item)
    }

    async fn delete_todo(&self, ctx: &Context<'_>, id: ID) -> FieldResult<bool> {
        let pool = ctx.data::<SqlitePool>();
        let id = id.parse::<String>()?;

        Todo::delete(&pool, &id).await?;

        SimpleBroker::publish(TodoChanged {
            mutation_type: MutationType::Deleted,
            id: id.into(),
            item: None,
        });

        Ok(true)
    }

    async fn update_todo(
        &self,
        ctx: &Context<'_>,
        id: ID,
        body: String,
    ) -> FieldResult<Option<Todo>> {
        let pool = ctx.data::<SqlitePool>();
        let id = id.parse::<String>()?;

        let item = Todo::update(&pool, &id, &body).await?;

        SimpleBroker::publish(TodoChanged {
            mutation_type: MutationType::Updated,
            id: id.into(),
            item: item.clone(),
        });

        Ok(item)
    }
    async fn set_complete(
        &self,
        ctx: &Context<'_>,
        id: ID,
        complete: bool,
    ) -> FieldResult<Option<Todo>> {
        let pool = ctx.data::<SqlitePool>();
        let id = id.parse::<String>()?;

        let item = Todo::set_complete(&pool, &id, complete).await?;

        SimpleBroker::publish(TodoChanged {
            mutation_type: MutationType::Updated,
            id: id.into(),
            item: item.clone(),
        });

        Ok(item)
    }
}

#[async_graphql::Enum]
#[derive(Copy, Clone)]
enum MutationType {
    Created,
    Updated,
    Deleted,
}

#[async_graphql::SimpleObject]
#[derive(Clone)]
struct TodoChanged {
    mutation_type: MutationType,
    id: ID,
    item: Option<Todo>,
}

pub struct SubscriptionRoot;

#[async_graphql::Subscription]
impl SubscriptionRoot {
    async fn todos(&self, mutation_type: Option<MutationType>) -> impl Stream<Item = TodoChanged> {
        SimpleBroker::<TodoChanged>::subscribe().filter(move |event| {
            if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            }
        })
    }
}
