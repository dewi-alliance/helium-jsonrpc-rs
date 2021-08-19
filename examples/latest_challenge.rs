use helium_jsonrpc::{blocks, transactions, Client, Transaction};

#[tokio::main]
async fn main() {
    let gateway = "11xzD6yrWF2e3oZcLLD6GhjZS7seFoDrG85xqHGxAUUgy4SZCRb";

    let client = Client::new_with_base_url("http://localhost:4467".to_string());

    let mut current_height = blocks::height(&client).await.unwrap();

    loop {
        let block_raw = match blocks::get_raw(&client, &current_height).await {
            Ok(b) => b,
            Err(_) => {
                panic!("Didn't find challenge..")
            }
        };

        let txns = block_raw.transactions;

        for txn in txns.iter() {
            let _tx = match transactions::get(&client, &txn.hash).await {
                Ok(tx) => match tx {
                    Transaction::PocRequestV1 { challenger, .. } => {
                        if challenger == gateway {
                            println!(
                                "Most recent challenge issued at block {}. tx {}",
                                current_height, txn.hash
                            );
                            return;
                        }
                        Ok(())
                    }
                    _ => Ok(()),
                },
                Err(e) => {
                    println!("Error with txn: {}: {:?}", txn.hash, e);
                    Err(e)
                }
            };
        }
        current_height -= 1;
    }
}
