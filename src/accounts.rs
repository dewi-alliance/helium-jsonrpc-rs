use crate::*;

#[derive(Deserialize, Debug)]
pub struct Account {
    pub address: String,
    pub block: u64,
    pub balance: Hnt,
    pub nonce: u64,
    pub sec_balance: Hst,
    pub sec_nonce: u64,
    pub dc_balance: u64,
    pub dc_nonce: u64,
}

/// Get current account state
pub async fn get(client: &Client, pubkey: &str) -> Result<Account> {
    let json = NodeCall::account_get(pubkey.to_string(), None);
    let url_path = "/";
    client.post(url_path, &json).await
}

/// Get an account state at a certain height
pub async fn get_at_height(client: &Client, pubkey: &str, height: u64) -> Result<Account> {
    let json = NodeCall::account_get(pubkey.to_string(), Some(height));
    let url_path = "/";
    client.post(url_path, &json).await
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[test]
    async fn get_account() {
        let client = Client::default();
        let account = accounts::get(
            &client,
            "1be3xdTQTYX8UbA5ND5F1cKcGEw1BSD2akyFtXbJkxDm5JtLzzD",
        )
        .await
        .unwrap();
    }
}
