mod model;
mod simple_broker;
use std::{str::FromStr, sync::Arc};

use async_graphql::ID;
use futures_util::lock::Mutex;
pub use model::schema;
use slab::Slab;

#[derive(Clone)]
pub struct Book {
    id: ID,
    name: String,
    author: String,
}

type Storage = Arc<Mutex<Slab<Book>>>;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum MutationType {
    Created,
    Deleted,
}

impl FromStr for MutationType {
    type Err = String; // Error type can be customized based on your needs

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CREATED" => Ok(MutationType::Created),
            "DELETED" => Ok(MutationType::Deleted),
            _ => Err(format!("Invalid MutationType: {}", s)),
        }
    }
}

#[derive(Clone)]
struct BookChanged {
    mutation_type: MutationType,
    id: ID,
}
