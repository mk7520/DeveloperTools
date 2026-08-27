// Error types for Code Maestro

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MaestroError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("LSP error: {0}")]
    LSP(String),

    #[error("AI engine error: {0}")]
    AIError(String),

    #[error("File operation error: {0}")]
    FileError(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    HttpError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Syntax error: {0}")]
    SyntaxError(String),

    #[error("Compilation error: {0}")]
    CompilationError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, MaestroError>;
