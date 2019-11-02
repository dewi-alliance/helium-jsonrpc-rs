use crate::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Represents a block response from blockchain-node.
pub struct Block {
	pub height: u64,
	pub hash: String,
	pub prev_hash: String,
	pub time: u64,
	pub transactions: Vec<String>,
}


/// Get the current height of the blockchain
pub async fn height(client: &Client) -> Result<u64> {
	let json = json!(NodeCall::height());
    client.post("/", &json).await?
}

pub async fn get_block(client: &Client, height: &u64) -> Result<Block> {
		let json = json!(NodeCall::block(*height));
		let url_path = "/";

    client
        .post(&url_path, &json)
        .await?
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[test]
    async fn height() {
        let client = Client::default();
        let height = blocks::height(&client).await.expect("height");
        assert!(height > 0);
    }

    #[test]
    async fn get_block() {
        let client = Client::default();
        let block = blocks::get_block(&client, &864203).await.expect("block");
        assert!(block.transactions.len() > 0);
        
    }

}

