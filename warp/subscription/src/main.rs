use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{BatchQueryDefinition, Schema};
use async_graphql_warp::{graphql_subscription, BatchGQLResponse};
use books::{MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use std::convert::Infallible;
use warp::{http::Response, Filter};

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    println!("Playground: http://localhost:8000");

    let graphql_post = async_graphql_warp::graphql(schema.clone()).and_then(
        |(schema, definition): (_, BatchQueryDefinition)| async move {
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

    let routes = graphql_subscription(schema)
        .or(graphql_playground)
        .or(graphql_post);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
