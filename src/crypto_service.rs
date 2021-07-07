use crate::{models, utils, Clients};
use serde::{de::DeserializeOwned, Serialize};
use warp::ws::Message;

pub fn get_triangles() -> Vec<models::TriangleRequest> {
    let triangles = vec![
        models::TriangleRequest {
            triangle: [
                String::from("BTC"),
                String::from("ETH"),
                String::from("LTC"),
            ],
            start_pair: String::from("ETHBTC"),
            mid_pair: String::from("LTCETH"),
            end_pair: String::from("LTCBTC"),
        },
        models::TriangleRequest {
            triangle: [
                String::from("BTC"),
                String::from("XRP"),
                String::from("BNB"),
            ],
            start_pair: String::from("XRPBTC"),
            mid_pair: String::from("XRPBNB"),
            end_pair: String::from("BNBBTC"),
        },
    ];

    return triangles;
}

pub async fn publish_message<T: Serialize>(payload: T, channel: String, clients: &Clients) {
    clients
        .lock()
        .await
        .iter_mut()
        .filter(|(_, client)| client.channels.contains(&channel))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(serde_json::to_string(&payload).unwrap())));
            }
        });
}

pub async fn get_data_from_exchange<T: DeserializeOwned>(
    api_endpoint_url: &str,
) -> std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let client = utils::get_req_client();

    let response = client
        .get(api_endpoint_url)
        .send()
        .await?
        .json::<T>()
        .await?;
    Ok(response)
}
