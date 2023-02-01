use async_graphql::{Context, CustomDirective, Directive, ResolveFut, ServerResult, Value};

struct LowercaseDirective;

#[async_trait::async_trait]
impl CustomDirective for LowercaseDirective {
    async fn resolve_field(
        &self,
        _ctx: &Context<'_>,
        resolve: ResolveFut<'_>,
    ) -> ServerResult<Option<Value>> {
        resolve.await.map(|value| {
            value.map(|value| match value {
                Value::String(str) => Value::String(str.to_ascii_lowercase()),
                _ => value,
            })
        })
    }
}

#[Directive(location = "field")]
pub fn lowercase() -> impl CustomDirective {
    LowercaseDirective
}
