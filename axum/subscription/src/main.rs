use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, ALL_WEBSOCKET_PROTOCOLS};
use async_graphql::Schema;
use async_graphql_axum::{
    graphql_subscription, GraphQLRequest, GraphQLResponse, SecWebsocketProtocol,
};
use axum::extract::TypedHeader;
use axum::response::IntoResponse;
use axum::ws::{ws, WebSocket};
use axum::{prelude::*, AddExtensionLayer};
use books::{BooksSchema, MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use hyper::http::HeaderValue;

async fn graphql_handler(
    schema: extract::Extension<BooksSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_subscription_handler(
    socket: WebSocket,
    schema: extract::Extension<BooksSchema>,
    protocol: TypedHeader<SecWebsocketProtocol>,
) {
    graphql_subscription(socket, schema.0.clone(), protocol.0).await
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    let app = route("/", get(graphql_playground).post(graphql_handler))
        .route(
            "/ws",
            ws(graphql_subscription_handler).protocols(ALL_WEBSOCKET_PROTOCOLS),
        )
        .layer(AddExtensionLayer::new(schema));

    println!("Playground: http://localhost:8000");

    hyper::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
