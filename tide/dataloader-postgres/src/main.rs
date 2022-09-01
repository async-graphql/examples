use std::{collections::HashMap, env};

use async_graphql::{
    dataloader::{DataLoader, Loader},
    futures_util::TryStreamExt,
    http::GraphiQLSource,
    Context, EmptyMutation, EmptySubscription, FieldError, Object, Result, Schema, SimpleObject,
};
use async_std::task;
use async_trait::async_trait;
use itertools::Itertools;
use sqlx::{Pool, Postgres};
use tide::{http::mime, Body, Response, StatusCode};

#[derive(sqlx::FromRow, Clone, SimpleObject)]
pub struct Book {
    id: i32,
    name: String,
    author: String,
}

pub struct BookLoader(Pool<Postgres>);

impl BookLoader {
    fn new(postgres_pool: Pool<Postgres>) -> Self {
        Self(postgres_pool)
    }
}

#[async_trait]
impl Loader<i32> for BookLoader {
    type Value = Book;
    type Error = FieldError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        println!("load book by batch {:?}", keys);

        if keys.contains(&9) {
            return Err("MOCK DBError".into());
        }

        let query = format!(
            "SELECT id, name, author FROM books WHERE id IN ({})",
            keys.iter().join(",")
        );
        Ok(sqlx::query_as(&query)
            .fetch(&self.0)
            .map_ok(|book: Book| (book.id, book))
            .try_collect()
            .await?)
    }
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn book(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Book>> {
        println!("pre load book by id {:?}", id);
        ctx.data_unchecked::<DataLoader<BookLoader>>()
            .load_one(id)
            .await
    }
}

fn main() -> Result<()> {
    task::block_on(run())
}

async fn run() -> Result<()> {
    let postgres_pool: Pool<Postgres> = Pool::connect(&env::var("DATABASE_URL")?).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            author TEXT NOT NULL
        );
        "#,
    )
    .execute(&postgres_pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO books (id, name, author)
        VALUES (1, 'name1', 'author1'), (2, 'name2', 'author2'), (3, 'name3', 'author3')
        ON CONFLICT (id) DO NOTHING
        ;
        "#,
    )
    .execute(&postgres_pool)
    .await?;

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(DataLoader::new(
            BookLoader::new(postgres_pool),
            async_std::task::spawn,
        ))
        .finish();

    let mut app = tide::new();

    app.at("/graphql").post(async_graphql_tide::graphql(schema));
    app.at("/").get(|_| async move {
        let mut resp = Response::new(StatusCode::Ok);
        resp.set_body(Body::from_string(
            GraphiQLSource::build()
                .endpoint("http://localhost:8000/graphql")
                .finish(),
        ));
        resp.set_content_type(mime::HTML);
        Ok(resp)
    });

    println!("GraphiQL IDE: http://127.0.0.1:8000");
    app.listen("127.0.0.1:8000").await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use async_std::prelude::*;
    use serde_json::{json, Value};

    use super::*;

    #[test]
    fn sample() -> Result<()> {
        task::block_on(async {
            let server: task::JoinHandle<Result<()>> = task::spawn(async move {
                run().await?;
                Ok(())
            });

            let client: task::JoinHandle<Result<()>> = task::spawn(async move {
                task::sleep(Duration::from_millis(1000)).await;

                //
                let string = surf::post("http://127.0.0.1:8000/graphql")
                    .body(
                        Body::from(r#"{"query":"{ book1: book(id: 1) {id, name, author} book2: book(id: 2) {id, name, author} book3: book(id: 3) {id, name, author} book4: book(id: 4) {id, name, author} }"}"#),
                    )
                    .header("Content-Type", "application/json")
                    .recv_string()
                    .await?;
                println!("{}", string);

                let v: Value = serde_json::from_str(&string)?;
                assert_eq!(
                    v["data"]["book1"],
                    json!({"id": 1, "name": "name1", "author": "author1"})
                );
                assert_eq!(
                    v["data"]["book2"],
                    json!({"id": 2, "name": "name2", "author": "author2"})
                );
                assert_eq!(
                    v["data"]["book3"],
                    json!({"id": 3, "name": "name3", "author": "author3"})
                );
                assert_eq!(v["data"]["book4"], json!(null));

                //
                let string = surf::post(    "http://127.0.0.1:8000/graphql")
                    .body(
                        Body::from(r#"{"query":"{ book1: book(id: 1) {id, name, author} book4: book(id: 4) {id, name, author} book9: book(id: 9) {id, name, author} }"}"#),
                    )
                    .header("Content-Type", "application/json")
                    .recv_string()
                    .await?;
                println!("{}", string);

                let v: Value = serde_json::from_str(&string)?;
                let error = v["errors"].as_array().unwrap()[0].clone();
                assert_eq!(error["message"], json!("MOCK DBError"));

                Ok(())
            });

            server.race(client).await?;

            Ok(())
        })
    }
}
