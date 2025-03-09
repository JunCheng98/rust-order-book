use binance_spot_connector_rust::{
    market_stream::{book_ticker::BookTickerStream, partial_depth::PartialDepthStream},
    tokio_tungstenite::{BinanceWebSocketClient, WebSocketState},
};
use tokio_tungstenite::{MaybeTlsStream, tungstenite::protocol::Message};
use tokio::net::TcpStream;

use crate::orderbook::OrderBook;
use crate::response::{StreamData, StreamResponse};


const LEVELS: u16 = 20;

// Establish connection and subscribe to streams
pub async fn init_connection(symbol: &str) -> WebSocketState<MaybeTlsStream<TcpStream>> {
    let (mut conn, _) = BinanceWebSocketClient::connect_async_default()
        .await
        .expect("Failed to connect");
    
    conn.subscribe(vec![
        &PartialDepthStream::from_100ms(symbol, LEVELS).into(),
        &BookTickerStream::from_symbol(symbol).into(),
    ])
    .await;

    conn
}

pub async fn handle_stream_data(message: Message, order_book: &mut OrderBook) {
    let binary_data = message.into_data();
    let data = std::str::from_utf8(&binary_data).expect("Failed to parse message");

    match serde_json::from_str::<StreamResponse>(&data) {
        Ok(resp) => {
            match resp.data {
                StreamData::BookTickerUpdateData(book_ticker) => order_book.update_book_ticker(&book_ticker),
                StreamData::DepthUpdateData(depth_update) => order_book.update_depth(&depth_update),
            }
        }
        Err(err) => {
            log::error!("Error when deserializing to response: {}", err);
        }
    }
}