use async_graphql::{EmptyMutation, EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use files::{FilesSchema, MutationRoot, QueryRoot, Storage};
use rocket::{State, response::content, routes};

pub type StarWarsSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(schema: &State<FilesSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json", rank = 1)]
async fn graphql_request(schema: &State<FilesSchema>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

#[rocket::post(
    "/graphql",
    data = "<request>",
    format = "multipart/form-data",
    rank = 2
)]
async fn graphql_request_multipart(
    schema: &State<FilesSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

#[rocket::launch]
fn rocket() -> _ {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        .finish();

    rocket::build().manage(schema).mount(
        "/",
        routes![
            graphql_query,
            graphql_request,
            graphql_request_multipart,
            graphiql
        ],
    )
}
