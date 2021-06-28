use crate::{ws, Client, Clients, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, ws::Message, Reply};

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    client_id: usize,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    uri: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    channel: String,
    client_id: Option<usize>,
    message: String,
}

pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {
    let client_id = body.client_id;
    let uuid = Uuid::new_v4().to_simple().to_string();

    register_client(uuid.clone(), client_id, clients).await;

    Ok(json(&RegisterResponse {
        uri: format!("ws://127.0.0.1:8000/ws/{}", uuid),
    }))
}

async fn register_client(id: String, client_id: usize, clients: Clients) {
    clients.lock().await.insert(
        id,
        Client {
            client_id,
            channels: vec![String::from("trades")],
            sender: None,
        },
    );
}

pub async fn unregister_handler(id: String, clients: Clients) -> Result<impl Reply> {
    clients.lock().await.remove(&id);
    Ok(StatusCode::OK)
}

pub async fn ws_handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply> {
    debug!("ws_handler >>> id: {}", id);

    let client = clients.lock().await.get(&id).cloned();
    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, id, clients, c))),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn publish_handler(body: Event, clients: Clients) -> Result<impl Reply> {
    clients
        .lock()
        .await
        .iter_mut()
        .filter(|(_, client)| match body.client_id {
            Some(v) => client.client_id == v,
            None => true,
        })
        .filter(|(_, client)| client.channels.contains(&body.channel))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(serde_json::to_string(&body).unwrap())));
                // let _ = sender.send(Ok(Message::text(body.message.clone())));
            }
        });

    Ok(StatusCode::OK)
}

pub async fn health_handler() -> Result<impl Reply> {
    debug!("health check >>>");
    Ok(StatusCode::OK)
}
