use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_poem::{GraphQL, GraphQLSubscription};
use books::{MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:8000")
            .subscription_endpoint("ws://localhost:8000/ws")
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
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
