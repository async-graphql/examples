use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_poem::{GraphQL, GraphQLSubscription};
use books::{MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use poem::listener::TcpListener;
use poem::web::Html;
use poem::{get, handler, IntoResponse, Route, Server};

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

    let listener = TcpListener::bind("0.0.0.0:8000");
    Server::new(listener).await.unwrap().run(app).await.unwrap();
}
