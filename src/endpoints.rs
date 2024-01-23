use binance_spot_connector_rust::{
    hyper::BinanceHttpClient,
    market,
};
use crate::model;

pub async fn depth_snapshot(query: &str) -> model::DepthSnapshotResponseData {
    let client = BinanceHttpClient::default();
    let req = market::depth(query).limit(500);
    let data = client
        .send(req)
        .await
        .expect("Request failed")
        .into_body_str()
        .await
        .expect("Failed to read response body");

    serde_json::from_str(&data).unwrap_or_default()
}