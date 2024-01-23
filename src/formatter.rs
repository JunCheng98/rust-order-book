use crate::model;


// prints the top 5 bids and asks from the order book
pub fn print_order_book(order_book: &model::OrderBook, query: &str) {
    print!("\x1B[2J\x1B[1;1H");

    println!("Order Book - {}", query);
    println!("Sell Order:");
    println!(
        "{0: <20} | {1: <20} | {2: <20}",
        "Price", "Amount", "Total"
    );

    let mut asks_iter = order_book.asks_map.iter().peekable();
    asks_iter.peek();
    for _ in 0..5 {
        let ask = &asks_iter.next().unwrap();
        println!("{0: <20} | {1: <20} | {2: <20}", ask.0.val, ask.1, ask.0.val * ask.1);
    }

    println!("\nBuy Order:");
    println!(
        "{0: <20} | {1: <20} | {2: <20}",
        "Price", "Amount", "Total"
    );

    let mut bids_iter = order_book.bids_map.iter().rev().peekable();
    bids_iter.peek();
    for _ in 0..5 {
        let bid = &bids_iter.next().unwrap();
        println!("{0: <20} | {1: <20} | {2: <20}", bid.0.val, bid.1, bid.0.val * bid.1);
    }
}