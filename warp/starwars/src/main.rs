use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::{BadRequest, Response};
use http::StatusCode;
use starwars::{QueryRoot, StarWars};
use std::convert::Infallible;
use tokio_compat_02::FutureExt;
use warp::{http::Response as HttpResponse, Filter, Rejection};

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    println!("Playground: http://localhost:8000");

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<QueryRoot, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move { Ok::<_, Infallible>(Response::from(schema.execute(request).await)) },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let routes = graphql_playground
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(BadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).compat().await;
}
