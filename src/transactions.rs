use crate::*;
use serde::{Deserialize, Serialize};
use serde_json::json;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PaymentV2Payment {
    amount: u64,
    memo: Option<String>,
    payee: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Witness {
    channel: u8,
    datarate: String,
    frequency: f64,
    gateway: String,
    is_valid: bool,
    packet_hash: String,
    signal: i64,
    snr: f64,
    timestamp: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Receipt {
    channel: u8,
    data: String,
    datarate: Option<String>,
    frequency: f64,
    gateway: String,
    origin: String,
    signal: i64,
    snr: f64,
    timestamp: u64,

}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PathElement {
    challengee: String,
    receipt: Option<Receipt>,
    witnesses: Vec<Witness>,

}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Reward{
    account: Option<String>,
    amount: u64,
    gateway: Option<String>,
    r#type: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
/// Represents a transaction response from blockchain-node.
pub enum Transaction {
	PocRequestV1 {
        hash: String, 
        block_hash: String,
        challenger: String,
        fee: u64,
        onion_key_hash: String,
        secret_hash: String,
        version: u64 },
    PaymentV2 {
        hash: String,
        fee: u64,
        nonce: u64,
        payer: String,
        payments: Vec<PaymentV2Payment>,        
    },
    PocReceiptsV1 {
        hash: String,
        challenger: String,
        fee: u64,
        onion_key_hash: String,
        path: Vec<PathElement>,
        request_block_hash: String,
        secret: String,
    },
    PaymentV1 {
        hash: String,
        amount: u64,
        fee: u64,
        nonce: u64,
        payer: String,
        payee: String,
    },
    RewardsV2 {
        hash: String,
        start_epoch: u64,
        end_epoch: u64,
        rewards: Vec<Reward>,
    },
    AssertLocationV2 {
        hash: String,
        fee: u64,
        gain: i64,
        nonce: u64,
        owner: String,
        payer: Option<String>,
        gateway: String,
        location: String,
        elevation: i64,
        staking_fee: u64,
    },
    AssertLocationV1 {
        hash: String,
        fee: u64,
        nonce: u64,
        owner: String,
        payer: Option<String>,
        gateway: String,
        location: String,
        staking_fee: u64,
    },
    AddGatewayV1 {
        hash: String,
        fee: u64,
        owner: String,
        payer: String,
        gateway: String,
        staking_fee: u64,
    },
    TransferHotspotV1 {
        hash: String,
        fee: u64,
        buyer: String,
        seller: String,
        gateway: String,
        buyer_nonce: u64,
        amount_to_seller: u64,
    },
    PriceOracleV1 {
        hash: String,
        fee: u64,
        price: u64,
    },
    #[serde(other)]
    Other,
}



pub async fn get_transaction(client: &Client, hash: &str) -> Result<Transaction> {
		let json = json!(NodeCall::transaction(hash.to_string()));
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
    async fn txn() {
        let client = Client::default();
        let txn = transactions::get_transaction(&client, 
        	"1gidN7e6OKn405Fru_0sGhsqca3lTsrfGKrM4dwM_E8")
        .await.expect("PocRequestV1");
        match txn {
            Transaction::PocRequestV1{ block_hash, .. } => assert_eq!(block_hash,"RS2mBvd_4pbKCglkkyMroDQekPNO0xDdYx6Te3HGDGg" ),
            _ => (),
        }
    }
    #[test]
    async fn payment_v2() {
        //dosqfzzaYtoGx278w4Xu5dnYt7aSZIkD1-IbtiiLQQM
        let client = Client::default();
        let txn = transactions::get_transaction(&client, 
            "C_jJZLKBOv_gRQ6P6wEpZPiRVAjf44FOx1iHOFD4haA")
        .await.expect("PaymentV2");
        match txn {
            Transaction::PaymentV2{ payments, .. } => assert_eq!(payments.len(), 1),
            _ => (),
        }
    }
    #[test]
    async fn poc_receipts_v1() {
        let client = Client::default();
        let txn = transactions::get_transaction(&client, 
            "8RaF-G4pvMVuIXfBYhdqNuIlFSEHPm_rC8TH-h4JYdE")
        .await.expect("PocReceipt");
        match txn {
            Transaction::PocReceiptsV1{ hash, .. } => assert_eq!(hash, "8RaF-G4pvMVuIXfBYhdqNuIlFSEHPm_rC8TH-h4JYdE"),
            _ => (),
        }
    }
    #[test]
    async fn payment_v1() {
        let client = Client::default();
        let txn = transactions::get_transaction(&client, 
            "iMSckt_hUcMFY_d7W-QOupY0MGq_g3-CC2dq3P-HWIw")
        .await.expect("PaymentV1");
        match txn {
            Transaction::PaymentV1{ payee, .. } => assert_eq!(payee, "14YeKFGXE23yAdACj6hu5NWEcYzzKxptYbm5jHgzw9A1P1UQfMv" ),
            _ => (),
        }
    }
    #[test]
    async fn rewards_v2() {
        let client = Client::default();
        let txn = transactions::get_transaction(&client,
            "X0HNRGZ1HAX51CR8qS6LTopAosjFkuaaKXl850IpNDE")
        .await.expect("RewardsV2");
        match txn {
            Transaction::RewardsV2{ rewards, .. } => assert_eq!(rewards.len(), 10138 ),
            _ => (),
        }
    }
    #[test]
    async fn assert_location_v1() {
        let client = Client::default();
        let txn = transactions::get_transaction(&client,
            "_I16bycHeltuOo7eyqa4uhv2Bc7awcztZflyvRkVZ24")
        .await.expect("AssertLocationV1");
        match txn {
            Transaction::AssertLocationV1{ hash, .. } => assert_eq!(hash, "_I16bycHeltuOo7eyqa4uhv2Bc7awcztZflyvRkVZ24"),
            _ => (),
        }
    }
    #[test]
    async fn assert_location_v2() {
        let client = Client::default();
        let txn = transactions::get_transaction(&client,
            "TfjRv733Q9FBQ1_unw1c9g5ewVmMBuyf7APuyxKEqrw")
        .await.expect("AssertLocationV2");
        match txn {
            Transaction::AssertLocationV2{ gateway, .. } => assert_eq!(gateway, "112WVxXCrCjiKmmDXLDUJuhYGEHMbXobUZe8oJQkHoMHEFa149a"),
            _ => (),
        }
    }
    #[test]
    async fn add_gateway_v1() {
        let client = Client::default();
        let txn = transactions::get_transaction(&client,
            "aoTggHSgaBAamuUUrXnY42jDZ5WUBxE0k-tshvfn35E")
        .await.expect("AddGatewayV1");
        match txn {
            Transaction::AddGatewayV1{ gateway, .. } => assert_eq!(gateway, "112uuvztDziVQyLVvBxMsovsSPV5ZXkN6uQ5hrWSaWwV1oEZTZtd"),
            _ => (),
        }
    }
    #[test]
    async fn transfer_hotspot_v1() {
        let client = Client::default();
        let txn = transactions::get_transaction(&client,
            "fSFua7A8G41K05QXAvJi5N2OB0QqmQ7xp7u-My4rYHc")
        .await.expect("TransferHotspotV1");
        match txn {
            Transaction::TransferHotspotV1{  seller, .. } => assert_eq!(seller, "14mo9fFGKYFaWh7xscpDLg7misWcuU5xqR8mc8gHr4c43nDnzeX"),
            _ => (),
        }
    }


}