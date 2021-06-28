use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use serde::Deserialize;
use serde_json::from_str;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

#[derive(Deserialize, Debug)]
pub struct ChannelsRequest {
    channels: Vec<String>,
}

pub async fn client_connection(ws: WebSocket, id: String, clients: Clients, mut client: Client) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            error!("error sending websocket msg: {}", e);
        }
    }));

    client.sender = Some(client_sender);
    clients.lock().await.insert(id.clone(), client);

    info!("connected {}", id);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!("error receiving message for id {}): {}", id.clone(), e);
                break;
            }
        };
        client_msg(&id, msg, &clients).await;
    }

    clients.lock().await.remove(&id);
    info!("{} disconnected", id);
}

async fn client_msg(id: &str, msg: Message, clients: &Clients) {
    info!("received message froom {}: {:?}", id, msg);

    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    if message == "ping" || message == "ping\n" {
        return;
    }

    let channel_req: ChannelsRequest = match from_str(&message) {
        Ok(v) => v,
        Err(e) => {
            error!("error while parsing message to channel request: {}", e);
            return;
        }
    };

    let mut locked = clients.lock().await;
    match locked.get_mut(id) {
        Some(v) => {
            v.channels = channel_req.channels;
        }
        None => return,
    };
}
