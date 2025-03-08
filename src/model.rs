use std::collections::{HashMap, BTreeMap};
use std::cmp::Ordering;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DepthSnapshotResponseData {
    pub lastUpdateId: i64,
    pub bids: Vec<[String; 2]>, // price, quantity
    pub asks: Vec<[String; 2]>,
}

#[derive(Debug, Default, Deserialize)]
pub struct StreamResponse {
    pub stream: String,
    pub data: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub enum StreamData {
    BookTickerUpdate,
    DepthUpdate,
}

#[derive(Debug, Deserialize)]
pub struct BookTickerUpdate {
    #[serde(rename = "u")]
    lastUpdateId: u64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "b")]
    bidPrice: f64,
    #[serde(rename = "B")]
    bidQty: f64,
    #[serde(rename = "a")]
    askPrice: f64,
    #[serde(rename = "A")]
    askQty: f64
}

#[derive(Debug, Deserialize)]
pub struct DepthUpdate {
    bids: Vec<[String; 2]>,
    asks: Vec<[String; 2]>,
    lastUpdateId: u64,
}


// using custom struct to do ordering since f64 does not implement Eq (due to NaN)
#[derive(Debug, Copy, Clone)]
pub struct FloatString {
    pub val: f64,
}

impl Ord for FloatString {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for FloatString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl PartialEq for FloatString {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for FloatString {}

#[derive(Debug)]
pub struct OrderBook {
    pub symbol: String,
    pub last_update_id: i64,
    pub prev_final_update_id: i64,
    pub bids_map: BTreeMap<FloatString, f64>,
    pub asks_map: BTreeMap<FloatString, f64>,
}

impl OrderBook {
    pub fn new(symbol: String) -> Self {
        OrderBook { 
            symbol, 
            last_update_id: -1, 
            prev_final_update_id: -1, 
            bids_map: BTreeMap::new(), 
            asks_map: BTreeMap::new(),
        }
    }

    pub fn update_book_ticker(&mut self, data: &BookTickerUpdate) {

    }

    pub fn update_depth(&mut self, data: &DepthUpdate) {

    }

    pub fn get_best_bid_ask(&self) -> Option<((f64, f64), (f64, f64))> {
        return Some(((0.0, 0.0), (0.0, 0.0)));
    }

    pub fn to_string(&self) -> String {
        return "".to_string();
    }
}
