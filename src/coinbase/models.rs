use crate::utils::de_float_from_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    trade_id: u64,
    #[serde(deserialize_with = "de_float_from_str")]
    price: f32,
    #[serde(deserialize_with = "de_float_from_str")]
    size: f32,
    time: String,
    side: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawOrderBook {
    pub sequence: u64,
    pub asks: Vec<(String, String, u32)>,
    pub bids: Vec<(String, String, u32)>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OfferData {
    pub price: f32,
    pub size: f32,
    pub num_orders: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderBookDTO {
    pub sequence: u64,
    pub asks: Vec<OfferData>,
    pub bids: Vec<OfferData>,
}
