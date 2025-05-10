use async_graphql::http::GraphiQLSource;
use async_graphql_poem::GraphQL;
use poem::{IntoResponse, Route, Server, get, handler, listener::TcpListener, web::Html};

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let app = Route::new().at(
        "/",
        get(graphiql).post(GraphQL::new(dynamic_files::schema().unwrap())),
    );

    println!("GraphiQL IDE: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
        .unwrap();
}
