use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, Data, EmptyMutation, Object, Schema, Subscription};
use async_graphql_warp::{graphql_subscription_with_data, Response};
use futures::{stream, Stream};
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter};

struct MyToken(String);

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<MyToken>().map(|token| token.0.as_str())
    }
}

struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn values(&self, ctx: &Context<'_>) -> async_graphql::Result<impl Stream<Item = i32>> {
        if ctx.data_unchecked::<MyToken>().0 != "123456" {
            return Err("Forbidden".into());
        }
        Ok(stream::once(async move { 10 }))
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, SubscriptionRoot).finish();

    println!("Playground: http://localhost:8000");

    let graphql_post = warp::header::optional::<String>("token")
        .and(async_graphql_warp::graphql(schema.clone()))
        .and_then(
            |token,
             (schema, mut request): (
                Schema<QueryRoot, EmptyMutation, SubscriptionRoot>,
                async_graphql::Request,
            )| async move {
                if let Some(token) = token {
                    request = request.data(MyToken(token));
                }
                let resp = schema.execute(request).await;
                Ok::<_, Infallible>(Response::from(resp))
            },
        );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
            ))
    });

    let routes = graphql_subscription_with_data(
        schema,
        Some(|value| {
            #[derive(serde_derive::Deserialize)]
            struct Payload {
                token: String,
            }

            if let Ok(payload) = serde_json::from_value::<Payload>(value) {
                let mut data = Data::default();
                data.insert(MyToken(payload.token));
                Ok(data)
            } else {
                Err("Token is required".into())
            }
        }),
    )
    .or(graphql_playground)
    .or(graphql_post);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
