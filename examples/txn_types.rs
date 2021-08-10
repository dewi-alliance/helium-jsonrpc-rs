use helium_jsonrpc::{blocks, transactions, Client};

//quick sanity check to make sure all transactions in the last 100 blocks can be serialized
#[tokio::main]
async fn main() {
    let client = Client::new_with_base_url("http://localhost:4467".to_string());
    let mut current_height = blocks::height(&client).await.unwrap();
    let start_height = current_height;
    println!("Height: {}", start_height);

    let mut types: Vec<String> = Vec::new();

    loop {
        let block = match blocks::get_raw(&client, &current_height).await {
            Ok(b) => b,
            Err(e) => panic!("Couldn't get block {}. {}", current_height, e),
        };

        for txn in block.transactions {
            match transactions::get(&client, &txn.hash.to_string()).await {
                Ok(_t) => types.push(txn.r#type),
                Err(e) => println!(
                    "Error getting txn {}. Type: {}, error: {}",
                    txn.hash, txn.r#type, e
                ),
            };
        }

        if start_height - current_height >= 100 {
            break;
        }

        current_height -= 1;
    }

    types.sort();
    types.dedup();

    println!("{:?}", types);
}
