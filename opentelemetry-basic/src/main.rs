use async_graphql::{extensions::OpenTelemetry, EmptyMutation, EmptySubscription, Schema};
use async_graphql_poem::{GraphQLProtocol, GraphQLRequest, GraphQLResponse};
use opentelemetry::sdk::export::trace::stdout;
use poem::{
    handler, http::HeaderMap, listener::TcpListener, web::Data, EndpointExt, IntoResponse, Route,
    Server,
};

#[handler]
async fn index(
    schema: Data<&TokenSchema>,
    headers: &HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req).await.into()
}

#[tokio::main]
async fn main() {
    let tracer = stdout::new_pipeline().install_simple();
    let opentelemetry_extension = OpenTelemetry::new(tracer);

    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);

    let app = Route::new().at("/", post(index)).data(schema);

    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
        .unwrap();
}
