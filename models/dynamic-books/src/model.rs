use async_graphql::{dynamic::*, Value, ID};
use futures_util::StreamExt;

use crate::{simple_broker::SimpleBroker, Book, BookChanged, MutationType, Storage};

impl<'a> From<MutationType> for FieldValue<'a> {
    fn from(value: MutationType) -> Self {
        match value {
            MutationType::Created => FieldValue::value("CREATED"),
            MutationType::Deleted => FieldValue::value("DELETED"),
        }
    }
}

pub fn schema() -> Result<Schema, SchemaError> {
    let mutation_type = Enum::new("MutationType")
        .item(EnumItem::new("CREATED").description("New book created."))
        .item(EnumItem::new("DELETED").description("Current book deleted."));

    let book = Object::new("Book")
        .description("A book that will be stored.")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::ID), |ctx| {
            FieldFuture::new(async move {
                let book = ctx.parent_value.try_downcast_ref::<Book>()?;
                Ok(Some(Value::from(book.id.to_owned())))
            })
        }))
        .field(Field::new(
            "name",
            TypeRef::named_nn(TypeRef::STRING),
            |ctx| {
                FieldFuture::new(async move {
                    let book = ctx.parent_value.try_downcast_ref::<Book>()?;
                    Ok(Some(Value::from(book.name.to_owned())))
                })
            },
        ))
        .field(Field::new(
            "author",
            TypeRef::named_nn(TypeRef::STRING),
            |ctx| {
                FieldFuture::new(async move {
                    let book = ctx.parent_value.try_downcast_ref::<Book>()?;
                    Ok(Some(Value::from(book.author.to_owned())))
                })
            },
        ));
    let book_changed = Object::new("BookChanged")
        .field(Field::new(
            "mutationType",
            TypeRef::named_nn(mutation_type.type_name()),
            |ctx| {
                FieldFuture::new(async move {
                    let book_changed = ctx.parent_value.try_downcast_ref::<BookChanged>()?;
                    Ok(Some(FieldValue::from(book_changed.mutation_type)))
                })
            },
        ))
        .field(Field::new("id", TypeRef::named_nn(TypeRef::ID), |ctx| {
            FieldFuture::new(async move {
                let book_changed = ctx.parent_value.try_downcast_ref::<BookChanged>()?;
                Ok(Some(Value::from(book_changed.id.to_owned())))
            })
        }))
        .field(Field::new(
            "book",
            TypeRef::named(book.type_name()),
            |ctx| {
                FieldFuture::new(async move {
                    let book_changed = ctx.parent_value.try_downcast_ref::<BookChanged>()?;
                    let id = book_changed.id.to_owned();
                    let book_id = id.parse::<usize>()?;
                    let store = ctx.data_unchecked::<Storage>().lock().await;
                    let book = store.get(book_id).cloned();
                    Ok(book.map(FieldValue::owned_any))
                })
            },
        ));

    let query_root = Object::new("Query")
        .field(Field::new(
            "getBooks",
            TypeRef::named_list(book.type_name()),
            |ctx| {
                FieldFuture::new(async move {
                    let store = ctx.data_unchecked::<Storage>().lock().await;
                    let books: Vec<Book> = store.iter().map(|(_, book)| book.clone()).collect();
                    Ok(Some(FieldValue::list(
                        books.into_iter().map(FieldValue::owned_any),
                    )))
                })
            },
        ))
        .field(
            Field::new("getBook", TypeRef::named(book.type_name()), |ctx| {
                FieldFuture::new(async move {
                    let id = ctx.args.try_get("id")?;
                    let book_id = match id.string() {
                        Ok(id) => id.to_string(),
                        Err(_) => id.u64()?.to_string(),
                    };
                    let book_id = book_id.parse::<usize>()?;
                    let store = ctx.data_unchecked::<Storage>().lock().await;
                    let book = store.get(book_id).cloned();
                    Ok(book.map(FieldValue::owned_any))
                })
            })
            .argument(InputValue::new("id", TypeRef::named_nn(TypeRef::ID))),
        );

    let mutatation_root = Object::new("Mutation")
        .field(
            Field::new("createBook", TypeRef::named(TypeRef::ID), |ctx| {
                FieldFuture::new(async move {
                    let mut store = ctx.data_unchecked::<Storage>().lock().await;
                    let name = ctx.args.try_get("name")?;
                    let author = ctx.args.try_get("author")?;
                    let entry = store.vacant_entry();
                    let id: ID = entry.key().into();
                    let book = Book {
                        id: id.clone(),
                        name: name.string()?.to_string(),
                        author: author.string()?.to_string(),
                    };
                    entry.insert(book);
                    let book_mutated = BookChanged {
                        mutation_type: MutationType::Created,
                        id: id.clone(),
                    };
                    SimpleBroker::publish(book_mutated);
                    Ok(Some(Value::from(id)))
                })
            })
            .argument(InputValue::new("name", TypeRef::named_nn(TypeRef::STRING)))
            .argument(InputValue::new(
                "author",
                TypeRef::named_nn(TypeRef::STRING),
            )),
        )
        .field(
            Field::new("deleteBook", TypeRef::named_nn(TypeRef::BOOLEAN), |ctx| {
                FieldFuture::new(async move {
                    let mut store = ctx.data_unchecked::<Storage>().lock().await;
                    let id = ctx.args.try_get("id")?;
                    let book_id = match id.string() {
                        Ok(id) => id.to_string(),
                        Err(_) => id.u64()?.to_string(),
                    };
                    let book_id = book_id.parse::<usize>()?;
                    if store.contains(book_id) {
                        store.remove(book_id);
                        let book_mutated = BookChanged {
                            mutation_type: MutationType::Deleted,
                            id: book_id.into(),
                        };
                        SimpleBroker::publish(book_mutated);
                        Ok(Some(Value::from(true)))
                    } else {
                        Ok(Some(Value::from(false)))
                    }
                })
            })
            .argument(InputValue::new("id", TypeRef::named_nn(TypeRef::ID))),
        );
    let subscription_root = Subscription::new("Subscription").field(SubscriptionField::new(
        "bookMutation",
        TypeRef::named_nn(book_changed.type_name()),
        |_| {
            SubscriptionFieldFuture::new(async {
                Ok(SimpleBroker::<BookChanged>::subscribe()
                    .map(|book| Ok(FieldValue::owned_any(book))))
            })
        },
    ));

    Schema::build(
        query_root.type_name(),
        Some(mutatation_root.type_name()),
        Some(subscription_root.type_name()),
    )
    .register(mutation_type)
    .register(book)
    .register(book_changed)
    .register(query_root)
    .register(subscription_root)
    .register(mutatation_root)
    .data(Storage::default())
    .finish()
}
