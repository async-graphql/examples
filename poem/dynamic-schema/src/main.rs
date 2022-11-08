use std::error::Error;

use async_graphql::{
    dynamic::*,
    http::{playground_source, GraphQLPlaygroundConfig},
    Value,
};
use async_graphql_poem::GraphQL;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let query = Object::new("Query").field(Field::new("value", TypeRef::INT, |_| {
        FieldFuture::new(async { Ok(Some(Value::from(100))) })
    }));
    let schema = Schema::build(query.type_name(), None, None)
        .register(query)
        .finish()?;

    let app = Route::new().at("/", get(graphql_playground).post(GraphQL::new(schema)));

    println!("Playground: http://localhost:8000");
    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await?;
    Ok(())
}
