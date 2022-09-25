use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, ID};
use async_graphql_poem::GraphQL;
use poem::{listener::TcpListener, Route, Server};

#[derive(SimpleObject)]
struct User {
    id: ID,
    username: String,
    profile_picture: Option<Picture>,
    /// This used to be part of this subgraph, but is now being overridden from
    /// `reviews`
    review_count: u32,
    joined_timestamp: u64,
}

impl User {
    fn me() -> User {
        User {
            id: "1234".into(),
            username: "Me".to_string(),
            profile_picture: Some(Picture {
                url: "http://localhost:8080/me.jpg".to_string(),
                width: 256,
                height: 256,
            }),
            review_count: 0,
            joined_timestamp: 1,
        }
    }
}

#[derive(SimpleObject)]
#[graphql(shareable)]
struct Picture {
    url: String,
    width: u32,
    height: u32,
}

struct Query;

#[Object]
impl Query {
    async fn me(&self) -> User {
        User::me()
    }

    #[graphql(entity)]
    async fn find_user_by_id(&self, id: ID) -> User {
        if id == "1234" {
            User::me()
        } else {
            let username = format!("User {}", id.as_str());
            User {
                id,
                username,
                profile_picture: None,
                review_count: 0,
                joined_timestamp: 1500,
            }
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    Server::new(TcpListener::bind("0.0.0.0:4001"))
        .run(Route::new().at("/", GraphQL::new(schema)))
        .await
}
