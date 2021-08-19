use crate::*;

/// Get a specifyc transaction by hash
pub async fn get(client: &Client, hash: &str) -> Result<Transaction> {
    let json = NodeCall::transaction(hash.to_string());
    let url_path = "/";
    client.post(url_path, &json).await
}
