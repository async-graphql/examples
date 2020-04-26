#![allow(clippy::needless_lifetimes)]

use async_graphql::http::{playground_source, GQLRequest, GQLResponse};
use async_graphql::{Context, EmptyMutation, EmptySubscription, IntoQueryBuilder, Schema};
use tide::{Request, Response, StatusCode};

struct MyToken(String);

struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    #[field]
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<MyToken>().map(|token| token.0.as_str())
    }
}

struct ServerState {
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
}

async fn graphql_post(mut req: Request<ServerState>) -> tide::Result<Response> {
    let gql_request: GQLRequest = req
        .body_json()
        .await
        .map_err(|e| tide::Error::new(StatusCode::BadRequest, e))?;

    let mut query_builder = gql_request
        .into_query_builder()
        .await
        .map_err(|e| tide::Error::new(StatusCode::BadRequest, e))?;

    let schema = &req.state().schema;

    if let Some(token) = req
        .header(&"token".parse().unwrap())
        .and_then(|values| values.first().map(|value| value.to_string()))
    {
        query_builder = query_builder.data(MyToken(token));
    }

    let query_response = query_builder.execute(&schema).await;

    let gql_response = GQLResponse(query_response);

    let resp = Response::new(StatusCode::Ok).body_json(&gql_response)?;

    Ok(resp)
}

async fn graphql_playground(_: Request<ServerState>) -> tide::Result<Response> {
    let resp = Response::new(StatusCode::Ok)
        .body_string(playground_source("/", None))
        .set_header("content-type".parse().unwrap(), "text/html");

    Ok(resp)
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    println!("Playground: http://localhost:8000");

    let server_state = ServerState { schema };
    let mut app = tide::with_state(server_state);

    app.at("/")
        .post(|req| async move { graphql_post(req).await });
    app.at("/").get(graphql_playground);

    app.listen("0.0.0.0:8000").await?;
    Ok(())
}
