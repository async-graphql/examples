use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use files::{FilesSchema, MutationRoot, QueryRoot, Storage};
use poem::listener::TcpListener;
use poem::middleware::Cors;
use poem::web::{Data, Html};
use poem::{get, handler, EndpointExt, IntoResponse, Route, Server};

#[handler]
async fn index(schema: Data<&FilesSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.0).await.into()
}

#[handler]
async fn gql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        .finish();

    println!("Playground: http://localhost:8000");

    let app = Route::new()
        .at("/", get(gql_playground).post(index))
        .with(Cors::new())
        .data(schema);
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
}
