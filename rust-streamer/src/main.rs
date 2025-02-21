use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt};
use serde::{Deserialize, Serialize};
use redis::AsyncCommands;

const BINANCE_WS_URL: &str = "wss://stream.binance.com:9443/ws/btcusdt@trade";

#[derive(Debug, Deserialize, Serialize)]
struct Trade {
    #[serde(rename = 'p')]
    price: String,
    #[serde(rename = 'q')]
    quantity: String,
    #[serde(rename = 'q')]
    timestamp: u64
}

#[tokio::main]
async fn main() {
    println!("📡 Connecting Binance WebSockets...");

    let (ws_stream, _) = connect_async(BINANCE_WS_URL)
        .await
        .expect("🚨 Binance WebSockets connection error");

    let (_, mut read) = ws_stream.split();

    let client = redis::Client::open("redis://redis:6379").expect("🚨 Redis connection error");
    let mut con = client.get_async_connection().await.expect("❌ Can't connect to Redis");

    println!("✅ Connection successful ! Reading trades in real time...");

    while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
            if let Ok(trade) = serde_json::from_str::<Trade>(&text) {
                let _: () = con.set("btc_price", &trade.price).await.unwrap();
                println!("💰 Price stored in Redis : {} USDT", trade.price);
            }
        }
    }
}