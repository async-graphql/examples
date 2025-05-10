use actix_web::{App, HttpResponse, HttpServer, guard, web, web::Data};
use async_graphql::{
    EmptySubscription, Schema,
    http::{GraphiQLSource, MultipartOptions},
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use files::{FilesSchema, MutationRoot, QueryRoot, Storage};

async fn index(schema: web::Data<FilesSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

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
