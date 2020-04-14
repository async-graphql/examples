use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GQLResponse};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::GQLRequest;

type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

struct MyToken(Option<String>);

struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    #[field]
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data::<MyToken>().0.as_deref()
    }
}

async fn index(
    schema: web::Data<MySchema>,
    req: HttpRequest,
    gql_request: GQLRequest,
) -> web::Json<GQLResponse> {
    let token = MyToken(
        req.headers()
            .get("Token")
            .and_then(|value| value.to_str().map(ToString::to_string).ok()),
    );

    web::Json(GQLResponse(
        gql_request.into_inner().data(token).execute(&schema).await,
    ))
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/", None))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
