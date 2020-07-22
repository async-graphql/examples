#![allow(clippy::needless_lifetimes)]

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, Data, EmptyMutation, FieldResult, Schema, BatchQueryDefinition};
use async_graphql_warp::{graphql_subscription_with_data, BatchGQLResponse};
use futures::{stream, Stream};
use std::convert::Infallible;
use warp::{http::Response, Filter};

struct MyToken(String);

struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<MyToken>().map(|token| token.0.as_str())
    }
}

struct SubscriptionRoot;

#[async_graphql::Subscription]
impl SubscriptionRoot {
    async fn values(&self, ctx: &Context<'_>) -> FieldResult<impl Stream<Item = i32>> {
        if ctx.data::<MyToken>().0 != "123456" {
            return Err("Forbidden".into());
        }
        Ok(stream::once(async move { 10 }))
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, SubscriptionRoot).finish();

    println!("Playground: http://localhost:8000");

    let graphql_post = warp::header::optional::<String>("token")
        .and(async_graphql_warp::graphql(schema.clone()))
        .and_then(
            |token, (schema, mut definition): (_, BatchQueryDefinition)| async move {
                if let Some(token) = token {
                    definition = definition.data(MyToken(token));
                }
                let resp = definition.execute(&schema).await;
                Ok::<_, Infallible>(BatchGQLResponse::from(resp))
            },
        );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
            ))
    });

    let routes = graphql_playground
        .or(graphql_subscription_with_data(schema, |value| {
            #[derive(serde_derive::Deserialize)]
            struct Payload {
                token: String,
            }

            if let Ok(payload) = serde_json::from_value::<Payload>(value) {
                let mut data = Data::default();
                data.insert(MyToken(payload.token));
                Ok(data)
            } else {
                Err("Token is required".into())
            }
        }))
        .or(graphql_post);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
