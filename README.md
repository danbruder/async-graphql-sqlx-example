# Rust GraphQL API Example

Minimal example using [async graphql](https://github.com/async-graphql/async-graphql), [sqlx](https://github.com/launchbadge/sqlx), [warp](https://github.com/seanmonstar/warp), and [sqlite](https://www.sqlite.org/index.html).

The GraphQL api has CRUD actions on todos as well as subscriptions to changes.

## Running

Clone the repo and run

```bash
cargo run
```

Open up `http://localhost:8080` to see the GraphQL playground

Create a subscription in one window:

```graphql
subscription {
  todos {
    id
    mutationType
    item
  }
}
```

Next, create a new todo in another window:

```graphql
mutation {
  createTodo(body: "New todo!", complete: false) {
    id
    body
  }
}
```
