use async_graphql::http::playground_source;
use async_graphql::{Context, EmptyMutation, EmptySubscription, FieldResult, Schema};
use async_std::task;
use async_trait::async_trait;
use dataloader::cached::Loader;
use dataloader::BatchFn;
use sqlx::{sqlite::SqliteQueryAs, Pool, SqliteConnection};
use std::collections::HashMap;
use std::env;
use std::result;
use tide::{
    http::{headers, mime},
    Request, Response, StatusCode,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[async_graphql::SimpleObject]
#[derive(sqlx::FromRow, Clone)]
pub struct Book {
    id: i32,
    name: String,
    author: String,
}

#[derive(Clone)]
enum BatchFnLoadError {
    NotFound,
    DBError(String),
}

pub struct BookBatcher(Pool<SqliteConnection>);
impl BookBatcher {
    fn new(sqlite_pool: Pool<SqliteConnection>) -> Self {
        Self(sqlite_pool)
    }
}
type BookBatcherLoadHashMapValue = result::Result<Book, BatchFnLoadError>;

#[async_trait]
impl BatchFn<i32, BookBatcherLoadHashMapValue> for BookBatcher {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, BookBatcherLoadHashMapValue> {
        println!("load book by batch {:?}", keys);

        if keys.contains(&9) {
            return keys
                .iter()
                .map(|k| {
                    (
                        *k,
                        Err(BatchFnLoadError::DBError("MOCK DBError".to_owned())),
                    )
                })
                .collect();
        }

        let stmt = format!(
            r#"SELECT id, name, author FROM books WHERE id in ({})"#,
            (0..keys.len())
                .map(|i| format!("${}", i + 1))
                .collect::<Vec<String>>()
                .join(",")
        );

        let books: result::Result<Vec<Book>, sqlx::Error> = keys
            .iter()
            .fold(sqlx::query_as(&stmt), |q, key| q.bind(key))
            .fetch_all(&self.0)
            .await;

        match books {
            Ok(books) => {
                let books_map = books.into_iter().map(|book| (book.id, Ok(book))).collect();

                keys.iter().fold(
                    books_map,
                    |mut map: HashMap<i32, BookBatcherLoadHashMapValue>, key| {
                        map.entry(*key).or_insert(Err(BatchFnLoadError::NotFound));
                        map
                    },
                )
            }
            Err(e) => keys
                .iter()
                .map(|k| (*k, Err(BatchFnLoadError::DBError(e.to_string()))))
                .collect(),
        }
    }
}

struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn book(&self, ctx: &Context<'_>, id: i32) -> FieldResult<Option<Book>> {
        println!("pre load book by id {:?}", id);
        match ctx
            .data::<Loader<i32, BookBatcherLoadHashMapValue, BookBatcher>>()
            .load(id)
            .await
        {
            Ok(book) => Ok(Some(book)),
            Err(err) => match err {
                BatchFnLoadError::NotFound => Ok(None),
                BatchFnLoadError::DBError(db_err) => Err(db_err.into()),
            },
        }
    }
}

struct AppState {
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
}

fn main() -> Result<()> {
    task::block_on(run())
}

async fn run() -> Result<()> {
    let sqlite_pool: Pool<SqliteConnection> = Pool::new("sqlite::memory:").await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            author TEXT NOT NULL
        );
        "#,
    )
    .execute(&sqlite_pool)
    .await?;

    sqlx::query(
        r#"
        INSERT OR IGNORE INTO books (id, name, author)
        VALUES (1, 'name1', 'author1'), (2, 'name2', 'author2'), (3, 'name3', 'author3')
        ;
        "#,
    )
    .execute(&sqlite_pool)
    .await?;

    let book_loader = Loader::new(BookBatcher::new(sqlite_pool));

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(book_loader)
        .finish();

    let app_state = AppState { schema };
    let mut app = tide::with_state(app_state);

    async fn graphql(req: Request<AppState>) -> tide::Result<Response> {
        let schema = req.state().schema.clone();
        async_graphql_tide::graphql(req, schema, |query_builder| query_builder).await
    }

    app.at("/graphql").post(graphql).get(graphql);
    app.at("/").get(|_| async move {
        let resp = Response::new(StatusCode::Ok)
            .body_string(playground_source("/graphql", None))
            .set_header(headers::CONTENT_TYPE, mime::HTML.to_string());

        Ok(resp)
    });

    let listen_addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "localhost:8000".to_owned());
    println!("Playground: http://{}", listen_addr);
    app.listen(listen_addr).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::prelude::*;
    use serde_json::{json, Value};
    use std::time::Duration;

    #[test]
    fn sample() -> Result<()> {
        task::block_on(async {
            let listen_addr = find_listen_addr().await;
            env::set_var("LISTEN_ADDR", format!("{}", listen_addr));

            let server: task::JoinHandle<Result<()>> = task::spawn(async move {
                run().await?;

                Ok(())
            });

            let client: task::JoinHandle<Result<()>> = task::spawn(async move {
                let listen_addr = env::var("LISTEN_ADDR").unwrap();

                task::sleep(Duration::from_millis(300)).await;

                //
                let string = surf::post(format!("http://{}/graphql", listen_addr))
                    .body_string(
                        r#"{"query":"{ book1: book(id: 1) {id, name, author} book2: book(id: 2) {id, name, author} book3: book(id: 3) {id, name, author} book4: book(id: 4) {id, name, author} }"}"#
                            .to_owned(),
                    )
                    .set_header("Content-Type", "application/json")
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
                let string = surf::post(format!("http://{}/graphql", listen_addr))
                    .body_string(
                        r#"{"query":"{ book1: book(id: 1) {id, name, author} book4: book(id: 4) {id, name, author} book9: book(id: 9) {id, name, author} }"}"#
                            .to_owned(),
                    )
                    .set_header("Content-Type", "application/json")
                    .recv_string()
                    .await?;
                println!("{}", string);

                let v: Value = serde_json::from_str(&string)?;
                let error = v["errors"].as_array().unwrap()[0].clone();
                assert_eq!(error["message"], json!("MOCK DBError"));
                assert_eq!(error["path"].to_string(), r#"["book9"]"#);

                Ok(())
            });

            server.race(client).await?;

            Ok(())
        })
    }

    async fn find_listen_addr() -> async_std::net::SocketAddr {
        async_std::net::TcpListener::bind("localhost:0")
            .await
            .unwrap()
            .local_addr()
            .unwrap()
    }
}
