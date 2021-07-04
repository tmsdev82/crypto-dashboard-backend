use crate::{coinbase, crypto_service, models};

const COINBASE_API_URL: &str = "http://api.pro.coinbase.com";

pub async fn get_trades_data_for_pair(
    coin_pair: &str,
) -> models::CoinPairData<Vec<coinbase::models::Trade>> {
    let exchange_name = "coinbase";
    let data_type = "trades";
    let trades = get_trades_for_pair(&coin_pair).await.unwrap();

    let trades_data = models::CoinPairData::<Vec<coinbase::models::Trade>> {
        coin_pair: String::from(coin_pair),
        data_set: trades,
        data_type: String::from(data_type),
        exchange_name: String::from(exchange_name),
    };

    trades_data
}

pub async fn get_trades_for_pair(
    crypto_pair: &str,
) -> std::result::Result<
    Vec<coinbase::models::Trade>,
    Box<dyn std::error::Error + Send + Sync + 'static>,
> {
    let root_url = format!("{}/products/{}/trades", COINBASE_API_URL, crypto_pair);
    let query_url = format!("{}?limit=10", root_url);

    let response = crypto_service::get_data_from_exchange(&query_url).await?;
    Ok(response)
}

fn raw_offer_data(raw: &Vec<(String, String, u32)>) -> Vec<coinbase::models::OfferData> {
    raw.iter()
        .map(|item| coinbase::models::OfferData {
            price: item.0.parse().unwrap(),
            size: item.1.parse().unwrap(),
            num_orders: item.2,
        })
        .collect()
}

pub async fn get_orderbooks_data_for_pair(
    coin_pair: &str,
) -> models::CoinPairData<coinbase::models::OrderBookDTO> {
    let exchange_name = "coinbase";
    let data_type = "orderbooks";
    let orderbooks = get_orderbooks_for_pair(&coin_pair).await.unwrap();

    let orderbooks = coinbase::models::OrderBookDTO {
        sequence: orderbooks.sequence,
        asks: raw_offer_data(&orderbooks.asks),
        bids: raw_offer_data(&orderbooks.bids),
    };

    let orderbook_data = models::CoinPairData::<coinbase::models::OrderBookDTO> {
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
    coinbase::models::RawOrderBook,
    Box<dyn std::error::Error + Send + Sync + 'static>,
> {
    let root_url = format!("{}/products/{}/book", COINBASE_API_URL, crypto_pair);
    let query_url = format!("{}?level=2", root_url);

    let response: coinbase::models::RawOrderBook =
        crypto_service::get_data_from_exchange(&query_url).await?;
    Ok(response)
}
