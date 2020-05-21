use actix_cors::Cors;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::playground_source;
use async_graphql::{Deferred, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_actix_web::{GQLRequest, GQLResponseStream};
use std::time::Duration;

type DeferSchema = Schema<Query, EmptyMutation, EmptySubscription>;

struct Comment {
    user: String,
    text: String,
}

#[Object]
impl Comment {
    async fn user(&self) -> &str {
        actix_rt::time::delay_for(Duration::from_secs(2)).await;
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

    async fn comments(&self) -> Deferred<Vec<Comment>> {
        if self.id == 1 {
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
            .into()
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
            .into()
        } else {
            Vec::new().into()
        }
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

async fn index(schema: web::Data<DeferSchema>, req: GQLRequest) -> GQLResponseStream {
    req.into_inner().execute_stream(&schema).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/", Some("/"))))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default())
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
