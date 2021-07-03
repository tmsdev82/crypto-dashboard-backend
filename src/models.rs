use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinPairData<T> {
    pub coin_pair: String,
    pub data_type: String,
    pub exchange_name: String,
    pub data_set: T,
}
