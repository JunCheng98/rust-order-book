use std::{cmp::Ordering, collections::BTreeMap, fmt::Write, ops::Bound::{Included, Unbounded}};

use crate::response::{BookTickerUpdate, DepthUpdate};

use f64 as Quantity;

#[cfg(test)]
mod tests;


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
        log::info!("{:?}", data);
        // skip since the depth update already contains all the latest levels
        if data.last_update_id <= self.last_update_id {
            return;
        }
        // filter out levels that are better than the current best bid/ask
        self.bid_levels = self.bid_levels
            .range((Unbounded, Included(Price{ val: data.bid_price })))
            .map(|(&prc, &qty)| (prc, qty))
            .collect();
        self.ask_levels = self.ask_levels
            .range((Included(Price{ val: data.ask_price }), Unbounded))
            .map(|(&prc, &qty)| (prc, qty))
            .collect();
        // update the best bid/ask qty if the price exists, otherwise insert
        self.bid_levels.insert(Price{ val: data.bid_price }, data.bid_qty);
        self.ask_levels.insert(Price{ val: data.ask_price }, data.ask_qty);
    }
 
    pub fn update_depth(&mut self, data: &DepthUpdate) {
        log::info!("{:?}", data);
        // when applying a snapshot, update local book as follows:
        // 1. Remove any levels better than the best snapshot level
        // 2. Update all levels within the snapshot
        // 3. Keep the levels worse than the last snapshot level
        // if we want to prevent lower levels from becoming stale, use snapshot API periodically
        let mut snapshot_bids = init_price_map(&data.bids);
        let mut snapshot_asks = init_price_map(&data.asks);

        if let Some((worst_bid_prc, _)) = snapshot_bids.first_key_value() {
            self.bid_levels = self.bid_levels
                .range((Unbounded, Included(worst_bid_prc)))
                .map(|(&prc, &qty)| (prc, qty))
                .collect();
        }
        if let Some((worst_ask_prc, _)) = snapshot_asks.last_key_value() {
            self.ask_levels = self.ask_levels
                .range((Included(worst_ask_prc), Unbounded))
                .map(|(&prc, &qty)| (prc, qty))
                .collect();
        }

        self.bid_levels.append(&mut snapshot_bids);
        self.ask_levels.append(&mut snapshot_asks);
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
        const PRINT_MAX_LEVELS: u32 = 20;

        let mut result = String::new();

        let mut bid_iter = self.bid_levels.iter().rev().peekable();
        let mut ask_iter = self.ask_levels.iter().peekable();

        let mut level = 1;
        while bid_iter.peek().is_some() || ask_iter.peek().is_some() {
            if level > PRINT_MAX_LEVELS {
                break;
            }

            let next_bid = bid_iter.next();
            let next_ask = ask_iter.next();

            write!(&mut result, "[{:2}] ", level).unwrap();
            // handle cases where there are uneven levels by padding with empty string
            match next_bid {
                Some((bid_price, bid_qty)) => write!(result, "[ {:>7} ] {:>8} | ", bid_qty, bid_price.val).unwrap(),
                None => result.push_str("                      | "),    
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
