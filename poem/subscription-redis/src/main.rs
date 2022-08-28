use async_graphql::{http::GraphiQLSource, Context, Object, Result, Schema, Subscription};
use async_graphql_poem::{GraphQL, GraphQLSubscription};
use futures_util::{Stream, StreamExt};
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};
use redis::{AsyncCommands, Client};

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn version(&self) -> &'static str {
        std::env!("CARGO_PKG_VERSION")
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn publish(&self, ctx: &Context<'_>, value: String) -> Result<bool> {
        let client = ctx.data_unchecked::<Client>();
        let mut conn = client.get_async_connection().await?;
        conn.publish("values", value).await?;
        Ok(true)
    }
}

struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn values(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let client = ctx.data_unchecked::<Client>();
        let mut conn = client.get_async_connection().await?.into_pubsub();
        conn.subscribe("values").await?;
        Ok(conn
            .into_on_message()
            .filter_map(|msg| async move { msg.get_payload().ok() }))
    }
}

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:8000")
            .subscription_endpoint("ws://localhost:8000/ws")
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    let client = Client::open("redis://127.0.0.1/").unwrap();

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(client)
        .finish();

    let app = Route::new()
        .at(
            "/",
            get(graphql_playground).post(GraphQL::new(schema.clone())),
        )
        .at("/ws", get(GraphQLSubscription::new(schema)));

    println!("Playground: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
        .unwrap();
}
