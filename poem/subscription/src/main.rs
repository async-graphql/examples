use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_poem::{GraphQL, GraphQLSubscription};
use books::{MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use poem::web::Html;
use poem::{handler, route, EndpointExt, IntoResponse, Server};

#[handler(method = "get")]
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

    let app = route()
        .at("/", graphql_playground.or(GraphQL::new(schema.clone())))
        .at("/ws", GraphQLSubscription::new(schema));

    println!("Playground: http://localhost:8000");

    Server::bind("0.0.0.0:8000")
        .await
        .unwrap()
        .run(app)
        .await
        .unwrap();
}
