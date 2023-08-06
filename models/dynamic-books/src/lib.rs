mod model;
use async_graphql::ID;

pub use model::schema;
use slab::Slab;
use std::collections::HashMap;

type Storage = Slab<Book>;

#[derive(Clone)]
pub struct Book {
    id: ID,
    name: &'static str,
    author: &'static str,
}

pub struct BookStore {
    store: Storage,
    books_by_id: HashMap<String, usize>,
    value: usize,
}
impl BookStore {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut store = Slab::new();
        let key_1 = store.insert(Book {
            id: "10".into(),
            name: "Luke Skywalker",
            author: "Tatooine",
        });
        let key_2 = store.insert(Book {
            id: 1001.into(),
            name: "Anakin Skywalker",
            author: "Tatooine",
        });

        let mut books_by_id = HashMap::new();
        books_by_id.insert("10".to_string(), key_1);
        books_by_id.insert("1001".to_string(), key_2);

        Self {
            store,
            books_by_id,
            value: 10,
        }
    }

    pub fn get_book(&self, id: &str) -> Option<&Book> {
        self.books_by_id
            .get(id)
            .and_then(|idx| self.store.get(*idx))
    }

    pub fn get_books(&self) -> Vec<&Book> {
        self.store.iter().map(|(_, book)| book).collect()
    }

    pub fn create_book(&mut self, id: ID, name: &'static str, author: &'static str) -> &Book {
        let id_str = id.to_string();
        let book = Book { id, name, author };
        let key = self.store.insert(book.clone());
        self.books_by_id.insert(id_str, key);
        self.store.get(key).unwrap()
    }
}
