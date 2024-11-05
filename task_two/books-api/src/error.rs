use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Book not found")]
    BookNotFound,
    #[error("Invalid command format: {0}")]
    InvalidCommand(String),
}
