//! Example: fetch a single BMS difficulty table and print summary

#[path = "util/shared.rs"]
mod shared;

use anyhow::Result;
use shared::Fetcher;
use std::env;
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    let url = env::args()
        .nth(1)
        .unwrap_or_else(|| "https://stellabms.xyz/sl/table.html".to_string());
    let url = Url::parse(&url)?;

    let fetcher = Fetcher::lenient()?;

    match fetcher.fetch_table(url.clone()).await {
        Ok(table) => {
            println!(
                "{} fetched successfully ({} charts, {} course groups, {} courses)",
                table.header.name,
                table.data.charts.len(),
                table.header.course.len(),
                table.header.flatten_courses().len()
            );
        }
        Err(e) => {
            eprintln!("Fetch failed for: {}", url);
            eprintln!("Message: {}", e);
            eprintln!("Causes:");
            for (i, cause) in e.chain().enumerate() {
                eprintln!("  [{}] {}", i, cause);
            }
            match std::env::var("RUST_BACKTRACE").as_deref() {
                Ok("1") => {
                    eprintln!("Backtrace:");
                    eprintln!("{:?}", e.backtrace());
                }
                _ => {
                    eprintln!("Hint: set RUST_BACKTRACE=1 to print backtrace.");
                }
            }
        }
    }

    Ok(())
}
