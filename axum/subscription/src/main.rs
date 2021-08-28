use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, ALL_WEBSOCKET_PROTOCOLS};
use async_graphql::Schema;
use async_graphql_axum::{
    graphql_subscription, GraphQLRequest, GraphQLResponse, SecWebsocketProtocol,
};
use axum::extract::{self, ws::WebSocketUpgrade, TypedHeader};
use axum::handler::get;
use axum::response::{self, IntoResponse};
use axum::{AddExtensionLayer, Router, Server};
use books::{BooksSchema, MutationRoot, QueryRoot, Storage, SubscriptionRoot};

async fn graphql_handler(
    schema: extract::Extension<BooksSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_subscription_handler(
    ws: WebSocketUpgrade,
    schema: extract::Extension<BooksSchema>,
    protocol: TypedHeader<SecWebsocketProtocol>,
) -> impl IntoResponse {
    ws.protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |socket| async move {
            graphql_subscription(socket, schema.0.clone(), protocol.0.clone()).await
        })
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/ws", get(graphql_subscription_handler))
        .layer(AddExtensionLayer::new(schema));

    println!("Playground: http://localhost:8000");

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
