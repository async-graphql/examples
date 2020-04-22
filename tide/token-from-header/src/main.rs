use async_graphql::http::{playground_source, GQLRequest, GQLResponse};
use async_graphql::{Context, EmptyMutation, EmptySubscription, IntoQueryBuilder, Schema};
use tide::{http_types, IntoResponse, Request, Response};

struct MyToken(Option<String>);

struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    #[field]
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data::<MyToken>().0.as_deref()
    }
}

struct ServerState {
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
}

async fn graphql_post(mut req: Request<ServerState>) -> Result<Response, http_types::Error> {
    let gql_request: GQLRequest = req
        .body_json()
        .await
        .map_err(|e| http_types::Error::new(http_types::StatusCode::BadRequest, e))?;

    let query_builder = gql_request
        .into_query_builder()
        .await
        .map_err(|e| http_types::Error::new(http_types::StatusCode::BadRequest, e))?;

    let schema = &req.state().schema;

    let token = req
        .header(&"token".parse().unwrap())
        .map(|values| values.first().map(|value| value.as_str().to_string()))
        .unwrap_or(Some("".to_string()));

    let query_response = query_builder.data(MyToken(token)).execute(&schema).await;

    let gql_response = GQLResponse(query_response);

    let resp = Response::new(http_types::StatusCode::Ok).body_json(&gql_response)?;

    Ok(resp)
}

async fn graphql_playground(_: Request<ServerState>) -> Response {
    Response::new(http_types::StatusCode::Ok)
        .body_string(playground_source("/", None))
        .set_header("content-type".parse().unwrap(), "text/html")
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    println!("Playground: http://localhost:8000");

    let server_state = ServerState { schema };
    let mut app = tide::with_state(server_state);

    app.at("/").post(|req| async move {
        graphql_post(req)
            .await
            .unwrap_or_else(|e| e.into_response())
    });
    app.at("/").get(graphql_playground);

    app.listen("0.0.0.0:8000").await?;
    Ok(())
}
