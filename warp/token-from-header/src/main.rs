use async_graphql::http::{playground_source, GQLResponse};
use async_graphql::{Context, EmptyMutation, EmptySubscription, QueryBuilder, Schema};
use std::convert::Infallible;
use warp::{http::Response, Filter, Reply};

struct MyToken(Option<String>);

struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    #[field]
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data::<MyToken>().0.as_deref()
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    println!("Playground: http://localhost:8000");

    let graphql_post = warp::header::optional::<String>("token")
        .and(async_graphql_warp::graphql(schema))
        .and_then(|token, (schema, builder): (_, QueryBuilder)| async move {
            let resp = builder.data(MyToken(token)).execute(&schema).await;
            Ok::<_, Infallible>(warp::reply::json(&GQLResponse(resp)).into_response())
        });

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source("/", None))
    });

    let routes = graphql_post.or(graphql_playground);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
