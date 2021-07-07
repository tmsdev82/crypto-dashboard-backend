use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinPairData<T> {
    pub coin_pair: String,
    pub data_type: String,
    pub exchange_name: String,
    pub data_set: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TriangleRequest {
    pub triangle: [String; 3],
    pub start_pair: String,
    pub mid_pair: String,
    pub end_pair: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrianglePairData<T> {
    pub pair: String,
    pub asks: Vec<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TriangleData<T> {
    pub triangle: String,
    pub start_pair: TrianglePairData<T>,
    pub mid_pair: TrianglePairData<T>,
    pub end_pair: TrianglePairData<T>,
    pub profits: Vec<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TriangleSetsData<T> {
    pub data_type: String,
    pub exchange_name: String,
    pub triangle_sets: Vec<TriangleData<T>>,
}
