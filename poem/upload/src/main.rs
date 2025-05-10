use async_graphql::{EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use files::{FilesSchema, MutationRoot, QueryRoot, Storage};
use poem::{
    EndpointExt, IntoResponse, Route, Server, get, handler,
    listener::TcpListener,
    middleware::Cors,
    web::{Data, Html},
};

#[handler]
async fn index(schema: Data<&FilesSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.0).await.into()
}

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
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
