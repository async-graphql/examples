use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::response::{self, IntoResponse};
use axum::routing::get;
use axum::{extract, AddExtensionLayer, Router, Server};
use books::{BooksSchema, MutationRoot, QueryRoot, Storage, SubscriptionRoot};

async fn graphql_handler(
    schema: extract::Extension<BooksSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/ws", GraphQLSubscription::new(schema.clone()))
        .layer(AddExtensionLayer::new(schema));

    println!("Playground: http://localhost:8000");

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
