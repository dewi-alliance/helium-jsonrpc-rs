use helium_jsonrpc::{blocks, transactions::Transaction, Client};

//This function will find the first block that blockchain-node has. Should be whatever height the snapshot
//was when the node was first started. May take a while if the node has been running for a long time.
#[tokio::main]
async fn main() {
    let client = Client::new_with_base_url("http://localhost:4467".to_string());
    let first_height = find_first_block(&client).await;

    println!("First block: {}", first_height);
}

async fn find_first_block(client: &Client) -> u64 {
    let mut current_height = blocks::height(&client).await.unwrap();
    let mut last_safe_height = current_height;
    let mut in_last_epoch = false;

    loop {
        let block = match blocks::get(&client, &current_height).await {
            Ok(b) => b,
            Err(_) if in_last_epoch => return last_safe_height,
            Err(_) => {
                in_last_epoch = true;
                current_height = last_safe_height - 1;
                match blocks::get(&client, &current_height).await {
                    Ok(b) => b,
                    Err(e) => panic!("error getting block: {}", current_height),
                }
            }
        };
        block.transactions.iter().for_each(|txn| match txn {
            Transaction::RewardsV2 { start_epoch, .. } => {
                current_height = *start_epoch;
            }
            _ => {
                last_safe_height = current_height;
            }
        });

        current_height -= 1;
    }
}
