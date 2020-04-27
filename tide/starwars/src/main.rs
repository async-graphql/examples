use async_graphql::http::playground_source;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use starwars::{QueryRoot, StarWars};
use tide::{Request, Response, StatusCode};

struct AppState {
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    println!("Playground: http://localhost:8000");

    let app_state = AppState { schema };
    let mut app = tide::with_state(app_state);

    app.at("/").post(|req: Request<AppState>| async move {
        let schema = req.state().schema.clone();
        async_graphql_tide::graphql(req, schema, |query_builder| query_builder).await
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
