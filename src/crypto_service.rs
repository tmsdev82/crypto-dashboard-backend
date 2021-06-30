use crate::Clients;
use serde::Serialize;
use warp::ws::Message;

pub async fn publish_message<T: Serialize>(payload: T, channel: String, clients: Clients) {
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
