#[macro_use]
extern crate thiserror;

use actix_web::{guard, web, App, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{
    EmptyMutation, EmptySubscription, ErrorExtensions, FieldError, FieldResult, Object, ResultExt,
    Schema,
};
use async_graphql_actix_web::{Request, Response};

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Could not find resource")]
    NotFound,

    #[error("ServerError")]
    ServerError(String),

    #[error("No Extensions")]
    ErrorWithoutExtensions,
}

impl ErrorExtensions for MyError {
    // lets define our base extensions
    fn extend(self) -> FieldError {
        self.extend_with(|err, e| match err {
            MyError::NotFound => e.set("code", "NOT_FOUND"),
            MyError::ServerError(reason) => e.set("reason", reason.to_string()),
            MyError::ErrorWithoutExtensions => {}
        })
    }
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    // It works on foreign types without extensions as before
    async fn parse_without_extensions(&self) -> FieldResult<i32> {
        Ok("234a".parse()?)
    }

    // Foreign types can be extended
    async fn parse_with_extensions(&self) -> FieldResult<i32> {
        Ok("234a"
            .parse()
            .map_err(|e: std::num::ParseIntError| e.extend_with(|_, e| e.set("code", 404)))?)
    }

    // THIS does unfortunately NOT work because ErrorExtensions is implemented for &E and not E.
    // Which is necessary for the overwrite by the user.

    // async fn parse_with_extensions_result(&self) -> FieldResult<i32> {
    //    Ok("234a".parse().extend_err(|_| json!({"code": 404}))?)
    // }

    // Using our own types we can implement some base extensions
    async fn extend(&self) -> FieldResult<i32> {
        Err(MyError::NotFound.extend())
    }

    // Or on the result
    async fn extend_result(&self) -> FieldResult<i32> {
        Err(MyError::NotFound).extend()
    }

    // Base extensions can be further extended
    async fn more_extensions(&self) -> FieldResult<String> {
        // resolves to extensions: { "code": "NOT_FOUND", "reason": "my reason" }
        Err(MyError::NotFound.extend_with(|_e, e| e.set("reason", "my reason")))
    }

    // works with results as well
    async fn more_extensions_on_result(&self) -> FieldResult<String> {
        // resolves to extensions: { "code": "NOT_FOUND", "reason": "my reason" }
        Err(MyError::NotFound).extend_err(|_e, e| e.set("reason", "my reason"))
    }

    // extend_with is chainable
    async fn chainable_extensions(&self) -> FieldResult<String> {
        let err = MyError::NotFound
            .extend_with(|_, e| e.set("ext1", 1))
            .extend_with(|_, e| e.set("ext2", 2))
            .extend_with(|_, e| e.set("ext3", 3));
        Err(err)
    }

    // extend_with overwrites keys which are already present
    async fn overwrite(&self) -> FieldResult<String> {
        Err(MyError::NotFound.extend_with(|_, e| e.set("code", "overwritten")))
    }
}

async fn index(
    schema: web::Data<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    req: Request,
) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .data(Schema::new(QueryRoot, EmptyMutation, EmptySubscription))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
