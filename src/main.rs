use binance_spot_connector_rust::hyper::Error;
use env_logger::Builder;
use futures_util::StreamExt;
use log;

mod websocket;
mod response;
mod orderbook;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    let symbol = match args.get(1) {
        Some(arg) => arg,
        None => panic!("Run the program as follows: cargo run -- [SYMBOL]\n Symbol examples: BTCUSDT, BNBBTC etc.")
    };

    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let mut order_book = orderbook::OrderBook::new(symbol.to_string());

    let mut conn = websocket::init_connection(symbol.as_str()).await;
    while let Some(message) = conn.as_mut().next().await {
        match message {
            Ok(message) => {
                websocket::handle_stream_data(message, &mut order_book).await;
                println!("Top Level: {:?}", order_book.get_best_bid_ask());
                println!("{}", order_book.to_string());
            }
            Err(err) => {
                log::error!("Error when reading stream: {}", err);
                break
            }
        }
    }
    conn.close().await.expect("Failed to disconnect");

    Ok(())
}
