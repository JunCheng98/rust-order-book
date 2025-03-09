use serde::{Deserialize, Deserializer};


#[derive(Debug, Deserialize)]
pub struct StreamResponse {
    pub stream: String,
    pub data: StreamData,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StreamData {
    BookTickerUpdateData(BookTickerUpdate),
    DepthUpdateData(DepthUpdate),
}

#[derive(Debug, Deserialize)]
pub struct BookTickerUpdate {
    #[serde(rename = "u")]
    pub last_update_id: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "b", deserialize_with = "str_to_f64")]
    pub bid_price: f64,
    #[serde(rename = "B", deserialize_with = "str_to_f64")]
    pub bid_qty: f64,
    #[serde(rename = "a", deserialize_with = "str_to_f64")]
    pub ask_price: f64,
    #[serde(rename = "A", deserialize_with = "str_to_f64")]
    pub ask_qty: f64
}

#[derive(Debug, Deserialize)]
pub struct DepthUpdate {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,
    pub bids: Vec<[String; 2]>,
    pub asks: Vec<[String; 2]>,
}


fn str_to_f64<'de, D>(des: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(des)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}
