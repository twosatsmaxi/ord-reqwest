use ordinals::RuneId;

use crate::ord_client::OrdClient;

pub mod ord_client;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let ord_client = OrdClient::new();
    let rune_id = RuneId::new(1, 0).unwrap();
    let rune_entry = ord_client.fetch_rune_details(rune_id).await;
    println!("Rune entry: {:?}", rune_entry.entry.spaced_rune);
    let block_height = ord_client.fetch_latest_block_height().await;
    println!("Block height: {}", block_height);
}
