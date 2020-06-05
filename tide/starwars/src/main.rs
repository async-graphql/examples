use async_graphql::http::playground_source;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_std::task;
use starwars::{QueryRoot, StarWars};
use std::env;
use tide::{
    http::{headers, mime},
    Request, Response, StatusCode,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

struct AppState {
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
}

fn main() -> Result<()> {
    task::block_on(run())
}

async fn run() -> Result<()> {
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "localhost:8000".to_owned());

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    println!("Playground: http://{}", listen_addr);

    let app_state = AppState { schema };
    let mut app = tide::with_state(app_state);

    async fn graphql(req: Request<AppState>) -> tide::Result<Response> {
        let schema = req.state().schema.clone();
        async_graphql_tide::graphql(req, schema, |query_builder| query_builder).await
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
                    .body_string(r#"{"query":"{ human(id:\"1000\") {name} }"}"#.to_owned())
                    .set_header("Content-Type", "application/json")
                    .recv_string()
                    .await?;

                assert_eq!(
                    string,
                    json!({"data":{"human":{"name":"Luke Skywalker"}}}).to_string()
                );

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
