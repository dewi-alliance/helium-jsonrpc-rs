use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::Duration;
use std::time::SystemTime;

pub use helium_api::models::transactions::Transaction;

pub mod error;

pub use error::{Error, Result};
pub mod blocks;
pub mod transactions;

/// The default timeout for API requests
pub const DEFAULT_TIMEOUT: u64 = 120;
/// The default base URL if none is specified.
pub const DEFAULT_BASE_URL: &str = "http://127.0.0.1:4467";
/// JSON RPC version
pub const JSON_RPC: &str = "2.0";

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

    async fn post<T: DeserializeOwned, D: Serialize>(&self, path: &str, data: D) -> Result<T> {
        #[derive(Clone, Serialize, Deserialize, Debug)]
        #[serde(untagged)]
        pub(crate) enum Response<T> {
            Data { result: T, id: String },
            Error { id: String, error: Error },
        }

        #[derive(Clone, Serialize, Deserialize, Debug)]
        pub(crate) struct Error {
            message: String,
            code: isize,
        }

        let request_url = format!("{}{}", self.base_url, path);
        let request = self.client.post(&request_url).json(&data);
        let response = request.send().await?;
        let body = response.text().await?;
        let v: Response<T> = serde_json::from_str(&body)?;
        match v {
            Response::Data { result, .. } => Ok(result),
            Response::Error { error, .. } => {
                Err(error::Error::NodeError(error.message, error.code))
            }
        }
    }
}

fn now_millis() -> String {
    let ms = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    ms.as_millis().to_string()
}

#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(tag = "method")]
#[serde(rename_all = "snake_case")]
enum Method {
    WalletList,
    BlockHeight,
    BlockGet { params: BlockParams },
    TransactionGet { params: TransactionParam },
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub(crate) struct NodeCall {
    jsonrpc: String,
    id: String,
    #[serde(flatten)]
    method: Method,
}
impl NodeCall {
    fn new(request: Method) -> NodeCall {
        NodeCall {
            jsonrpc: JSON_RPC.to_string(),
            id: now_millis(),
            method: request,
        }
    }

    pub(crate) fn height() -> Self {
        Self::new(Method::BlockHeight)
    }

    pub(crate) fn block(height: u64) -> Self {
        Self::new(Method::BlockGet {
            params: BlockParams { height },
        })
    }

    pub(crate) fn transaction(hash: String) -> Self {
        Self::new(Method::TransactionGet {
            params: TransactionParam { hash },
        })
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
struct BlockParams {
    height: u64,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
struct TransactionParam {
    hash: String,
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
