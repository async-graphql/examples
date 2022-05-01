use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Schema,
};
use async_graphql_poem::{GraphQL, GraphQLSubscription};
use books::{MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    let app = Route::new()
        .at(
            "/",
            get(graphql_playground).post(GraphQL::new(schema.clone())),
        )
        .at("/ws", get(GraphQLSubscription::new(schema)));

    println!("Playground: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
        .unwrap();
}
