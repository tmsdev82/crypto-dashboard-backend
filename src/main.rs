use std::{collections::HashMap, convert::Infallible, sync::Arc};

use tokio::sync::{mpsc, Mutex};
use tokio::task;
use tokio::time::Duration;
use warp::{ws::Message, Filter, Rejection};

use pretty_env_logger;
#[macro_use]
extern crate log;

mod handler;
mod workers;
mod ws;

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: usize,
    pub channels: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<Mutex<HashMap<String, Client>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("Server is starting...");

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    let health_route = warp::path!("health").and_then(handler::health_handler);
    let register_root = warp::path("register");
    let register_routes = register_root
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::register_handler)
        .or(register_root
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_clients(clients.clone()))
            .and_then(handler::unregister_handler));

    let publish_route = warp::path!("publish")
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::publish_handler);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(handler::ws_handler);

    let routes = health_route
        .or(register_routes)
        .or(ws_route)
        .or(publish_route)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
