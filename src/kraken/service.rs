use core::panic;

use crate::{
    crypto_service, kraken,
    models::{self, CoinPairData},
};

const KRAKEN_API_URL: &str = "https://api.kraken.com/0/public";

fn raw_to_offer_data(raw: &Vec<(String, String, u32)>) -> Vec<kraken::models::OfferData> {
    raw.iter()
        .map(|item| kraken::models::OfferData {
            price: item.0.parse().unwrap(),
            size: item.1.parse().unwrap(),
        })
        .collect()
}

pub async fn get_orderbooks_data_for_pair(
    coin_pair: &str,
) -> models::CoinPairData<kraken::models::OrderBookDTO> {
    let exchange_name = "kraken";
    let data_type = "orderbooks";
    let result = get_orderbooks_for_pair(&coin_pair).await;
    let orderbooks = match result {
        Ok(v) => v,
        Err(e) => {
            error!("error occurred: {}", e);
            panic!("error occured");
        }
    };

    let keys: Vec<String> = orderbooks.result.keys().cloned().collect();
    let key: String = String::from(&keys[0]);
    let orderbooks = kraken::models::OrderBookDTO {
        bids: raw_to_offer_data(&orderbooks.result.get(&key).unwrap().bids),
        asks: raw_to_offer_data(&orderbooks.result.get(&key).unwrap().asks),
    };

    let orderbook_data = models::CoinPairData::<kraken::models::OrderBookDTO> {
        coin_pair: String::from(coin_pair),
        data_set: orderbooks,
        data_type: String::from(data_type),
        exchange_name: String::from(exchange_name),
    };

    orderbook_data
}

pub async fn get_orderbooks_for_pair(
    crypto_pair: &str,
) -> std::result::Result<
    kraken::models::RawOrderBook,
    Box<dyn std::error::Error + Send + Sync + 'static>,
> {
    let root_url = format!("{}/Depth", KRAKEN_API_URL);
    let query_url = format!("{}?pair={}&count=50", root_url, crypto_pair);
    let response: kraken::models::RawOrderBook =
        crypto_service::get_data_from_exchange(&query_url).await?;

    Ok(response)
}
