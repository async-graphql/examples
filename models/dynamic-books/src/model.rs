use async_graphql::{dynamic::*, Value};
use futures_util::StreamExt;

use crate::{Book, BookStore};

pub fn schema() -> Result<Schema, SchemaError> {
    let book = Object::new("Book")
        .description("A book that will be stored.")
        .field(
            Field::new("id", TypeRef::named_nn(TypeRef::ID), |ctx| {
                FieldFuture::new(async move {
                    let book = ctx.parent_value.try_downcast_ref::<Book>()?;
                    Ok(Some(Value::from(book.id.to_owned())))
                })
            })
            .description("The id of the book."),
        )
        .field(
            Field::new("name", TypeRef::named_nn(TypeRef::STRING), |ctx| {
                FieldFuture::new(async move {
                    let book = ctx.parent_value.try_downcast_ref::<Book>()?;
                    Ok(Some(Value::from(book.name)))
                })
            })
            .description("The name of the book."),
        )
        .field(
            Field::new("author", TypeRef::named_nn(TypeRef::STRING), |ctx| {
                FieldFuture::new(async move {
                    let book = ctx.parent_value.try_downcast_ref::<Book>()?;
                    Ok(Some(Value::from(book.author)))
                })
            })
            .description("The author of the book."),
        );

    let query = Object::new("Query")
        .field(
            Field::new("getBook", TypeRef::named(book.type_name()), |ctx| {
                FieldFuture::new(async move {
                    let store = ctx.data::<BookStore>().unwrap();
                    let id = ctx.args.try_get("id")?;
                    Ok(store
                        .get_book(id.string()?)
                        .map(|b| FieldValue::borrowed_any(b)))
                })
            })
            .argument(InputValue::new("id", TypeRef::named_nn(TypeRef::STRING))),
        )
        .field(Field::new(
            "getBooks",
            TypeRef::named_nn_list_nn(book.type_name()),
            |ctx| {
                FieldFuture::new(async move {
                    let store = ctx.data::<BookStore>().unwrap();
                    let books = store.get_books();
                    Ok(Some(FieldValue::list(
                        books.into_iter().map(|b| FieldValue::borrowed_any(b)),
                    )))
                })
            },
        ));

    // Todo:Show book.value
    let subscription = Subscription::new("Subscription").field(SubscriptionField::new(
        "value",
        TypeRef::named_nn(TypeRef::INT),
        |ctx| {
            SubscriptionFieldFuture::new(async move {
                let value_1 = ctx.data_unchecked::<BookStore>().value;
                Ok(futures_util::stream::repeat(value_1).map(|value| Ok(Value::from(value))))
            })
        },
    ));

    Schema::build(query.type_name(), None, Some(subscription.type_name()))
        .register(book)
        .register(query)
        .register(subscription)
        .data(BookStore::new())
        .finish()
}
