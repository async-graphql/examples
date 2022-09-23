use async_graphql::{
    http::{GraphiQLSource, ALL_WEBSOCKET_PROTOCOLS},
    EmptyMutation, Schema,
};
use async_graphql_poem::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use poem::{
    get, handler,
    http::HeaderMap,
    listener::TcpListener,
    web::{websocket::WebSocket, Data, Html},
    EndpointExt, IntoResponse, Route, Server,
};
use token::{on_connection_init, QueryRoot, SubscriptionRoot, Token, TokenSchema};

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("Token")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
}

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:8000")
            .subscription_endpoint("ws://localhost:8000/ws")
            .finish(),
    )
}

#[handler]
async fn index(
    schema: Data<&TokenSchema>,
    headers: &HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.0;
    if let Some(token) = get_token_from_headers(headers) {
        req = req.data(token);
    }
    schema.execute(req).await.into()
}

#[handler]
async fn ws(
    schema: Data<&TokenSchema>,
    headers: &HeaderMap,
    protocol: GraphQLProtocol,
    websocket: WebSocket,
) -> impl IntoResponse {
    let schema = schema.0.clone();
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |stream| {
            GraphQLWebSocket::new(stream, schema, protocol)
                .on_connection_init(on_connection_init)
                .serve()
        })
}

#[tokio::main]
async fn main() {
    let schema = Schema::new(QueryRoot, EmptyMutation, SubscriptionRoot);

    let app = Route::new()
        .at("/", get(graphiql).post(index))
        .at("/ws", get(ws))
        .data(schema);

    println!("GraphiQL IDE: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
        .unwrap();
}
