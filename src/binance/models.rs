use crate::utils::de_float_from_str;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RawOrderBook {
    pub lastUpdateId: u64,
    pub bids: Vec<[String; 2]>,
    pub asks: Vec<[String; 2]>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OfferData {
    pub price: f32,
    pub size: f32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderBookDTO {
    pub lastUpdateId: u64,
    pub bids: Vec<OfferData>,
    pub asks: Vec<OfferData>,
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
