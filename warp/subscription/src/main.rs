use std::convert::Infallible;

use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_warp::{graphql_subscription, GraphQLResponse};
use books::{MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use warp::{http::Response as HttpResponse, Filter};

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    let graphql_post = async_graphql_warp::graphql(schema.clone()).and_then(
        |(schema, request): (
            Schema<QueryRoot, MutationRoot, SubscriptionRoot>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );

    let graphiql = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(
                GraphiQLSource::build()
                    .endpoint("http://localhost:8000")
                    .subscription_endpoint("ws://localhost:8000")
                    .finish(),
            )
    });

    let routes = graphql_subscription(schema).or(graphiql).or(graphql_post);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
