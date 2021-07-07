use crate::{binance::models, binance::service, crypto_service, models as dmodels};

pub async fn triangular_arbitrage() -> Vec<dmodels::TriangleData<models::OfferData>> {
    let triangles = crypto_service::get_triangles();
    let mut triangle_sets: Vec<dmodels::TriangleData<models::OfferData>> = vec![];
    // First get prices for the pairs
    for triangle in triangles.iter() {
        info!("binance triangle arbitrage processing: {:?}", triangle);
        let tr_arr = triangle.triangle.clone();

        let orderbook_a = service::get_orderbooks_for_pair(&triangle.start_pair)
            .await
            .unwrap();
        let orderbook_b = service::get_orderbooks_for_pair(&triangle.mid_pair)
            .await
            .unwrap();
        let orderbook_c = service::get_orderbooks_for_pair(&triangle.end_pair)
            .await
            .unwrap();

        let start_pair = dmodels::TrianglePairData::<models::OfferData> {
            pair: triangle.start_pair.clone(),
            asks: service::raw_to_offer_data(&orderbook_a.asks[..3].to_vec()),
        };
        let mid_pair = dmodels::TrianglePairData::<models::OfferData> {
            pair: triangle.mid_pair.clone(),
            asks: service::raw_to_offer_data(&orderbook_b.asks[..3].to_vec()),
        };
        let end_pair = dmodels::TrianglePairData::<models::OfferData> {
            pair: triangle.end_pair.clone(),
            asks: service::raw_to_offer_data(&orderbook_c.asks[..3].to_vec()),
        };

        let mut profits: Vec<f32> = Vec::new();
        let mut triangle_result: f32;

        for i in 0..start_pair.asks.len() {
            if triangle.start_pair[..3] == tr_arr[0] {
                triangle_result = 1.0 * start_pair.asks[i].price;
            } else {
                triangle_result = 1.0 / start_pair.asks[i].price;
            }

            if triangle.mid_pair[..3] == tr_arr[1] {
                triangle_result = triangle_result * mid_pair.asks[i].price;
            } else {
                triangle_result = triangle_result / mid_pair.asks[i].price;
            }

            if triangle.end_pair[..3] == tr_arr[2] {
                triangle_result = triangle_result * end_pair.asks[i].price;
            } else {
                triangle_result = triangle_result / end_pair.asks[i].price;
            }

            triangle_result = (100.0 * triangle_result) - 100.0;
            profits.push(triangle_result);
        }

        let triangle_data = dmodels::TriangleData {
            triangle: format!("{}-{}-{}", tr_arr[0], tr_arr[1], tr_arr[2]),
            start_pair,
            mid_pair,
            end_pair,
            profits,
        };

        triangle_sets.push(triangle_data);
    }
    triangle_sets
}
