use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::debug_handler;
use loco_rs::prelude::*;
use starwars::{QueryRoot, StarWars};

#[debug_handler]
async fn graphiql() -> Result<Response> {
    format::html(&GraphiQLSource::build().endpoint("/").finish())
}

pub fn routes() -> Routes {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    Routes::new().add("/", get(graphiql).post_service(GraphQL::new(schema)))
}
