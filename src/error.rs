use thiserror::Error;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub(crate) struct ErrorElement {
    message: String,
    code: i32,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("request error")]
    Request(#[from] reqwest::Error),
    #[error("unexpected value")]
    Value(serde_json::Value),
    #[error("invalid decimals in {0}, only 8 allowed")]
    Decimals(String),
    #[error("unexpected or invalid number {0}")]
    Number(String),
    #[error("error code {1} from node: {0}")]
    NodeError(String, i32),
}

impl Error {
    pub fn value(value: serde_json::Value) -> Self {
        Self::Value(value)
    }

    pub fn decimals(value: &str) -> Self {
        Self::Decimals(value.to_string())
    }

    pub fn number(value: &str) -> Self {
        Self::Number(value.to_string())
    }
}
