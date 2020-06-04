use async_graphql::http::playground_source;
use async_graphql::{Deferred, EmptyMutation, EmptySubscription, Object, QueryBuilder, Schema};
use async_graphql_warp::{BadRequest, GQLResponseStream};
use http::StatusCode;
use std::convert::Infallible;
use std::time::Duration;
use tokio::time;
use warp::{http::Response, Filter, Rejection, Reply};

struct Comment {
    user: String,
    text: String,
}

#[Object]
impl Comment {
    async fn user(&self) -> &str {
        time::delay_for(Duration::from_secs(2)).await;
        &self.user
    }

    async fn text(&self) -> &str {
        &self.text
    }
}

struct Book {
    id: i32,
    title: String,
    author: String,
}

#[Object]
impl Book {
    async fn title(&self) -> &str {
        &self.title
    }

    async fn author(&self) -> &str {
        &self.author
    }

    async fn comments(&self) -> Deferred<Option<Vec<Comment>>> {
        let comments = if self.id == 1 {
            vec![
                Comment {
                    user: "John".to_string(),
                    text: "I liked it".to_string(),
                },
                Comment {
                    user: "Mary".to_string(),
                    text: "It is a book".to_string(),
                },
            ]
        } else if self.id == 2 {
            vec![
                Comment {
                    user: "Alberta".to_string(),
                    text: "Amazing :-)".to_string(),
                },
                Comment {
                    user: "Joanna".to_string(),
                    text: "Excellent".to_string(),
                },
            ]
        } else {
            Vec::new()
        };

        Some(comments).into()
    }
}

struct Query;

#[Object]
impl Query {
    async fn books(&self) -> Vec<Book> {
        vec![
            Book {
                id: 1,
                title: "Harry Potter and the Chamber of Secrets".to_string(),
                author: "J.K. Rowling".to_string(),
            },
            Book {
                id: 2,
                title: "Jurassic Park".to_string(),
                author: "Michael Crichton".to_string(),
            },
            Book {
                id: 3,
                title: "Moby Dick".to_string(),
                author: "Herman Melville".to_string(),
            },
        ]
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    println!("Playground: http://localhost:9000");

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, builder): (_, QueryBuilder)| async move {
            let resp = builder.execute_stream(&schema).await;
            let stream: GQLResponseStream = resp.into();
            Ok::<_, Infallible>(stream.into_response())
        },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source("/", None))
    });

    let cors = warp::cors()
        .allow_credentials(true)
        .allow_any_origin()
        .allow_headers(vec!["authorization", "content-type"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);

    let routes = graphql_post
        .or(graphql_playground)
        .recover(|err: Rejection| async move {
            if let Some(BadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        })
        .with(cors);

    warp::serve(routes).run(([0, 0, 0, 0], 9000)).await;
}
