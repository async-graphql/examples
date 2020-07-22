use actix_web::{guard, web, App, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, IntoQueryBuilderOpts, Schema};
use async_graphql_actix_web::{BatchGQLRequest, BatchGQLResponse};
use files::{FilesSchema, MutationRoot, QueryRoot, Storage};

async fn index(schema: web::Data<FilesSchema>, req: BatchGQLRequest) -> BatchGQLResponse {
    req.into_inner().execute(&schema).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        .finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index).app_data(
                IntoQueryBuilderOpts {
                    max_num_files: Some(3),
                    ..IntoQueryBuilderOpts::default()
                },
            ))
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
