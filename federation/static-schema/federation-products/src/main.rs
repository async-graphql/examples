use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use async_graphql_poem::GraphQL;
use poem::{Route, Server, listener::TcpListener};

#[derive(SimpleObject)]
struct Product {
    upc: String,
    name: String,
    #[graphql(shareable)]
    price: i32,
}

struct Query;

#[Object]
impl Query {
    async fn top_products<'a>(&self, ctx: &'a Context<'_>) -> &'a Vec<Product> {
        ctx.data_unchecked::<Vec<Product>>()
    }

    #[graphql(entity)]
    async fn find_product_by_upc<'a>(
        &self,
        ctx: &'a Context<'_>,
        upc: String,
    ) -> Option<&'a Product> {
        let hats = ctx.data_unchecked::<Vec<Product>>();
        hats.iter().find(|product| product.upc == upc)
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
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

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(hats)
        .directive(directives::lowercase)
        .finish();

    Server::new(TcpListener::bind("127.0.0.1:4002"))
        .run(Route::new().at("/", GraphQL::new(schema)))
        .await
}
