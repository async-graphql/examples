use async_graphql::{Context, Object, Result, Schema, Subscription, http::GraphiQLSource};
use async_graphql_poem::{GraphQL, GraphQLSubscription};
use futures_util::{Stream, StreamExt};
use poem::{IntoResponse, Route, Server, get, handler, listener::TcpListener, web::Html};
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
        let mut conn = client.get_multiplexed_async_connection().await?;
        conn.publish::<_, _, ()>("values", value).await?;
        Ok(true)
    }
}

struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn values(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        let client = ctx.data_unchecked::<Client>();
        let mut conn = client.get_async_pubsub().await?;
        conn.subscribe("values").await?;
        Ok(conn
            .into_on_message()
            .filter_map(|msg| async move { msg.get_payload().ok() }))
    }
}

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
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
        .at("/", get(graphiql).post(GraphQL::new(schema.clone())))
        .at("/ws", get(GraphQLSubscription::new(schema)));

    println!("GraphiQL IDE: http://localhost:8000");
    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await
        .unwrap();
}
