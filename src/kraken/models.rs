use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderBookResult {
    pub bids: Vec<(String, String, u32)>,
    pub asks: Vec<(String, String, u32)>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RawOrderBook {
    pub error: Vec<String>,
    pub result: HashMap<String, OrderBookResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OfferData {
    pub price: f32,
    pub size: f32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderBookDTO {
    pub bids: Vec<OfferData>,
    pub asks: Vec<OfferData>,
}
