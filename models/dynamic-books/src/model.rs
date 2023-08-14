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
                    Ok(Some(Value::from(book.name.to_owned())))
                })
            })
            .description("The name of the book."),
        )
        .field(
            Field::new("author", TypeRef::named_nn(TypeRef::STRING), |ctx| {
                FieldFuture::new(async move {
                    let book = ctx.parent_value.try_downcast_ref::<Book>()?;
                    Ok(Some(Value::from(book.author.to_owned())))
                })
            })
            .description("The author of the book."),
        );

    let query = Object::new("Query")
        .field(
            Field::new("getBook", TypeRef::named(book.type_name()), |ctx| {
                FieldFuture::new(async move {
                    let id = ctx.args.try_get("id")?;
                    let book_by_id = &ctx.data_unchecked::<BookStore>().books_by_id.lock().await;
                    let book_id = book_by_id.get(id.string()?).unwrap();
                    let store = ctx.data_unchecked::<BookStore>().store.lock().await;
                    let book = store.get(*book_id).cloned();
                    Ok(book.map(FieldValue::owned_any))
                })
            })
            .argument(InputValue::new("id", TypeRef::named_nn(TypeRef::STRING))),
        )
        .field(Field::new(
            "getBooks",
            TypeRef::named_nn_list_nn(book.type_name()),
            |ctx| {
                FieldFuture::new(async move {
                    let store = ctx.data_unchecked::<BookStore>().store.lock().await;
                    let books: Vec<Book> = store.iter().map(|(_, book)| book.clone()).collect();
                    Ok(Some(FieldValue::list(
                        books.into_iter().map(FieldValue::owned_any),
                    )))
                })
            },
        ));

    let mutatation = Object::new("Mutation").field(
        Field::new("createBook", TypeRef::named(book.type_name()), |ctx| {
            FieldFuture::new(async move {
                let mut book_by_id = ctx.data_unchecked::<BookStore>().books_by_id.lock().await;
                let mut store = ctx.data_unchecked::<BookStore>().store.lock().await;
                let id = ctx.args.try_get("id")?;
                let name = ctx.args.try_get("name")?;
                let author = ctx.args.try_get("author")?;
                let book = Book {
                    id: id.string()?.into(),
                    name: name.string()?.to_string(),
                    author: author.string()?.to_string(),
                };
                let key = store.insert(book);
                book_by_id.insert(id.string()?.to_string(), key);
                Ok(Some(FieldValue::owned_any(store.get(key).unwrap().clone())))
            })
        })
        .argument(InputValue::new("id", TypeRef::named_nn(TypeRef::STRING)))
        .argument(InputValue::new("name", TypeRef::named_nn(TypeRef::STRING)))
        .argument(InputValue::new(
            "author",
            TypeRef::named_nn(TypeRef::STRING),
        )),
    );
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

    Schema::build(
        query.type_name(),
        Some(mutatation.type_name()),
        Some(subscription.type_name()),
    )
    .register(book)
    .register(query)
    .register(subscription)
    .register(mutatation)
    .data(BookStore::new())
    .finish()
}
