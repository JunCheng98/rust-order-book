use std::collections::{HashMap, BTreeMap};
use serde_json::Value;

use crate::{endpoints, model};

pub async fn init_order_book(query: &str) -> model::OrderBook {
    let depth_snapshot_data = endpoints::depth_snapshot(query).await;
    model::OrderBook{
        last_update_id: depth_snapshot_data.lastUpdateId,
        prev_final_update_id: -1,
        bids_map: init_price_map(&depth_snapshot_data.bids),
        asks_map: init_price_map(&depth_snapshot_data.asks),
    }
}

fn update_price_to_qty(price_to_qty: &Option<&Value>) -> HashMap<String, String> {
    price_to_qty
        .and_then(Value::as_array)
        .unwrap_or(&Vec::new())
        .iter()
        .map(|item| (item[0].as_str().unwrap_or_default().to_owned(), item[1].as_str().unwrap_or_default().to_owned()))
        .collect()
}

fn init_price_map(price_to_qty: &Vec<[String; 2]>) -> BTreeMap<model::FloatString, f64> {
    price_to_qty.iter()
    .map(|item| (model::FloatString{val: item[0].parse::<f64>().unwrap_or_default()}, item[1].parse::<f64>().unwrap_or_default()))
    .collect()
}

// update using the rules listed in the Binance documentation
// https://binance-docs.github.io/apidocs/spot/en/#how-to-manage-a-local-order-book-correctly
pub async fn update_order_book(resp: model::StreamResponse, order_book: &mut model::OrderBook) {
    let depth_diff_data = resp.data;

    let first_update_id = depth_diff_data.get("U").and_then(Value::as_i64).unwrap_or_default();
    let final_update_id = depth_diff_data.get("u").and_then(Value::as_i64).unwrap_or_default();
    // drop any event with u <= lastUpdateId
    if final_update_id <= order_book.last_update_id {
        return
    }
    
    // first processed event should have U <= lastUpdateId+1 <= u, if not we get the snapshot again
    if order_book.prev_final_update_id == -1 && (first_update_id > order_book.last_update_id+1 || final_update_id < order_book.last_update_id+1) {
        let symbol = depth_diff_data.get("s").and_then(Value::as_str).unwrap_or_default();
        let depth_snapshot = endpoints::depth_snapshot(symbol).await;

        order_book.last_update_id = depth_snapshot.lastUpdateId;
        order_book.asks_map = init_price_map(&depth_snapshot.asks);
        order_book.bids_map = init_price_map(&depth_snapshot.bids);
    }

    // new event's U should be prev event's u+1
    if order_book.prev_final_update_id != -1 && order_book.prev_final_update_id + 1 != first_update_id {
        return
    }
    order_book.prev_final_update_id = final_update_id;

    let ask_price_to_qty = update_price_to_qty(&depth_diff_data.get("a"));
    let bid_price_to_qty = update_price_to_qty(&depth_diff_data.get("b"));
    for (p, q) in ask_price_to_qty {
        let price = model::FloatString{val: p.parse::<f64>().unwrap_or_default()};
        let qty = q.parse::<f64>().unwrap_or_default();
        if qty == 0.0 {
            order_book.asks_map.remove(&price);
        } else {
            order_book.asks_map.insert(price, qty);
        }
    }

    for (p, q) in bid_price_to_qty {
        let price = model::FloatString{val: p.parse::<f64>().unwrap_or_default()};
        let qty = q.parse::<f64>().unwrap_or_default();
        if qty == 0.0 {
            order_book.bids_map.remove(&price);
        } else {
            order_book.bids_map.insert(price, qty);
        }
    }
}