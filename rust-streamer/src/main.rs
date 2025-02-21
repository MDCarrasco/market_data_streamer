use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use redis::AsyncCommands;
use rustls::crypto::ring;

const BINANCE_WS_URL: &str = "wss://stream.binance.com:9443/ws/btcusdt@trade";

#[derive(Debug, Deserialize, Serialize)]
struct Trade {
    #[serde(rename = "p")]
    price: String,
    #[serde(rename = "q")]
    quantity: String,
    #[serde(rename = "T")]
    timestamp: u64,
}

#[tokio::main]
async fn main() {
    // Install Rustls Crypto Provider (fixes TLS issue)
    ring::default_provider()
        .install_default()
        .expect("Failed to install Rustls CryptoProvider");

    println!("📡 Connecting to Binance WebSockets...");

    let (ws_stream, _) = connect_async(BINANCE_WS_URL)
        .await
        .expect("🚨 Binance WebSockets connection error");

    let (_, mut read) = ws_stream.split();

    let client = redis::Client::open("redis://redis:6379").expect("🚨 Redis connection error");

    let mut attempts = 5;
    let mut con = None;

    while attempts > 0 {
        match client.get_multiplexed_async_connection().await {
            Ok(connection) => {
                println!("✅ Connected to Redis");
                con = Some(connection);
                break;
            }
            Err(_) => {
                println!("⏳ Redis not ready... retrying in 2s ({})", attempts);
                attempts -= 1;
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        }
    }

    if con.is_none() {
        panic!("❌ Redis connection failed after multiple attempts");
    }

    let mut con = con.unwrap();
    println!("✅ Connection successful! Reading trades in real-time...");

    while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
            if let Ok(trade) = serde_json::from_str::<Trade>(&text) {
                let _: () = con.set("btc_price", &trade.price).await.unwrap();
                println!("💰 Price stored in Redis: {} USDT", trade.price);
            }
        }
    }
}
