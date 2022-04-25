use crate::*;

#[derive(Deserialize)]
struct PriceCurrentResult {
    height: u64,
    price: Usd,
}



pub mod prices {
    use super::*;
    
    pub async fn current(client: &Client) -> Result<(u64, Usd)> {
        let json = NodeCall::oracle_price_current();
        let url_path = "/";
        let result: PriceCurrentResult = client.post(url_path, &json).await?;
        Ok((result.height, result.price))
    }
}