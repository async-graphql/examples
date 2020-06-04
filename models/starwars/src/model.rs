use super::StarWars;
use async_graphql::connection::{Connection, DataSource, EmptyFields};
use async_graphql::{Context, FieldResult};

/// One of the films in the Star Wars Trilogy
#[async_graphql::Enum]
pub enum Episode {
    /// Released in 1977.
    NewHope,

    /// Released in 1980.
    Empire,

    /// Released in 1983.
    Jedi,
}

pub struct Human(usize);

/// A humanoid creature in the Star Wars universe.
#[async_graphql::Object]
impl Human {
    /// The id of the human.
    async fn id(&self, ctx: &Context<'_>) -> &str {
        ctx.data::<StarWars>().chars[self.0].id
    }

    /// The name of the human.
    async fn name(&self, ctx: &Context<'_>) -> &str {
        ctx.data::<StarWars>().chars[self.0].name
    }

    /// The friends of the human, or an empty list if they have none.
    async fn friends(&self, ctx: &Context<'_>) -> Vec<Character> {
        ctx.data::<StarWars>().chars[self.0]
            .friends
            .iter()
            .map(|id| Human(*id).into())
            .collect()
    }

    /// Which movies they appear in.
    async fn appears_in<'a>(&self, ctx: &'a Context<'_>) -> &'a [Episode] {
        &ctx.data::<StarWars>().chars[self.0].appears_in
    }

    /// The home planet of the human, or null if unknown.
    async fn home_planet<'a>(&self, ctx: &'a Context<'_>) -> &'a Option<&'a str> {
        &ctx.data::<StarWars>().chars[self.0].home_planet
    }
}

pub struct Droid(usize);

/// A mechanical creature in the Star Wars universe.
#[async_graphql::Object]
impl Droid {
    /// The id of the droid.
    async fn id(&self, ctx: &Context<'_>) -> &str {
        ctx.data::<StarWars>().chars[self.0].id
    }

    /// The name of the droid.
    async fn name(&self, ctx: &Context<'_>) -> &str {
        ctx.data::<StarWars>().chars[self.0].name
    }

    /// The friends of the droid, or an empty list if they have none.
    async fn friends(&self, ctx: &Context<'_>) -> Vec<Character> {
        ctx.data::<StarWars>().chars[self.0]
            .friends
            .iter()
            .map(|id| Droid(*id).into())
            .collect()
    }

    /// Which movies they appear in.
    async fn appears_in<'a>(&self, ctx: &'a Context<'_>) -> &'a [Episode] {
        &ctx.data::<StarWars>().chars[self.0].appears_in
    }

    /// The primary function of the droid.
    async fn primary_function<'a>(&self, ctx: &'a Context<'_>) -> &'a Option<&'a str> {
        &ctx.data::<StarWars>().chars[self.0].primary_function
    }
}

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn hero(
        &self,
        ctx: &Context<'_>,
        #[arg(
            desc = "If omitted, returns the hero of the whole saga. If provided, returns the hero of that particular episode."
        )]
        episode: Episode,
    ) -> Character {
        if episode == Episode::Empire {
            Human(ctx.data::<StarWars>().luke).into()
        } else {
            Droid(ctx.data::<StarWars>().artoo).into()
        }
    }

    async fn human(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "id of the human")] id: String,
    ) -> Option<Human> {
        ctx.data::<StarWars>().human(&id).map(Human)
    }

    async fn humans(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> FieldResult<Connection<usize, Human, EmptyFields, EmptyFields>> {
        let humans = ctx
            .data::<StarWars>()
            .humans()
            .iter()
            .copied()
            .collect::<Vec<_>>();
        humans
            .as_slice()
            .query(after, before, first, last)
            .await
            .map(|conn| conn.map_node(|id| Human(*id)))
    }

    async fn droid(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "id of the droid")] id: String,
    ) -> Option<Droid> {
        ctx.data::<StarWars>().droid(&id).map(Droid)
    }

    async fn droids(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> FieldResult<Connection<usize, Droid, EmptyFields, EmptyFields>> {
        let droids = ctx
            .data::<StarWars>()
            .droids()
            .iter()
            .copied()
            .collect::<Vec<_>>();
        droids
            .as_slice()
            .query(after, before, first, last)
            .await
            .map(|conn| conn.map_node(|id| Droid(*id)))
    }
}

#[async_graphql::Interface(
    field(name = "id", type = "&str", context),
    field(name = "name", type = "&str", context),
    field(name = "friends", type = "Vec<Character>", context),
    field(name = "appears_in", type = "&'ctx [Episode]", context)
)]
pub enum Character {
    Human(Human),
    Droid(Droid),
}
