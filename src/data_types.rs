use serde::{de, Deserialize, Deserializer, Serialize};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Trades {
    pub coin_pair: String,
    pub trades: Vec<Trade>,
}

fn de_float_from_str<'a, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f32>().map_err(de::Error::custom)
}
