use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use files::{FilesSchema, MutationRoot, QueryRoot, Storage};
use poem::{
    get, handler,
    listener::TcpListener,
    middleware::Cors,
    web::{Data, Html},
    EndpointExt, IntoResponse, Route, Server,
};

#[handler]
async fn index(schema: Data<&FilesSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.0).await.into()
}

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:8000")
            .finish(),
    )
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    let app = Route::new()
        .at("/", get(graphiql).post(index))
        .with(Cors::new())
        .data(schema);
    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await
}
