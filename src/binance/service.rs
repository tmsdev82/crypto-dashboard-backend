use crate::{binance, ws, Client, Clients, Result};
use reqwest;
use reqwest::header;
use serde::Serialize;

const BINANCE_API_URL: &str = "http://api.binance.com/api/v3";

pub async fn binance_get_trades_for_pair(
    crypto_pair: &str,
) -> std::result::Result<
    Vec<binance::models::Trade>,
    Box<dyn std::error::Error + Send + Sync + 'static>,
> {
    let trade_root_url = format!("{}/trades", BINANCE_API_URL);
    let trades_url = format!("{}?symbol={}&limit=5", trade_root_url, crypto_pair);

    let response = reqwest::get(trades_url)
        .await?
        .json::<Vec<binance::models::Trade>>()
        .await?;

    // response
    Ok(response)
}
