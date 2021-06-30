use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Trades<T> {
    pub coin_pair: String,
    pub exchange_name: String,
    pub trades: Vec<T>,
}
