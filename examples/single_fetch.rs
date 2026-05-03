//! Example: fetch a single BMS difficulty table and print summary
#![cfg_attr(not(feature = "reqwest"), allow(unused_imports))]

use std::env;

#[cfg(feature = "reqwest")]
use bms_table::fetch::Error as FetchError;
#[cfg(feature = "reqwest")]
use bms_table::fetch::reqwest::Fetcher;
#[cfg(feature = "reqwest")]
use url::Url;

#[cfg(feature = "reqwest")]
#[tokio::main]
async fn main() -> Result<(), FetchError> {
    let url = env::args()
        .nth(1)
        .unwrap_or_else(|| "https://stellabms.xyz/sl/table.html".to_string());
    let url = Url::parse(&url).map_err(|e| FetchError::Validation {
        field: "url",
        reason: e.to_string(),
    })?;

    let fetcher = Fetcher::lenient()?;

    match fetcher.fetch_table(url.clone()).await {
        Ok(fetched) => {
            let table = fetched.table;
            println!(
                "{} fetched successfully ({} charts, {} course groups, {} courses)",
                table.header.name,
                table.data.charts.len(),
                table.header.course.len(),
                table.header.course.iter().flatten().count()
            );
        }
        Err(e) => {
            eprintln!("Fetch failed for: {}", url);
            eprintln!("Error: {}", e);
            let mut source = std::error::Error::source(&e);
            let mut i = 0;
            while let Some(s) = source {
                eprintln!("  [{}] {}", i, s);
                source = std::error::Error::source(s);
                i += 1;
            }
        }
    }

    Ok(())
}

#[cfg(not(feature = "reqwest"))]
fn main() {
    eprintln!("This example requires the `reqwest` feature to be enabled.");
}
