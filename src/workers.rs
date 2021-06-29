use crate::{crypto_service, data_types, Clients};
use reqwest;
use tokio;
use tokio::time::Duration;

const API_URL: &str = "http://api.binance.com/api/v3";

async fn get_trades_for_pair(
    crypto_pair: &str,
) -> Result<Vec<data_types::Trade>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let trade_root_url = format!("{}/trades", API_URL);
    let trades_url = format!("{}?symbol={}&limit=5", trade_root_url, crypto_pair);

    let response = reqwest::get(trades_url)
        .await?
        .json::<Vec<data_types::Trade>>()
        .await?;

    // response
    Ok(response)
}

pub async fn main_worker(clients: Clients) {
    loop {
        tokio::time::sleep(Duration::from_millis(4000)).await;

        // Get trade data
        let clients = clients.clone();
        {
            if clients.lock().await.len() > 0 {
                tokio::task::spawn(async move {
                    let coin_pair = String::from("ETHBTC");
                    let trades = get_trades_for_pair(&coin_pair).await;
                    let trades = match trades {
                        Ok(v) => v,
                        Err(error) => {
                            error!("get trades -> error occured: {}", error);
                            return;
                        }
                    };

                    let trades_data = data_types::Trades { coin_pair, trades };

                    crypto_service::publish_message(trades_data, String::from("trades"), clients)
                        .await;
                });
            } else {
                debug!("No clients connected: skipping data collection");
            }
        }
    }
}
