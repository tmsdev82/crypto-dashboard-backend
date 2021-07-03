use crate::{utils, Clients};
use serde::{de::DeserializeOwned, Serialize};
use warp::ws::Message;

pub async fn publish_message<T: Serialize>(payload: T, channel: String, clients: &Clients) {
    clients
        .lock()
        .await
        .iter_mut()
        .filter(|(_, client)| client.channels.contains(&channel))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(serde_json::to_string(&payload).unwrap())));
                // let _ = sender.send(Ok(Message::text(body.message.clone())));
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
