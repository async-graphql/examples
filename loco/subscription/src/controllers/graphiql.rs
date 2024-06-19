use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_axum::GraphQL;
use axum::debug_handler;
use books::{MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use loco_rs::prelude::*;

#[debug_handler]
async fn graphiql() -> Result<Response> {
    format::html(
        &GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

pub fn routes() -> Routes {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    Routes::new().add("/", get(graphiql).post_service(GraphQL::new(schema)))
}
