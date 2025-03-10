use super::*;

fn init_test_orderbook() -> OrderBook {
    let bids: BTreeMap<Price, Quantity> = 
        [(Price{ val: 10.0 }, 0.05), (Price{ val: 9.9 }, 0.011), (Price{ val: 9.7 }, 0.3)].into();
    let asks: BTreeMap<Price, Quantity> = 
        [(Price{ val: 10.5 }, 0.08), (Price{ val: 11.0 }, 0.21), (Price{ val: 11.1 }, 0.06)].into();

    OrderBook { 
        symbol: "BTCUSDT".to_string(), 
        last_update_id: 0, 
        bid_levels: bids, 
        ask_levels: asks
    }
}

#[test]
fn depth_worse_price() {
    let mut order_book = init_test_orderbook();
    let worse_depth = DepthUpdate{
        last_update_id: 1,
        bids: vec![
            ["9.9".to_string(), "0.013".to_string()],
            ["9.85".to_string(), "0.43".to_string()],
            ["9.7".to_string(), "0.3".to_string()]
        ],
        asks: vec![
            ["10.7".to_string(), "0.278".to_string()],
            ["11.0".to_string(), "0.21".to_string()],
            ["11.1".to_string(), "0.59".to_string()]
        ],
    };

    order_book.update_depth(&worse_depth);

    assert_eq!(order_book.bid_levels.last_key_value(), Some((&Price{ val: 9.9 }, &0.013)));
    assert_eq!(order_book.ask_levels.first_key_value(), Some((&Price{ val: 10.7 }, &0.278)));

    assert_eq!(order_book.bid_levels.get(&Price{ val: 9.7 }), Some(&0.3));
    assert_eq!(order_book.ask_levels.get(&Price{ val: 11.1 }), Some(&0.59))
}

#[test]
fn depth_better_price() {
    let mut order_book = init_test_orderbook();
    let worse_depth = DepthUpdate{
        last_update_id: 1,
        bids: vec![
            ["10.1".to_string(), "0.71".to_string()],
            ["9.9".to_string(), "0.011".to_string()],
            ["9.7".to_string(), "0.3".to_string()]
        ],
        asks: vec![
            ["10.3".to_string(), "0.86".to_string()],
            ["10.5".to_string(), "0.08".to_string()],
            ["11.0".to_string(), "0.5".to_string()]
        ],
    };

    order_book.update_depth(&worse_depth);

    assert_eq!(order_book.bid_levels.last_key_value(), Some((&Price{ val: 10.1 }, &0.71)));
    assert_eq!(order_book.ask_levels.first_key_value(), Some((&Price{ val: 10.3 }, &0.86)));

    assert_eq!(order_book.bid_levels.get(&Price{ val: 9.9 }), Some(&0.011));
    assert_eq!(order_book.ask_levels.get(&Price{ val: 11.1 }), Some(&0.06))
}

#[test]
fn ticker_worse_price() {
    let mut order_book = init_test_orderbook();
    let worse_bbo = BookTickerUpdate{
        last_update_id: 1,
        bid_price: 9.8,
        bid_qty: 0.03,
        ask_price: 10.7,
        ask_qty: 0.22,
        symbol: "BTCUSDT".to_string(),
    };

    order_book.update_book_ticker(&worse_bbo);

    assert_eq!(order_book.bid_levels.last_key_value(), Some((&Price{ val: 9.8 }, &0.03)));
    assert_eq!(order_book.ask_levels.first_key_value(), Some((&Price{ val: 10.7 }, &0.22)));
}

#[test]
fn ticker_better_price() {
    let mut order_book = init_test_orderbook();
    let worse_bbo = BookTickerUpdate{
        last_update_id: 1,
        bid_price: 10.2,
        bid_qty: 0.13,
        ask_price: 10.4,
        ask_qty: 0.11,
        symbol: "BTCUSDT".to_string(),
    };

    order_book.update_book_ticker(&worse_bbo);

    assert_eq!(order_book.bid_levels.last_key_value(), Some((&Price{ val: 10.2 }, &0.13)));
    assert_eq!(order_book.ask_levels.first_key_value(), Some((&Price{ val: 10.4 }, &0.11)));
}

#[test]
fn ticker_same_price() {
    let mut order_book = init_test_orderbook();
    let worse_bbo = BookTickerUpdate{
        last_update_id: 1,
        bid_price: 10.0,
        bid_qty: 0.03,
        ask_price: 10.5,
        ask_qty: 0.22,
        symbol: "BTCUSDT".to_string(),
    };

    order_book.update_book_ticker(&worse_bbo);

    assert_eq!(order_book.bid_levels.last_key_value(), Some((&Price{ val: 10.0 }, &0.03)));
    assert_eq!(order_book.ask_levels.first_key_value(), Some((&Price{ val: 10.5 }, &0.22)));
}

#[test]
fn best_level_normal() {
    let orderbook = init_test_orderbook();

    assert_eq!(orderbook.get_best_bid_ask(), Some(((10.0, 0.05), (10.5, 0.08))));
}

#[test]
fn best_level_one_empty() {
    let mut orderbook = init_test_orderbook();

    orderbook.ask_levels.clear();

    assert_eq!(orderbook.get_best_bid_ask(), None);
}

#[test]
fn best_level_both_empty() {
    let orderbook = OrderBook::new("BTCUSDT".to_string());

    assert_eq!(orderbook.get_best_bid_ask(), None);
}