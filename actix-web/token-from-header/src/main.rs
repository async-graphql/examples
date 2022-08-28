use actix_web::{
    guard, http::header::HeaderMap, web, App, HttpRequest, HttpResponse, HttpServer, Result,
};
use async_graphql::{http::GraphiQLSource, Data, EmptyMutation, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use token::{on_connection_init, QueryRoot, SubscriptionRoot, Token, TokenSchema};

async fn gql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("http://localhost:8000")
                .subscription_endpoint("ws://localhost:8000/ws")
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
    let mut data = Data::default();
    if let Some(token) = get_token_from_headers(req.headers()) {
        data.insert(token);
    }

    GraphQLSubscription::new(Schema::clone(&*schema))
        .with_data(data)
        .on_connection_init(on_connection_init)
        .start(&req, payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::new(QueryRoot, EmptyMutation, SubscriptionRoot);

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Get()).to(gql_playground))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/ws").to(index_ws))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
