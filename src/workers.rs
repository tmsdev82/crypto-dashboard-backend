use crate::{binance, coinbase, crypto_service, kraken, Clients};
use tokio;
use tokio::time::Duration;

async fn binance_worker(clients: Clients) {
    tokio::task::spawn(async move {
        // let trades_data = binance::service::get_trades_data_for_pair("ETHBTC").await;
        // crypto_service::publish_message(trades_data, String::from("trades"), &clients).await;

        let orderbooks_data = binance::service::get_orderbooks_data_for_pair("ETHBTC").await;
        crypto_service::publish_message(orderbooks_data, String::from("orderbooks"), &clients)
            .await;
    });
}

async fn coinbase_worker(clients: Clients) {
    tokio::task::spawn(async move {
        // let trades_data = coinbase::service::get_trades_data_for_pair("ETH-BTC").await;
        // crypto_service::publish_message(trades_data, String::from("trades"), &clients).await;

        let orderbooks_data = coinbase::service::get_orderbooks_data_for_pair("ETH-BTC").await;
        crypto_service::publish_message(orderbooks_data, String::from("orderbooks"), &clients)
            .await;
    });
}

async fn kraken_worker(clients: Clients) {
    tokio::task::spawn(async move {
        // let trades_data = coinbase::service::get_trades_data_for_pair("ETH-BTC").await;
        // crypto_service::publish_message(trades_data, String::from("trades"), &clients).await;

        let orderbooks_data = kraken::service::get_orderbooks_data_for_pair("ETHBTC").await;
        crypto_service::publish_message(orderbooks_data, String::from("orderbooks"), &clients)
            .await;
    });
}

pub async fn main_worker(clients: Clients) {
    loop {
        tokio::time::sleep(Duration::from_millis(4000)).await;

        // Get trade data

        if clients.lock().await.len() == 0 {
            debug!("No clients connected: skipping data collection");
            continue;
        }

        let clients_b = clients.clone();
        binance_worker(clients_b).await;

        let clients_c = clients.clone();
        coinbase_worker(clients_c).await;

        let clients_k = clients.clone();
        kraken_worker(clients_k).await;
    }
}
