# helium-jsonrpc-client

An async library for Helium's [blockchain-node](https://github.com/helium/blockchain-node) using JSON-RPC calls.

## Overview

It is part of the ETL Lite project for tracking and storing data from the Helium blockchain.

## Example


```rust,no-run
use helium_jsonrpc_rs::{ blocks };

#[tokio::main]
async fn main() {
	let height = 873465;
	let client = helium_jsonrpc_rs::Client::new_with_base_url("http://localhost:4467".to_string());
	let block = match blocks::get_block(&client, &height).await {
		Ok(b) => b, 
		Err(e) => panic!("Couldn't get block: {}", e),
	};

	println!("Found block {} with {} transactions.", height, block.transactions.len());
}
```

See the examples folder and unit tests for more examples.
