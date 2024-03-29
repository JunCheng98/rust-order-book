use tokio_tungstenite::tungstenite::protocol::Message;
use crate::{manager, model};

const DEPTH: &str = "@depth";
const BOOK_TICKER: &str = "@bookTicker";

pub async fn handle_stream_data(message: Message, order_book: &mut model::OrderBook) {
    let binary_data = message.into_data();
    let data = std::str::from_utf8(&binary_data).expect("Failed to parse message");
    log::info!("{:?}", data);
    let resp: model::StreamResponse = serde_json::from_str(&data).unwrap_or_default();

    match &resp.stream {
        _ if resp.stream.contains(DEPTH) => manager::update_order_book(resp, order_book).await,
        _ if resp.stream.contains(BOOK_TICKER) => manager::update_best_bid_and_ask(resp, order_book),
        &_ => ()
        // TODO: match more streams
    }
}