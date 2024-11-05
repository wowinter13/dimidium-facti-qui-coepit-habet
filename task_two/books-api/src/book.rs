use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    #[serde(deserialize_with = "deserialize_year")]
    pub year: u16,
}

fn deserialize_year<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Year {
        Num(u16),
        Str(String),
    }

    match Year::deserialize(deserializer)? {
        Year::Num(n) => Ok(n),
        Year::Str(s) => s.parse().map_err(serde::de::Error::custom),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum Command {
    #[serde(rename = "get_books")]
    GetBooks,
    #[serde(rename = "get_book")]
    GetBook { id: String },
    #[serde(rename = "add_book")]
    AddBook { book: Book },
    #[serde(rename = "update_book")]
    UpdateBook { id: String, book: Book },
    #[serde(rename = "delete_book")]
    DeleteBook { id: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Response {
    Success { data: ResponseData },
    Error { message: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseData {
    Books(HashMap<String, Book>),
    Book(Book),
    Id(String),
    None,
}

pub struct BookStore {
    books: RwLock<HashMap<String, Book>>,
}

impl BookStore {
    pub fn new() -> Self {
        Self {
            books: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_books(&self) -> HashMap<String, Book> {
        self.books.read().clone()
    }

    pub fn get_book(&self, id: &str) -> Option<Book> {
        self.books.read().get(id).cloned()
    }

    pub fn add_book(&self, book: Book) -> String {
        let id = Uuid::new_v4().to_string();
        self.books.write().insert(id.clone(), book);
        id
    }

    pub fn update_book(&self, id: &str, book: Book) -> bool {
        if let Some(existing) = self.books.write().get_mut(id) {
            *existing = book;
            true
        } else {
            false
        }
    }

    pub fn delete_book(&self, id: &str) -> bool {
        self.books.write().remove(id).is_some()
    }
}
