use async_graphql::dynamic::{
    Field, FieldFuture, FieldValue, Object, Schema, SchemaError, TypeRef,
};
use async_graphql_poem::GraphQL;
use poem::{listener::TcpListener, Route, Server};

struct Product {
    upc: String,
    name: String,
    price: i32,
}

fn schema() -> Result<Schema, SchemaError> {
    let hats = vec![
        Product {
            upc: "top-1".to_string(),
            name: "Trilby".to_string(),
            price: 11,
        },
        Product {
            upc: "top-2".to_string(),
            name: "Fedora".to_string(),
            price: 22,
        },
        Product {
            upc: "top-3".to_string(),
            name: "Boater".to_string(),
            price: 33,
        },
    ];

    let product = Object::new("Product")
        .field(Field::new(
            "upc",
            TypeRef::named_nn(TypeRef::STRING),
            |ctx| {
                FieldFuture::new(async move {
                    let product = ctx.parent_value.try_downcast_ref::<Product>()?;
                    Ok(Some(FieldValue::value(&product.upc)))
                })
            },
        ))
        .field(Field::new(
            "name",
            TypeRef::named_nn(TypeRef::STRING),
            |ctx| {
                FieldFuture::new(async move {
                    let product = ctx.parent_value.try_downcast_ref::<Product>()?;
                    Ok(Some(FieldValue::value(&product.name)))
                })
            },
        ))
        .field(
            Field::new("price", TypeRef::named_nn(TypeRef::INT), |ctx| {
                FieldFuture::new(async move {
                    let product = ctx.parent_value.try_downcast_ref::<Product>()?;
                    Ok(Some(FieldValue::value(product.price)))
                })
            })
            .shareable(),
        )
        .key("upc");

    let query = Object::new("Query").field(Field::new(
        "topProducts",
        TypeRef::named_nn_list_nn(product.type_name()),
        |ctx| {
            FieldFuture::new(async move {
                let mut values = Vec::new();
                let products = ctx.data_unchecked::<Vec<Product>>();
                for product in products {
                    values.push(FieldValue::borrowed_any(product));
                }
                Ok(Some(values))
            })
        },
    ));

    Schema::build("Query", None, None)
        .data(hats)
        .register(product)
        .register(query)
        .entity_resolver(|ctx| {
            FieldFuture::new(async move {
                let products = ctx.data_unchecked::<Vec<Product>>();
                let representations = ctx.args.try_get("representations")?.list()?;
                let mut values = Vec::new();

                for item in representations.iter() {
                    let item = item.object()?;
                    let typename = item
                        .try_get("__typename")
                        .and_then(|value| value.string())?;

                    if typename == "Product" {
                        let upc = item.try_get("upc")?.string()?;
                        if let Some(product) = products.iter().find(|product| product.upc == upc) {
                            values.push(FieldValue::borrowed_any(product));
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
    Server::new(TcpListener::bind("127.0.0.1:4002"))
        .run(Route::new().at("/", GraphQL::new(schema().unwrap())))
        .await
}
