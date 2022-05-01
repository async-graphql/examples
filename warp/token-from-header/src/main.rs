use std::convert::Infallible;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Data, EmptyMutation, Schema,
};
use async_graphql_warp::{graphql_protocol, GraphQLResponse, GraphQLWebSocket};
use token::{on_connection_init, QueryRoot, SubscriptionRoot, Token};
use warp::{http::Response as HttpResponse, ws::Ws, Filter};

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, SubscriptionRoot).finish();

    println!("Playground: http://localhost:8000");

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
            ))
    });

    let graphql_post = warp::header::optional::<String>("token")
        .and(async_graphql_warp::graphql(schema.clone()))
        .and_then(
            |token,
             (schema, mut request): (
                Schema<QueryRoot, EmptyMutation, SubscriptionRoot>,
                async_graphql::Request,
            )| async move {
                if let Some(token) = token {
                    request = request.data(Token(token));
                }
                let resp = schema.execute(request).await;
                Ok::<_, Infallible>(GraphQLResponse::from(resp))
            },
        );

    let subscription = warp::path!("ws")
        .and(warp::ws())
        .and(warp::header::optional::<String>("token"))
        .and(warp::any().map(move || schema.clone()))
        .and(graphql_protocol())
        .map(
            move |ws: Ws,
                  token,
                  schema: Schema<QueryRoot, EmptyMutation, SubscriptionRoot>,
                  protocol| {
                let reply = ws.on_upgrade(move |socket| {
                    let mut data = Data::default();
                    if let Some(token) = token {
                        data.insert(Token(token));
                    }

                    GraphQLWebSocket::new(socket, schema, protocol)
                        .with_data(data)
                        .on_connection_init(on_connection_init)
                        .serve()
                });

                warp::reply::with_header(
                    reply,
                    "Sec-WebSocket-Protocol",
                    protocol.sec_websocket_protocol(),
                )
            },
        );

    let routes = subscription.or(graphql_playground).or(graphql_post);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
