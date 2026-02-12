use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Compiler error: {0}")]
    Compiler(String),
    #[error("Line {0}: {1}")]
    User(usize, String),
}