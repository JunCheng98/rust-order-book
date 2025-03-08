use binance_spot_connector_rust::{
    market_stream::{book_ticker::BookTickerStream, partial_depth::PartialDepthStream},
    tokio_tungstenite::{BinanceWebSocketClient, WebSocketState},
};
use tokio_tungstenite::MaybeTlsStream;
use tokio::net::TcpStream;

const LEVELS: u16 = 20;

// Establish connection
pub async fn init(symbol: &str) -> WebSocketState<MaybeTlsStream<TcpStream>> {
    let (mut conn, _) = BinanceWebSocketClient::connect_async_default()
        .await
        .expect("Failed to connect");
    
    // Subscribe to streams
    conn.subscribe(vec![
        &PartialDepthStream::from_100ms(symbol, LEVELS).into(),
        &BookTickerStream::from_symbol(symbol).into(),
    ])
    .await;

    conn
}