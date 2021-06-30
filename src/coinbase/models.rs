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
