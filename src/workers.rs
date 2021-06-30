use crate::{binance, coinbase, crypto_service, models, Clients};
use tokio;
use tokio::time::Duration;

pub async fn main_worker(clients: Clients) {
    loop {
        tokio::time::sleep(Duration::from_millis(4000)).await;

        // Get trade data

        if clients.lock().await.len() == 0 {
            debug!("No clients connected: skipping data collection");
            continue;
        }

        let clients_b = clients.clone();
        tokio::task::spawn(async move {
            let coin_pair = String::from("ETHBTC");
            let exchange_name = String::from("binance");
            let trades = binance::service::binance_get_trades_for_pair(&coin_pair).await;
            let trades = match trades {
                Ok(v) => v,
                Err(error) => {
                    error!("get trades -> error occured: {}", error);
                    return;
                }
            };

            let trades_data = models::Trades::<binance::models::Trade> {
                coin_pair,
                trades,
                exchange_name,
            };

            crypto_service::publish_message(trades_data, String::from("trades"), clients_b).await;
        });

        let clients_c = clients.clone();
        tokio::task::spawn(async move {
            let coin_pair = String::from("ETH-BTC");
            let exchange_name = String::from("coinbase");
            let trades = coinbase::service::coinbase_get_trades_for_pair(&coin_pair).await;
            let trades = match trades {
                Ok(v) => v,
                Err(error) => {
                    error!("get trades -> error occured: {}", error);
                    return;
                }
            };

            let trades_data = models::Trades::<coinbase::models::Trade> {
                coin_pair,
                trades,
                exchange_name,
            };

            crypto_service::publish_message(trades_data, String::from("trades"), clients_c).await;
        });
    }
}
