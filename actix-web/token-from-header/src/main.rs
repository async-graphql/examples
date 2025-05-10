use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Result, guard, http::header::HeaderMap, web,
};
use async_graphql::{EmptyMutation, Schema, http::GraphiQLSource};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use token::{QueryRoot, SubscriptionRoot, Token, TokenSchema, on_connection_init};

async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("/")
                .subscription_endpoint("/ws")
                .finish(),
        )
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("Token")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
}

async fn index(
    schema: web::Data<TokenSchema>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_request.into_inner();
    if let Some(token) = get_token_from_headers(req.headers()) {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

async fn index_ws(
    schema: web::Data<TokenSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema))
        .on_connection_init(on_connection_init)
        .start(&req, payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::new(QueryRoot, EmptyMutation, SubscriptionRoot);

    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Get()).to(graphiql))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/ws").to(index_ws))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
