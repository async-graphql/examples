use std::convert::Infallible;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Schema,
};
use async_graphql_warp::{graphql_subscription, GraphQLResponse};
use books::{MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use warp::{http::Response as HttpResponse, Filter};

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    println!("Playground: http://localhost:8000");

    let graphql_post = async_graphql_warp::graphql(schema.clone()).and_then(
        |(schema, request): (
            Schema<QueryRoot, MutationRoot, SubscriptionRoot>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
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
