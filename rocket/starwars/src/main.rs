use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_rocket::{Query, Request, Response};
use rocket::{response::content, routes, State};
use starwars::{QueryRoot, StarWars};

pub type StarWarsSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[rocket::get("/")]
fn graphql_playground() -> content::Html<String> {
    content::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(schema: &State<StarWarsSchema>, query: Query) -> Response {
    query.execute(schema).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<StarWarsSchema>, request: Request) -> Response {
    request.execute(schema).await
}

#[rocket::launch]
fn rocket() -> _ {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    rocket::build().manage(schema).mount(
        "/",
        routes![graphql_query, graphql_request, graphql_playground],
    )
}
