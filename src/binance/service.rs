use core::panic;

use crate::{
    binance, crypto_service,
    models::{self, CoinPairData},
};

const BINANCE_API_URL: &str = "https://api.binance.com/api/v3";

pub async fn get_trades_for_pair(
    crypto_pair: &str,
) -> std::result::Result<
    Vec<binance::models::Trade>,
    Box<dyn std::error::Error + Send + Sync + 'static>,
> {
    let root_url = format!("{}/trades", BINANCE_API_URL);
    let query_url = format!("{}?symbol={}&limit=10", root_url, crypto_pair);

    let response: Vec<binance::models::Trade> =
        crypto_service::get_data_from_exchange(&query_url).await?;

    Ok(response)
}

pub async fn get_trades_data_for_pair(
    coin_pair: &str,
) -> models::CoinPairData<Vec<binance::models::Trade>> {
    let exchange_name = "binance";
    let data_type = "trades";
    let trades = get_trades_for_pair(&coin_pair).await.unwrap();

    let trades_data = models::CoinPairData::<Vec<binance::models::Trade>> {
        coin_pair: String::from(coin_pair),
        data_set: trades,
        data_type: String::from(data_type),
        exchange_name: String::from(exchange_name),
    };

    trades_data
}

pub fn raw_to_offer_data(raw: &Vec<[String; 2]>) -> Vec<binance::models::OfferData> {
    raw.iter()
        .map(|item| binance::models::OfferData {
            price: item[0].parse().unwrap(),
            size: item[1].parse().unwrap(),
        })
        .collect()
}

pub async fn get_orderbooks_data_for_pair(
    coin_pair: &str,
) -> models::CoinPairData<binance::models::OrderBookDTO> {
    let exchange_name = "binance";
    let data_type = "orderbooks";
    let result = get_orderbooks_for_pair(&coin_pair).await;
    let orderbooks = match result {
        Ok(v) => v,
        Err(e) => {
            error!("error occurred: {}", e);
            panic!("error occured");
        }
    };

    let orderbooks = binance::models::OrderBookDTO {
        lastUpdateId: orderbooks.lastUpdateId,
        asks: raw_to_offer_data(&orderbooks.asks),
        bids: raw_to_offer_data(&orderbooks.bids),
    };

    let orderbook_data = models::CoinPairData::<binance::models::OrderBookDTO> {
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
    binance::models::RawOrderBook,
    Box<dyn std::error::Error + Send + Sync + 'static>,
> {
    let root_url = format!("{}/depth", BINANCE_API_URL);
    let query_url = format!("{}?symbol={}&limit=50", root_url, crypto_pair);
    let response: binance::models::RawOrderBook =
        crypto_service::get_data_from_exchange(&query_url).await?;

    Ok(response)
}
