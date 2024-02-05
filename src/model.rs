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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StreamResponse {
    pub stream: String,
    pub data: HashMap<String, Value>,
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
    pub last_update_id: i64,
    pub prev_final_update_id: i64,
    pub bids_map: BTreeMap<FloatString, f64>,
    pub asks_map: BTreeMap<FloatString, f64>,
}
