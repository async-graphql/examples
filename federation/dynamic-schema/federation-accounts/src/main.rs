use async_graphql::dynamic::{
    Field, FieldFuture, FieldValue, Object, Schema, SchemaError, TypeRef,
};
use async_graphql_poem::GraphQL;
use poem::{listener::TcpListener, Route, Server};

struct Picture {
    url: String,
    width: u32,
    height: u32,
}

struct User {
    id: String,
    username: String,
    profile_picture: Option<Picture>,
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

fn schema() -> Result<Schema, SchemaError> {
    let picture = Object::new("Picture")
        .field(Field::new(
            "url",
            TypeRef::named_nn(TypeRef::STRING),
            |ctx| {
                FieldFuture::new(async move {
                    let picture = ctx.parent_value.try_downcast_ref::<Picture>()?;
                    Ok(Some(FieldValue::value(&picture.url)))
                })
            },
        ))
        .field(Field::new(
            "width",
            TypeRef::named_nn(TypeRef::INT),
            |ctx| {
                FieldFuture::new(async move {
                    let picture = ctx.parent_value.try_downcast_ref::<Picture>()?;
                    Ok(Some(FieldValue::value(picture.width)))
                })
            },
        ))
        .field(Field::new(
            "height",
            TypeRef::named_nn(TypeRef::INT),
            |ctx| {
                FieldFuture::new(async move {
                    let picture = ctx.parent_value.try_downcast_ref::<Picture>()?;
                    Ok(Some(FieldValue::value(picture.height)))
                })
            },
        ));

    let user = Object::new("User")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::ID), |ctx| {
            FieldFuture::new(async move {
                let user = ctx.parent_value.try_downcast_ref::<User>()?;
                Ok(Some(FieldValue::value(&user.id)))
            })
        }))
        .field(Field::new(
            "username",
            TypeRef::named_nn(TypeRef::STRING),
            |ctx| {
                FieldFuture::new(async move {
                    let user = ctx.parent_value.try_downcast_ref::<User>()?;
                    Ok(Some(FieldValue::value(&user.username)))
                })
            },
        ))
        .field(Field::new(
            "profilePicture",
            TypeRef::named_nn(TypeRef::STRING),
            |ctx| {
                FieldFuture::new(async move {
                    let user = ctx.parent_value.try_downcast_ref::<User>()?;
                    Ok(user.profile_picture.as_ref().map(FieldValue::borrowed_any))
                })
            },
        ))
        .field(Field::new(
            "reviewCount",
            TypeRef::named_nn(TypeRef::INT),
            |ctx| {
                FieldFuture::new(async move {
                    let user = ctx.parent_value.try_downcast_ref::<User>()?;
                    Ok(Some(FieldValue::value(user.review_count)))
                })
            },
        ))
        .field(Field::new(
            "joinedTimestamp",
            TypeRef::named_nn(TypeRef::STRING),
            |ctx| {
                FieldFuture::new(async move {
                    let user = ctx.parent_value.try_downcast_ref::<User>()?;
                    Ok(Some(FieldValue::value(user.joined_timestamp.to_string())))
                })
            },
        ))
        .key("id");

    let query = Object::new("Query").field(Field::new(
        "me",
        TypeRef::named_nn(user.type_name()),
        |_| FieldFuture::new(async move { Ok(Some(FieldValue::owned_any(User::me()))) }),
    ));

    Schema::build("Query", None, None)
        .register(picture)
        .register(user)
        .register(query)
        .entity_resolver(|ctx| {
            FieldFuture::new(async move {
                let representations = ctx.args.try_get("representations")?.list()?;
                let mut values = Vec::new();

                for item in representations.iter() {
                    let item = item.object()?;
                    let typename = item
                        .try_get("__typename")
                        .and_then(|value| value.string())?;

                    if typename == "User" {
                        let id = item.try_get("id")?.string()?;
                        if id == "1234" {
                            values.push(FieldValue::owned_any(User::me()));
                        } else {
                            let username = format!("User {}", id);
                            let user = User {
                                id: id.to_string(),
                                username,
                                profile_picture: None,
                                review_count: 0,
                                joined_timestamp: 1500,
                            };
                            values.push(FieldValue::owned_any(user));
                        }
                    }
                }

                Ok(Some(FieldValue::list(values)))
            })
        })
        .finish()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    Server::new(TcpListener::bind("127.0.0.1:4001"))
        .run(Route::new().at("/", GraphQL::new(schema().unwrap())))
        .await
}
