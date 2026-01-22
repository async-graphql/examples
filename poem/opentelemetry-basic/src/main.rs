use async_graphql::{
    EmptyMutation, EmptySubscription, Object, Result, Schema, 
};
use async_graphql_extras::extensions::OpenTelemetry;
use async_graphql_poem::GraphQL;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::TracerProvider;
use poem::{EndpointExt, Route, Server, listener::TcpListener, post};

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> Result<String> {
        Ok("World".to_string())
    }
}

#[tokio::main]
async fn main() {
    let provider = TracerProvider::builder()
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .build();
    let tracer = provider.tracer("poem-opentelemetry-basic");
    let opentelemetry_extension = OpenTelemetry::new(tracer);

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .extension(opentelemetry_extension)
        .finish();

    let app = Route::new()
        .at("/", post(GraphQL::new(schema.clone())))
        .data(schema);

    let example_curl = "\
    curl '127.0.0.1:8000' \
    -X POST \
    -H 'content-type: application/json' \
    --data '{ \"query\": \"{ hello }\" }'";

    println!(
        "Run this curl command from another terminal window to see opentelemetry output in this terminal.\n\n{example_curl}\n\n"
    );

    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await
        .unwrap();
}
