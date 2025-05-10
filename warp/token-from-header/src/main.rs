use std::convert::Infallible;

use async_graphql::{Data, EmptyMutation, Schema, http::GraphiQLSource};
use async_graphql_warp::{GraphQLResponse, GraphQLWebSocket, graphql_protocol};
use token::{QueryRoot, SubscriptionRoot, Token, on_connection_init};
use warp::{Filter, http::Response as HttpResponse, ws::Ws};

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, SubscriptionRoot).finish();

    println!("GraphiQL IDE: http://localhost:8000");

    let graphiql = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(
                GraphiQLSource::build()
                    .endpoint("/")
                    .subscription_endpoint("/ws")
                    .finish(),
            )
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

    let routes = subscription.or(graphiql).or(graphql_post);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
