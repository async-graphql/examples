#![allow(clippy::needless_lifetimes)]

use async_graphql::http::playground_source;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema};
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

struct AppState {
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    println!("Playground: http://localhost:8000");

    let app_state = AppState { schema };
    let mut app = tide::with_state(app_state);

    app.at("/").post(|req: Request<AppState>| async move {
        let schema = req.state().schema.clone();
        let token = &req
            .header(&"token".parse().unwrap())
            .and_then(|values| values.first().map(|value| value.to_string()));

        async_graphql_tide::graphql(req, schema, |mut query_builder| {
            if let Some(token) = token {
                query_builder = query_builder.data(MyToken(token.to_string()));
            }
            query_builder
        })
        .await
    });
    app.at("/").get(|_| async move {
        let resp = Response::new(StatusCode::Ok)
            .body_string(playground_source("/", None))
            .set_header("content-type".parse().unwrap(), "text/html");

        Ok(resp)
    });

    app.listen("0.0.0.0:8000").await?;
    Ok(())
}
