use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_rocket::{GraphQL, Request, Response};
use rocket::{http::Status, response::content, routes, State};
use starwars::{QueryRoot, StarWars};

pub type StarWarsSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[rocket::get("/")]
fn graphql_playground() -> content::Html<String> {
    content::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[rocket::post("/?<query..>")]
async fn graphql_query(
    schema: State<'_, StarWarsSchema>,
    query: Request,
) -> Result<Response, Status> {
    query.execute(&schema).await
}

#[rocket::post("/", data = "<request>", format = "application/json")]
async fn graphql_request(
    schema: State<'_, StarWarsSchema>,
    request: Request,
) -> Result<Response, Status> {
    request.execute(&schema).await
}

#[rocket::launch]
fn rocket() -> rocket::Rocket {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    rocket::ignite().attach(GraphQL::fairing(schema)).mount(
        "/",
        routes![graphql_query, graphql_request, graphql_playground],
    )
}
