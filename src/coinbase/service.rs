use crate::{coinbase, ws, Client, Clients, Result};
use reqwest;
use reqwest::header;
use serde::Serialize;

const COINBASE_API_URL: &str = "http://api.pro.coinbase.com";

pub async fn coinbase_get_trades_for_pair(
    crypto_pair: &str,
) -> std::result::Result<
    Vec<coinbase::models::Trade>,
    Box<dyn std::error::Error + Send + Sync + 'static>,
> {
    let trade_root_url = format!("{}/products/{}/trades", COINBASE_API_URL, crypto_pair);
    let trades_url = format!("{}?limit=5", trade_root_url);

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_static(""));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .user_agent(String::from("testclient"))
        .build()?;

    let response = client
        .get(trades_url)
        .send()
        .await?
        // .json::<serde_json::Value>()
        .json::<Vec<coinbase::models::Trade>>()
        .await?;
    // debug!("{:#?}", response);
    Ok(response)
    // Ok(Vec::new())
}
