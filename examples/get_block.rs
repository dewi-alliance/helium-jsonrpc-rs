use helium_jsonrpc_rs::{ blocks };

#[tokio::main]
async fn main() {
	let height = 873465;
	let client = helium_jsonrpc_rs::Client::new_with_base_url("http://192.168.1.12:4467".to_string());
	let block = match blocks::get_block(&client, &height).await {
		Ok(b) => b, 
		Err(e) => panic!("Couldn't get block: {}", e),
	};
	println!("Found block {} with {} transactions.", height, block.transactions.len());
}
