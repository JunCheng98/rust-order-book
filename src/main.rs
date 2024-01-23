use binance_spot_connector_rust::hyper::Error;
use env_logger::Builder;
use futures_util::StreamExt;
use log;

use std::env;

mod model;
mod endpoints;
mod websocket;
mod stream;
mod manager;
mod formatter;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    // fetch the current order book from provided symbol
    let mut order_book = manager::init_order_book(query).await;

    // Websocket
    let mut conn = websocket::init(query).await;

    // Read messages
    while let Some(message) = conn.as_mut().next().await {
        match message {
            Ok(message) => {
                stream::handle_stream_data(message, &mut order_book).await;
            }
            Err(err) => {
                log::error!("Error when reading stream: {}", err);
                break
            }
        }

        formatter::print_order_book(&order_book, &query);
    }

    // Disconnect
    conn.close().await.expect("Failed to disconnect");

    Ok(())
}
