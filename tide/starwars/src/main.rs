use std::env;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_std::task;
use starwars::{QueryRoot, StarWars};
use tide::{http::mime, Body, Response, StatusCode};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    task::block_on(run())
}

async fn run() -> Result<()> {
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "localhost:8000".to_owned());

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    println!("Playground: http://{}", listen_addr);

    let mut app = tide::new();

    app.at("/graphql").post(async_graphql_tide::graphql(schema));

    app.at("/").get(|_| async move {
        let mut resp = Response::new(StatusCode::Ok);
        resp.set_body(Body::from_string(
            GraphiQLSource::build().endpoint("/graphql").finish(),
        ));
        resp.set_content_type(mime::HTML);
        Ok(resp)
    });

    app.listen(listen_addr).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use async_std::prelude::*;
    use serde_json::json;

    use super::*;

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
                    .body(Body::from(r#"{"query":"{ human(id:\"1000\") {name} }"}"#))
                    .header("Content-Type", "application/json")
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
