use async_graphql::dynamic::{
    Enum, Field, FieldFuture, FieldValue, Object, Schema, SchemaError, TypeRef,
};
use async_graphql_poem::GraphQL;
use poem::{Route, Server, listener::TcpListener};

struct Picture {
    url: String,
    width: u32,
    height: u32,
    alt_text: String,
}

struct Review {
    id: String,
    body: String,
    pictures: Vec<Picture>,
}

struct Product {
    upc: String,
    price: u32,
}

impl Review {
    fn get_product(&self) -> Product {
        match self.id.as_str() {
            "review-1" => Product {
                upc: "top-1".to_string(),
                price: 10,
            },
            "review-2" => Product {
                upc: "top-2".to_string(),
                price: 20,
            },
            "review-3" => Product {
                upc: "top-3".to_string(),
                price: 30,
            },
            _ => panic!("Unknown review id"),
        }
    }

    fn get_author(&self) -> User {
        let user_id = match self.id.as_str() {
            "review-1" => "1234",
            "review-2" => "1234",
            "review-3" => "7777",
            _ => panic!("Unknown review id"),
        }
        .to_string();
        user_by_id(user_id, None)
    }
}

struct User {
    id: String,
    review_count: u32,
    joined_timestamp: u64,
}

fn user_by_id(id: String, joined_timestamp: Option<u64>) -> User {
    let review_count = match id.as_str() {
        "1234" => 2,
        "7777" => 1,
        _ => 0,
    };
    // This will be set if the user requested the fields that require it.
    let joined_timestamp = joined_timestamp.unwrap_or(9001);
    User {
        id,
        review_count,
        joined_timestamp,
    }
}

fn schema() -> Result<Schema, SchemaError> {
    let picture = Object::new("Picture")
        .shareable()
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
        ))
        .field(
            Field::new("altText", TypeRef::named_nn(TypeRef::INT), |ctx| {
                FieldFuture::new(async move {
                    let picture = ctx.parent_value.try_downcast_ref::<Picture>()?;
                    Ok(Some(FieldValue::value(&picture.alt_text)))
                })
            })
            .inaccessible(),
        );

    let review = Object::new("Review")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::ID), |ctx| {
            FieldFuture::new(async move {
                let review = ctx.parent_value.try_downcast_ref::<Review>()?;
                Ok(Some(FieldValue::value(&review.id)))
            })
        }))
        .field(Field::new(
            "body",
            TypeRef::named_nn(TypeRef::STRING),
            |ctx| {
                FieldFuture::new(async move {
                    let review = ctx.parent_value.try_downcast_ref::<Review>()?;
                    Ok(Some(FieldValue::value(&review.body)))
                })
            },
        ))
        .field(Field::new(
            "pictures",
            TypeRef::named_nn_list_nn(picture.type_name()),
            |ctx| {
                FieldFuture::new(async move {
                    let review = ctx.parent_value.try_downcast_ref::<Review>()?;
                    Ok(Some(FieldValue::list(
                        review
                            .pictures
                            .iter()
                            .map(|review| FieldValue::borrowed_any(review)),
                    )))
                })
            },
        ))
        .field(
            Field::new("product", TypeRef::named_nn(TypeRef::STRING), |ctx| {
                FieldFuture::new(async move {
                    let review = ctx.parent_value.try_downcast_ref::<Review>()?;
                    Ok(Some(FieldValue::owned_any(review.get_product())))
                })
            })
            .provides("price"),
        )
        .field(Field::new("author", TypeRef::named_nn("User"), |ctx| {
            FieldFuture::new(async move {
                let review = ctx.parent_value.try_downcast_ref::<Review>()?;
                let author = review.get_author();
                Ok(Some(FieldValue::owned_any(author)))
            })
        }));

    let trust_worthiness =
        Enum::new("Trustworthiness").items(["ReallyTrusted", "KindaTrusted", "NotTrusted"]);

    let user = Object::new("User")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::ID), |ctx| {
            FieldFuture::new(async move {
                let user = ctx.parent_value.try_downcast_ref::<User>()?;
                Ok(Some(FieldValue::value(&user.id)))
            })
        }))
        .field(
            Field::new("reviewCount", TypeRef::named_nn(TypeRef::INT), |ctx| {
                FieldFuture::new(async move {
                    let user = ctx.parent_value.try_downcast_ref::<User>()?;
                    Ok(Some(FieldValue::value(user.review_count)))
                })
            })
            .override_from("accounts"),
        )
        .field(
            Field::new("joinedTimestamp", TypeRef::named_nn(TypeRef::INT), |ctx| {
                FieldFuture::new(async move {
                    let user = ctx.parent_value.try_downcast_ref::<User>()?;
                    Ok(Some(FieldValue::value(user.joined_timestamp)))
                })
            })
            .external(),
        )
        .field(Field::new(
            "reviews",
            TypeRef::named_nn_list_nn(review.type_name()),
            |ctx| {
                FieldFuture::new(async move {
                    let reviews = ctx.data::<Vec<Review>>()?;
                    Ok(Some(FieldValue::list(
                        reviews
                            .iter()
                            .map(|review| FieldValue::borrowed_any(review)),
                    )))
                })
            },
        ))
        .field(
            Field::new(
                "trustworthiness",
                TypeRef::named_nn_list_nn(review.type_name()),
                |ctx| {
                    FieldFuture::new(async move {
                        let user = ctx.parent_value.try_downcast_ref::<User>()?;
                        Ok(Some(
                            if user.joined_timestamp < 1_000 && user.review_count > 1 {
                                FieldValue::value("ReallyTrusted")
                            } else if user.joined_timestamp < 2_000 {
                                FieldValue::value("KindaTrusted")
                            } else {
                                FieldValue::value("NotTrusted")
                            },
                        ))
                    })
                },
            )
            .requires("joinedTimestamp"),
        )
        .key("id");

    let product = Object::new("Product")
        .field(Field::new("upc", TypeRef::named_nn(TypeRef::ID), |ctx| {
            FieldFuture::new(async move {
                let product = ctx.parent_value.try_downcast_ref::<Product>()?;
                Ok(Some(FieldValue::value(&product.upc)))
            })
        }))
        .field(
            Field::new("price", TypeRef::named_nn(TypeRef::INT), |ctx| {
                FieldFuture::new(async move {
                    let product = ctx.parent_value.try_downcast_ref::<Product>()?;
                    Ok(Some(FieldValue::value(product.price)))
                })
            })
            .external(),
        )
        .field(Field::new(
            "reviews",
            TypeRef::named_nn_list_nn(review.type_name()),
            |ctx| {
                FieldFuture::new(async move {
                    let user = ctx.parent_value.try_downcast_ref::<User>()?;
                    let reviews = ctx.data::<Vec<Review>>()?;
                    Ok(Some(FieldValue::list(
                        reviews
                            .iter()
                            .filter(|review| review.get_author().id == user.id)
                            .map(|review| FieldValue::borrowed_any(review)),
                    )))
                })
            },
        ))
        .key("upc");

    let reviews = vec![
            Review {
                id: "review-1".into(),
                body: "A highly effective form of birth control.".into(),
                pictures: vec![
                    Picture {
                        url: "http://localhost:8080/ugly_hat.jpg".to_string(),
                        width: 100,
                        height: 100,
                        alt_text: "A Trilby".to_string(),
                    },
                    Picture {
                        url: "http://localhost:8080/troll_face.jpg".to_string(),
                        width: 42,
                        height: 42,
                        alt_text: "The troll face meme".to_string(),
                    },
                ],
            },
            Review {
                id: "review-2".into(),
                body: "Fedoras are one of the most fashionable hats around and can look great with a variety of outfits.".into(),
                pictures: vec![],
            },
            Review {
                id: "review-3".into(),
                body: "This is the last straw. Hat you will wear. 11/10".into(),
                pictures: vec![],
            },
        ];

    let query = Object::new("Query");

    Schema::build("Query", None, None)
        .data(reviews)
        .register(picture)
        .register(review)
        .register(trust_worthiness)
        .register(user)
        .register(product)
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
                        let joined_timestamp = item
                            .get("joinedTimestamp")
                            .and_then(|value| value.u64().ok());
                        values.push(FieldValue::owned_any(user_by_id(
                            id.to_string(),
                            joined_timestamp,
                        )));
                    } else if typename == "Product" {
                        let upc = item.try_get("upc")?.string()?;
                        values.push(FieldValue::owned_any(Product {
                            upc: upc.to_string(),
                            price: 0,
                        }));
                    }
                }

                Ok(Some(FieldValue::list(values)))
            })
        })
        .finish()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    Server::new(TcpListener::bind("127.0.0.1:4003"))
        .run(Route::new().at("/", GraphQL::new(schema().unwrap())))
        .await
}
