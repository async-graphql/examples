use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Extension;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{AddExtensionLayer, Router};
use files::{FilesSchema, MutationRoot, QueryRoot, Storage};
use hyper::{Method, Server};
use tower_http::cors::{CorsLayer, Origin};

async fn graphql_handler(schema: Extension<FilesSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.0).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        .finish();

    println!("Playground: http://localhost:8000");

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(AddExtensionLayer::new(schema))
        .layer(
            CorsLayer::new()
                .allow_origin(Origin::predicate(|_, _| true))
                .allow_methods(vec![Method::GET, Method::POST]),
        );

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
