use async_trait::async_trait;
use futures::{future, Future as StdFuture, FutureExt, TryFutureExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::SystemTime;
use std::{pin::Pin, time::Duration};

/// A type alias for `Future` that may return `crate::error::Error`
pub type Future<T> = Pin<Box<dyn StdFuture<Output = Result<T>> + Send>>;

pub mod error;

pub use error::{Error, Result};
pub mod blocks;
pub mod transactions;

/// The default timeout for API requests
pub const DEFAULT_TIMEOUT: u64 = 120;
/// The default base URL if none is specified.
pub const DEFAULT_BASE_URL: &str = "http://127.0.0.1:4467";
/// A utility constant to pass an empty query slice to the various client fetch
/// functions
pub const NO_QUERY: &[&str; 0] = &[""; 0];

pub const JSON_RPC: &str = "2.0";

pub const BLOCK_HEIGHT: &str = "block_height";
pub const BLOCK_GET: &str = "block_get";
pub const TXN_GET: &str = "transaction_get";

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum Response<T> {
    Data { result: T, id: String },
    Error { id: String, error: ErrorElement },
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct ErrorElement {
    message: String,
    code: i32,
}

#[derive(Clone, Debug)]
pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new_with_base_url(DEFAULT_BASE_URL.to_string())
    }
}

impl Client {
    /// Create a new client using a given base URL and a default
    /// timeout. The library will use absoluate paths based on this
    /// base_url.
    pub fn new_with_base_url(base_url: String) -> Self {
        Self::new_with_timeout(base_url, DEFAULT_TIMEOUT)
    }

    /// Create a new client using a given base URL, and request
    /// timeout value.  The library will use absoluate paths based on
    /// the given base_url.
    pub fn new_with_timeout(base_url: String, timeout: u64) -> Self {
        let client = reqwest::Client::builder()
            .gzip(true)
            .timeout(Duration::from_secs(timeout))
            .build()
            .unwrap();
        Self { base_url, client }
    }

    pub(crate) fn post<T, R>(&self, path: &str, json: &T) -> Future<Result<R>>
    where
        T: Serialize + ?Sized,
        R: 'static + DeserializeOwned + std::marker::Send,
    {
        let request_url = format!("{}{}", self.base_url, path);
        self.client
            .post(&request_url)
            .json(json)
            .send()
            .map_err(error::Error::from)
            .and_then(|response| match response.error_for_status() {
                Ok(result) => {
                    let data: Future<Result<R>> = result
                        .json()
                        .map_err(error::Error::from)
                        .map_ok(|v: Response<R>| match v {
                            Response::Data { result, .. } => Ok(result),
                            Response::Error { error, .. } => {
                                Err(Error::NodeError(error.message, error.code))
                            }
                        })
                        .boxed();
                    data
                }
                Err(e) => future::err(error::Error::from(e)).boxed(),
            })
            .boxed()
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Params {
    Hash(String),
    Height(u64),
    None(String),
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct NodeCall {
    jsonrpc: String,
    id: String,
    method: String,
    params: Option<Params>,
}

impl NodeCall {
    pub(crate) fn height() -> Self {
        NodeCall {
            jsonrpc: JSON_RPC.to_string(),
            id: now_millis(),
            method: BLOCK_HEIGHT.to_string(),
            params: Some(Params::None("null".to_string())),
        }
    }
    pub(crate) fn block(height: u64) -> Self {
        NodeCall {
            jsonrpc: JSON_RPC.to_string(),
            id: now_millis(),
            method: BLOCK_GET.to_string(),
            params: Some(Params::Height(height)),
        }
    }
    pub(crate) fn transaction(hash: String) -> Self {
        NodeCall {
            jsonrpc: JSON_RPC.to_string(),
            id: now_millis(),
            method: TXN_GET.to_string(),
            params: Some(Params::Hash(hash)),
        }
    }
}

fn now_millis() -> String {
    let ms = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    ms.as_millis().to_string()
}

#[async_trait]
pub trait IntoVec {
    type Item;

    async fn into_vec(self) -> Result<Vec<Self::Item>>;
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[test]
    async fn txn_err() {
        let client = Client::default();
        let txn = transactions::get(&client, "1gidN7e6OKn405Fru_0sGhsqca3lTsrfGKrM4dwM").await;
        let er = match txn {
            Err(e) => format!("{}", e),
            _ => panic!("??"),
        };
        assert_eq!(er, "error code -100 from node: No transaction: <<\"1gidN7e6OKn405Fru_0sGhsqca3lTsrfGKrM4dwM\">>");
    }
}
