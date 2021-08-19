use helium_jsonrpc::{transactions, Client, Transaction};

#[tokio::main]
async fn main() {
    let client = Client::default();

    let tx = transactions::get(&client, "bDd-yA7MWLDlowyPTC96QEspnqzUKEg6yroL8G9dDQk")
        .await
        .unwrap();
    match tx {
        Transaction::VarsV1 { .. } => println!("VarsV1"),
        _ => println!("Didn't find VarsV1"),
    }

    let state_channel_close =
        transactions::get(&client, "-j71ACV87GyyRylytzfZJlqtkyvSYwJGb8P72UREL6E")
            .await
            .unwrap();
    match state_channel_close {
        Transaction::StateChannelCloseV1 { .. } => println!("StateChannelCloseV1"),
        _ => println!("Didn't find StateChannelCloseV1"),
    }

    let state_channel_open_v1 =
        transactions::get(&client, "I0pTZ1yLLt58Y7ox8MGGEzqyqqDm0An5CFaLrkuz5Ks")
            .await
            .unwrap();
    match state_channel_open_v1 {
        Transaction::StateChannelOpenV1 { .. } => println!("StateChannelOpenV1"),
        _ => println!("Didn't find StateChannelOpenV1"),
    }

    let token_burn_v1 = transactions::get(&client, "5HyVcJIFIDEPQYBmc8OxeGxolkAdt8jfqPZYRsSi7BA")
        .await
        .unwrap();
    match token_burn_v1 {
        Transaction::TokenBurnV1 { .. } => println!("TokenBurnV1"),
        _ => println!("Didn't find TokenBurnV1"),
    }

    let routing_v1 = transactions::get(&client, "D5M1wmQNWQm9DqzatDf3MfNrW9Eh2L6vYjfmVDqoS7M")
        .await
        .unwrap();
    match routing_v1 {
        Transaction::RoutingV1 { .. } => println!("Found RoutingV1"),
        _ => println!("Didn't find RoutingV1"),
    }

    let oui_v1 = transactions::get(&client, "7fl-fxx0FKHDB0owBnxybc5GlkFakbTbUbrmC2pxttY")
        .await
        .unwrap();
    match oui_v1 {
        Transaction::OuiV1 { .. } => println!("OuiV1"),
        _ => println!("Didn't find OuiV1"),
    }
}
