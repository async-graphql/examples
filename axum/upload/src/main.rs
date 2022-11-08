use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use files::{FilesSchema, MutationRoot, QueryRoot, Storage};
use hyper::{Method, Server};
use tower_http::cors::{CorsLayer, Origin};

async fn graphql_handler(schema: Extension<FilesSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.0).await.into()
}

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:8000")
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
        .layer(
            CorsLayer::new()
                .allow_origin(Origin::predicate(|_, _| true))
                .allow_methods(vec![Method::GET, Method::POST]),
        );

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
