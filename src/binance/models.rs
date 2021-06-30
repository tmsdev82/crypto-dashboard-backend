use crate::utils::de_float_from_str;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderBook {
    lastUpdateId: u64,
    bids: Vec<[String; 2]>,
    asks: Vec<[String; 2]>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    id: u64,
    #[serde(deserialize_with = "de_float_from_str")]
    price: f32,
    #[serde(deserialize_with = "de_float_from_str")]
    qty: f32,
    #[serde(deserialize_with = "de_float_from_str")]
    quoteQty: f32,
    time: u64,
    isBuyerMaker: bool,
    isBestMatch: bool,
}
