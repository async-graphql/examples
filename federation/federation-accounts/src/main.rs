use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, ID};
use async_graphql_warp::graphql;
use std::convert::Infallible;
use warp::{Filter, Reply};

#[SimpleObject]
struct User {
    id: ID,
    username: String,
}

struct Query;

#[Object(extends)]
impl Query {
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
            format!("User {:?}", id)
        };
        User { id, username }
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

    warp::serve(graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(warp::reply::json(&schema.execute(request).await).into_response())
        },
    ))
    .run(([0, 0, 0, 0], 4001))
    .await;
}
