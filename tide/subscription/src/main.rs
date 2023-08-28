use async_graphql::{http::GraphiQLSource, Schema};
use async_std::task;
use books::{MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use tide::{http::mime, Body, Response, StatusCode};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    task::block_on(run())
}

async fn run() -> Result<()> {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    let mut app = tide::new();

    app.at("/graphql")
        .post(async_graphql_tide::graphql(schema.clone()))
        .get(async_graphql_tide::GraphQLSubscription::new(schema).build());

    app.at("/").get(|_| async move {
        let mut resp = Response::new(StatusCode::Ok);
        resp.set_body(Body::from_string(
            GraphiQLSource::build()
                .endpoint("/graphql")
                .subscription_endpoint("/graphql")
                .finish(),
        ));
        resp.set_content_type(mime::HTML);
        Ok(resp)
    });

    app.listen("0.0.0.0:8000").await?;

    Ok(())
}
