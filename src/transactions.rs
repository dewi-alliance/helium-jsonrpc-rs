use crate::*;

/// Get a specifyc transaction by hash
pub async fn get(client: &Client, hash: &str) -> Result<Transaction> {
    let json = NodeCall::transaction(hash.to_string());
    let url_path = "/";
    client.post(url_path, &json).await
}

#[derive(Deserialize)]
struct SubmitResult {
    hash: String,
}

/// Submit a transaction in base64
pub async fn submit(client: &Client, base64: &str) -> Result<String> {
    let json = NodeCall::transaction_submit(base64.to_string());
    let url_path = "/";
    let result: SubmitResult = client.post(url_path, &json).await?;
    Ok(result.hash)
}

/// Verify a transaction in base64
pub async fn verify(client: &Client, base64: &str) -> Result {
    let json = NodeCall::transaction_verify(base64.to_string());
    let url_path = "/";
    let result: String = client.post(url_path, &json).await?;
    if result == "valid" {
        Ok(())
    } else {
        Err(Error::TxnInvalid(result))
    }
}
