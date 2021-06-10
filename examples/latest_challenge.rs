
use helium_jsonrpc_rs::{ Client, blocks, transactions, transactions::Transaction };

#[tokio::main]
async fn main() {

	let gateway = "11xzD6yrWF2e3oZcLLD6GhjZS7seFoDrG85xqHGxAUUgy4SZCRb";

	let client = helium_jsonrpc_rs::Client::new_with_base_url("http://192.168.1.12:4467".to_string());

	let mut current_height = blocks::height(&client).await.unwrap();

	loop {
		let block = match blocks::get_block(&client, &current_height).await {
			Ok(b) => b,
			Err(_) => {
				panic!("Didn't find challenge..")
			},
		};

		let txns = block.transactions;

		for tx_hash in txns.iter() {
			let _tx = match transactions::get_transaction(&client, tx_hash).await {
				Ok(tx) => match tx {
					Transaction::PocRequestV1{ challenger, .. } => {
						if challenger == gateway {
							println!("Most recent challenge issued at block {}. tx {}", current_height, tx_hash);
							return
						}
						Ok(())
					},
					_ =>{
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
