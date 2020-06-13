use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Error, Schema};
use async_graphql_actix_web::{GQLRequest, GQLResponse};

type SimpleSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

async fn index(schema: web::Data<SimpleSchema>, req: GQLRequest) -> GQLResponse {
    let query_builder = req.into_inner();
    let query_source = query_builder.query_source().to_owned();
    let response = query_builder.execute(&schema).await;

    // We are only interested in QueryErrors.
    if let Err(Error::Query { err, path, pos }) = &response {
        // Condense the query to make it fit on one line
        // Note: If you handle login through GraphQL, you'll probably want to filter the login query from logging,
        //       or at least remove the password from that query.
        let condensed_query = query_source.replace(char::is_whitespace, "");
        match path {
            Some(p) => log::error!(
                "Error: '{}' at path {}, pos {}, while resolving query: {}",
                err,
                p,
                pos,
                condensed_query
            ),
            None => log::error!(
                "Error: '{}' at pos {}, while resolving query: {}",
                err,
                pos,
                condensed_query
            ),
        };
    }

    response.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    #[field(desc = "Create an error.")]
    async fn make_error(&self) -> async_graphql::FieldResult<String> {
        // Always return an error variant to be able to display the logging.
        Err("Oh no, a terrible error happened!".into())
    }
}
