//! Single-run example for fetching the table list
//!
//! How to run:
//!   cargo run --example single_fetch_list

#[path = "util/shared.rs"]
mod shared;

use shared::Fetcher;
use url::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = Url::parse(
        "https://script.google.com/macros/s/AKfycbzaQbcI9UZDcDlSHHl2NHilhmePrNrwxRdOFkmIXsfnbfksKKmAB3V65WZ8jPWU-7E/exec?table=tablelist",
    )?;

    let fetcher = Fetcher::lenient()?;
    let out = fetcher.fetch_table_list(url.clone()).await?;
    let listes = out.tables;
    let raw = out.raw_json;
    println!("Fetched {} table list entries.", listes.len());

    for (i, item) in listes.iter().take(10).enumerate() {
        println!("#{i}: {} [{}] -> {}", item.name, item.symbol, item.url);
    }

    println!("Raw JSON length: {}", raw.len());
    Ok(())
}
