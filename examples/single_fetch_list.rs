//! Fetch and print the BMS difficulty table list.
//!
//! Usage:
//! ```sh
//! cargo run --example single_fetch_list [URL]
//! ```

use std::time::Duration;

use anyhow::{Context, Result};
use bms_table::BmsTableList;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let url = std::env::args().nth(1).unwrap_or_else(|| {
        "https://script.google.com/macros/s/AKfycbzaQbcI9UZDcDlSHHl2NHilhmePrNrwxRdOFkmIXsfnbfksKKmAB3V65WZ8jPWU-7E/exec?table=tablelist".to_string()
    });

    let client = build_client()?;

    let text = client
        .get(&url)
        .send()
        .await
        .context("Failed to fetch table list")?
        .text()
        .await
        .context("Failed to read response body")?;

    // Some servers return JSON with stray control characters.
    // Try a direct parse first; if that fails, strip control chars and retry.
    let list: BmsTableList = serde_json::from_str(&text)
        .or_else(|_| {
            let cleaned: String = text.chars().filter(|c| !c.is_control()).collect();
            serde_json::from_str(&cleaned)
        })
        .context("Failed to parse table list JSON")?;

    println!("Fetched {} table list entries.", list.entries.len());
    for (i, item) in list.entries.iter().take(10).enumerate() {
        println!("#{i}: {} [{}] -> {}", item.name, item.symbol, item.url);
    }

    Ok(())
}

/// Build a lenient HTTP client suitable for fetching BMS tables.
fn build_client() -> Result<Client> {
    Client::builder()
        .user_agent("Mozilla/5.0 bms-table-rs example")
        .timeout(Duration::from_secs(60))
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build()
        .context("Failed to build HTTP client")
}
