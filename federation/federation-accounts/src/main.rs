use async_graphql::http::GQLResponse;
use async_graphql::{
    EmptyMutation, EmptySubscription, Object, QueryBuilder, Schema, SimpleObject, ID,
};
use async_graphql_warp::graphql;
use std::convert::Infallible;
use warp::{Filter, Reply};

#[SimpleObject]
struct User {
    #[field]
    id: ID,

    #[field]
    username: String,
}

struct Query;

#[Object(extends)]
impl Query {
    #[field]
    async fn me(&self) -> User {
        User {
            id: "1234".into(),
            username: "Me".to_string(),
        }
    }

    #[entity]
    async fn find_user_by_id(&self, id: ID) -> User {
        let username = if id == "1234" {
            "Me".to_string()
        } else {
            format!("User {}", id)
        };
        User { id, username }
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

    warp::serve(
        graphql(schema).and_then(|(schema, builder): (_, QueryBuilder)| async move {
            let resp = builder.execute(&schema).await;
            Ok::<_, Infallible>(warp::reply::json(&GQLResponse(resp)).into_response())
        }),
    )
    .run(([0, 0, 0, 0], 4001))
    .await;
}
