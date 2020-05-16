#![allow(clippy::needless_lifetimes)]

use async_graphql::http::playground_source;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema};
use async_std::task;
use std::env;
use tide::{
    http::{headers, mime},
    Request, Response, StatusCode,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

struct MyToken(String);

struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<MyToken>().map(|token| token.0.as_str())
    }
}

struct AppState {
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
}

fn main() -> Result<()> {
    task::block_on(run())
}

async fn run() -> Result<()> {
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "localhost:8000".to_owned());

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    println!("Playground: http://{}", listen_addr);

    let app_state = AppState { schema };
    let mut app = tide::with_state(app_state);

    async fn graphql(req: Request<AppState>) -> tide::Result<Response> {
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
    }

    app.at("/graphql").post(graphql).get(graphql);
    app.at("/").get(|_| async move {
        let resp = Response::new(StatusCode::Ok)
            .body_string(playground_source("/graphql", None))
            .set_header(headers::CONTENT_TYPE, mime::HTML.to_string());

        Ok(resp)
    });

    app.listen(listen_addr).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::prelude::*;
    use serde_json::json;
    use std::time::Duration;

    #[test]
    fn sample() -> Result<()> {
        task::block_on(async {
            let listen_addr = find_listen_addr().await;
            env::set_var("LISTEN_ADDR", format!("{}", listen_addr));

            let server: task::JoinHandle<Result<()>> = task::spawn(async move {
                run().await?;

                Ok(())
            });

            let client: task::JoinHandle<Result<()>> = task::spawn(async move {
                let listen_addr = env::var("LISTEN_ADDR").unwrap();

                task::sleep(Duration::from_millis(300)).await;

                let string = surf::post(format!("http://{}/graphql", listen_addr))
                    .body_bytes(r#"{"query":"{ currentToken }"}"#)
                    .set_header("Content-Type".parse().unwrap(), "application/json")
                    .set_header("Token".parse().unwrap(), "1234")
                    .recv_string()
                    .await?;

                assert_eq!(string, json!({"data":{"currentToken":"1234"}}).to_string());

                let string = surf::post(format!("http://{}/graphql", listen_addr))
                    .body_bytes(r#"{"query":"{ currentToken }"}"#)
                    .set_header("Content-Type".parse().unwrap(), "application/json")
                    .recv_string()
                    .await?;

                assert_eq!(string, json!({"data":{"currentToken":null}}).to_string());

                Ok(())
            });

            server.race(client).await?;

            Ok(())
        })
    }

    async fn find_listen_addr() -> async_std::net::SocketAddr {
        async_std::net::TcpListener::bind("localhost:0")
            .await
            .unwrap()
            .local_addr()
            .unwrap()
    }
}
