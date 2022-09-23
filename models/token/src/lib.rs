use async_graphql::{Context, Data, EmptyMutation, Object, Result, Schema, Subscription};
use futures_util::Stream;
use serde::Deserialize;

pub type TokenSchema = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;

pub struct Token(pub String);

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<Token>().map(|token| token.0.as_str())
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn values(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = i32>> {
        if ctx.data::<Token>()?.0 != "123456" {
            return Err("Forbidden".into());
        }
        Ok(futures_util::stream::once(async move { 10 }))
    }
}

// For more details see:
// https://github.com/enisdenjo/graphql-ws/blob/master/PROTOCOL.md#connectioninit
pub async fn on_connection_init(value: serde_json::Value) -> Result<Data> {
    #[derive(Deserialize)]
    struct Payload {
        token: String,
    }

    // Coerce the connection params into our `Payload` struct so we can
    // validate the token exists in the headers.
    if let Ok(payload) = serde_json::from_value::<Payload>(value) {
        let mut data = Data::default();
        data.insert(Token(payload.token));
        Ok(data)
    } else {
        Err("Token is required".into())
    }
}
