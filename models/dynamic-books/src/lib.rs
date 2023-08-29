mod model;
mod simple_broker;
use async_graphql::ID;

use futures_util::lock::Mutex;
pub use model::schema;
use slab::Slab;
use std::sync::Arc;

#[derive(Clone)]
pub struct Book {
    id: ID,
    name: String,
    author: String,
}

type Storage = Arc<Mutex<Slab<Book>>>;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum MutationType {
    Created,
    Deleted,
}

#[derive(Clone)]
struct BookChanged {
    mutation_type: MutationType,
    id: ID,
}
