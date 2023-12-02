use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use files::{MutationRoot, QueryRoot, Storage};
use hyper::Method;
use tower_http::cors::{CorsLayer, AllowOrigin};
use tokio::net::TcpListener;

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    let app = Router::new()
        .route("/", get(graphiql).post_service(GraphQL::new(schema)))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(|_, _| true))
                .allow_methods(vec![Method::GET, Method::POST]),
        );

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
