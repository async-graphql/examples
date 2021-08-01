use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, MultipartOptions};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use files::{FilesSchema, MutationRoot, QueryRoot, Storage};

async fn index(schema: web::Data<FilesSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        .finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(index)
                    .app_data(MultipartOptions::default().max_num_files(3)),
            )
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
