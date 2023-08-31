use async_graphql::http::GraphiQLSource;
use async_graphql_poem::{GraphQL, GraphQLSubscription};
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};

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
    let schema = dynamic_books::schema().unwrap();
    let app = Route::new()
        .at("/", get(graphiql).post(GraphQL::new(schema.clone())))
        .at("/ws", get(GraphQLSubscription::new(schema)));

    println!("GraphiQL IDE: http://localhost:8080");
    Server::new(TcpListener::bind("127.0.0.1:8080"))
        .run(app)
        .await
        .unwrap();
}
