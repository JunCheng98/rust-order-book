use binance_spot_connector_rust::{
    market_stream::diff_depth::DiffDepthStream,
    tokio_tungstenite::{BinanceWebSocketClient, WebSocketState},
};
use tokio_tungstenite::MaybeTlsStream;
use tokio::net::TcpStream;

// Establish connection
pub async fn init(query: &str) -> WebSocketState<MaybeTlsStream<TcpStream>> {
    let (mut conn, _) = BinanceWebSocketClient::connect_async_default()
        .await
        .expect("Failed to connect");
    
    // Subscribe to streams
    conn.subscribe(vec![
        &DiffDepthStream::from_1000ms(query).into(),
    ])
    .await;

    conn
}