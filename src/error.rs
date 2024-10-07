use std::sync::PoisonError;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ErrorLLM>;

#[derive(Debug, Error)]
pub enum ErrorLLM {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parsing error: {0}")]
    SerdeParse(#[from] serde_json::Error),

    #[error("Ollama error: {0}")]
    Ollama(#[from] ollama_rs::error::OllamaError),

    #[error("ParseColor error: {0}")]
    Ratatui(#[from] ratatui::style::ParseColorError),

    #[error("Empty input")]
    EmptyInput,
}
