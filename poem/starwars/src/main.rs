use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_poem::GraphQL;
use poem::route::RouteMethod;
use poem::web::Html;
use poem::{handler, route, IntoResponse, Server};
use starwars::{QueryRoot, StarWars};

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    let app = route().at(
        "/",
        RouteMethod::new()
            .get(graphql_playground)
            .post(GraphQL::new(schema)),
    );

    println!("Playground: http://localhost:8000");

    Server::bind("0.0.0.0:8000")
        .await
        .unwrap()
        .run(app)
        .await
        .unwrap();
}
