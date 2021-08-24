use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_poem::GraphQL;
use poem::web::Html;
use poem::{handler, route, EndpointExt, IntoResponse, Server};
use starwars::{QueryRoot, StarWars};

#[handler(method = "get")]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    let app = route().at("/", graphql_playground.or(GraphQL::new(schema)));

    println!("Playground: http://localhost:8000");

    Server::bind("0.0.0.0:8000")
        .await
        .unwrap()
        .run(app)
        .await
        .unwrap();
}
