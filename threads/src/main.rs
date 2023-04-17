#![allow(unused)]
use threads::fut::{future, sync};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //  sync::future_plays::sync_fut();
    //  sync::future_plays::async_fut();
    future::fut_test::fut().await
}
