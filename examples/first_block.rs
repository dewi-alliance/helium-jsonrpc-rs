
use helium_jsonrpc_rs::{ Client, blocks, transactions, transactions::Transaction };


//This function will find the first block that blockchain-node has. Should be whatever height the snapshot 
//was when the node was first started. May take a while if the node has been running for a long time.
#[tokio::main]
async fn main() {
	let client = helium_jsonrpc_rs::Client::new_with_base_url("http://192.168.2.23:4467".to_string());
	let first_height = find_first_block(&client).await;

	println!("First block: {}", first_height);

}

async fn find_first_block(client: &Client) -> u64 {
	let mut current_height = blocks::height(&client).await.unwrap();
	let mut last_safe_height = current_height; 
	let mut in_last_epoch = false;

	loop {
		let block = match blocks::get_block(&client, &current_height).await {
			Ok(b) => b,
			Err(_) => {
				if in_last_epoch { 
					return last_safe_height;
				}
				in_last_epoch = true;
				current_height = last_safe_height-1;
				blocks::get_block(&client, &current_height).await.unwrap()
			},
		};
		let txns = block.transactions;

		for tx_hash in txns.iter() {
			let _tx = match transactions::get_transaction(&client, tx_hash).await {
				Ok(tx) => match tx {
					Transaction::RewardsV2{ start_epoch, .. } => {
						current_height = start_epoch;
						Ok(())
					},
					_ =>{
						last_safe_height = current_height;
						Ok(())
					},
				},
				Err(e) => {
					println!("Error with txn: {}: {:?}", tx_hash, e);
					Err(e)
				},
			};
		}
		current_height -= 1;
	}
}
