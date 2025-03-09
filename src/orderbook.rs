use std::{cmp::Ordering, collections::BTreeMap, fmt::Write};

use crate::response::{BookTickerUpdate, DepthUpdate};

use f64 as Quantity;


// use custom struct to order by price since f64 does not implement Eq (due to NaN)
#[derive(Debug, Copy, Clone)]
pub struct Price {
    pub val: f64,
}

impl Ord for Price {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for Price {}

#[derive(Debug)]
pub struct OrderBook {
    pub symbol: String,
    pub last_update_id: u64,
    pub bid_levels: BTreeMap<Price, Quantity>,
    pub ask_levels: BTreeMap<Price, Quantity>,
}

impl OrderBook {
    pub fn new(symbol: String) -> Self {
        OrderBook { 
            symbol, 
            last_update_id: 0, 
            bid_levels: BTreeMap::new(), 
            ask_levels: BTreeMap::new(),
        }
    }

    pub fn update_book_ticker(&mut self, data: &BookTickerUpdate) {
        // log::info!("{:?}", data);
        // skip since the depth update already contains all the latest levels
        if data.last_update_id <= self.last_update_id {
            return;
        }
        // for speed, we favour making a copy here over retain(), which scans the entire map
        let filtered_bids: BTreeMap<Price, _> = self.bid_levels
            .iter()
            .filter(|(&prc, _)| prc.val <= data.bid_price)
            .map(|(&prc, &qty)| (prc, qty))
            .collect();
        let filtered_asks: BTreeMap<_, _> = self.ask_levels
            .iter()
            .filter(|(&prc, _)| prc.val >= data.ask_price)
            .map(|(&prc, &qty)| (prc, qty))
            .collect();

        self.bid_levels = filtered_bids;
        self.ask_levels = filtered_asks;
        // update the best bid/ask qty if the price exists, otherwise insert
        self.bid_levels.insert(Price{ val: data.bid_price }, data.bid_qty);
        self.ask_levels.insert(Price{ val: data.ask_price }, data.ask_qty);
    }
 
    pub fn update_depth(&mut self, data: &DepthUpdate) {
        // log::info!("{:?}", data);
        // since this returns the snapshot for the top k levels, just replace the entire orderbook
        self.bid_levels = init_price_map(&data.bids);
        self.ask_levels = init_price_map(&data.asks);
        // when applying a snapshot, update local book as follows:
        // 1. Remove any levels better than the best snapshot level
        // 2. Update all levels within the snapshot
        // 3. Leave the levels worse than the last snapshot level
    }

    // if one side of the book is empty, return None to signal that we should fix the book
    pub fn get_best_bid_ask(&self) -> Option<((f64, f64), (f64, f64))> {
        if let (Some((bid_prc, bid_qty)), Some((ask_prc, ask_qty))) = (self.bid_levels.last_key_value(), self.ask_levels.first_key_value()) {
            Some(((bid_prc.val, *bid_qty), (ask_prc.val, *ask_qty)))
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        let mut bid_iter = self.bid_levels.iter().rev().peekable();
        let mut ask_iter = self.ask_levels.iter().peekable();

        let mut level = 1;
        while bid_iter.peek().is_some() || ask_iter.peek().is_some() {
            let next_bid = bid_iter.next();
            let next_ask = ask_iter.next();

            write!(&mut result, "[{:2}] ", level).unwrap();
            // handle cases where there are uneven levels by padding with empty string
            match next_bid {
                Some((bid_price, bid_qty)) => write!(result, "[ {:>7} ] {:>8} | ", bid_qty, bid_price.val).unwrap(),
                None => result.push_str("           | "),    
            }
            match next_ask {
                Some((ask_price, ask_qty)) => write!(result, "{:<8} [ {:>7} ]\n", ask_price.val, ask_qty).unwrap(),
                None => result.push('\n')
            }

            level += 1;
        }

        return result;
    }
}

pub fn init_price_map(price_to_qty: &Vec<[String; 2]>) -> BTreeMap<Price, Quantity> {
    price_to_qty
        .iter()
        .map(|[prc, qty]| 
            (Price{ val: prc.parse::<f64>().unwrap_or_default() }, qty.parse::<f64>().unwrap_or_default()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticker_worse_price() {

    }

    #[test]
    fn ticker_better_price() {

    }

    #[test]
    fn ticker_same_price() {

    }

    #[test]
    fn ticker_same_price_diff_qty() {

    }

    #[test]
    fn best_level_normal() {

    }

    #[test]
    fn best_level_one_empty() {

    }

    #[test]
    fn best_level_both_empty() {
        
    }
}