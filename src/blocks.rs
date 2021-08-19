use crate::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use transactions::Transaction;

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Represents a block response from blockchain-node.
pub struct BlockRaw {
    pub height: u64,
    pub hash: String,
    pub prev_hash: String,
    pub time: u64,
    pub transactions: Vec<BlockTransaction>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Represents transaction in 'block_height' response from blockchain-node
pub struct BlockTransaction {
    pub hash: String,
    pub r#type: String,
}

impl BlockRaw {
    /// Returns all the transactions in this block
    pub async fn get_transactions(&self, client: &Client) -> Result<Vec<Transaction>> {
        let mut txns: Vec<Transaction> = Vec::new();
        for txn in &self.transactions {
            txns.push(transactions::get(client, &txn.hash).await?);
        }
        Ok(txns)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Represents a complete block with all complete transactions
pub struct Block {
    pub height: u64,
    pub hash: String,
    pub prev_hash: String,
    pub time: u64,
    pub transactions: Vec<Transaction>,
}

/// Get the current height of the blockchain
pub async fn height(client: &Client) -> Result<u64> {
    let json = json!(NodeCall::height());
    client.post("/", &json).await
}

/// Gets a full block (with complete transactions) at a specific block height.
pub async fn get(client: &Client, height: &u64) -> Result<Block> {
    let raw = get_raw(client, height).await?;
    let txns = raw.get_transactions(client).await?;
    Ok(Block {
        height: raw.height.to_owned(),
        hash: raw.hash.to_owned(),
        prev_hash: raw.prev_hash.to_owned(),
        time: raw.time.to_owned(),
        transactions: txns,
    })
}

/// Gets the raw block data (without full transaction) at a specific block
/// height.
pub async fn get_raw(client: &Client, height: &u64) -> Result<BlockRaw> {
    let json = json!(NodeCall::block(*height));
    let url_path = "/";
    client.post(url_path, &json).await
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
        let block = blocks::get(&client, &864203).await.expect("block");
        assert!(block.transactions.len() > 0);
    }
}
