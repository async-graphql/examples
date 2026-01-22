use std::error::Error;

use async_graphql::{Value, dynamic::*, http::GraphiQLSource};
use async_graphql_poem::GraphQL;
use poem::{IntoResponse, Route, Server, get, handler, listener::TcpListener, web::Html};

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let query =
        Object::new("Query").field(Field::new("value", TypeRef::named_nn(TypeRef::INT), |_| {
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
