//! Fetch a single BMS difficulty table and print a summary.
//!
//! The URL can point to a JSON header or an HTML page containing a
//! `<meta name="bmstable">` that links to the header JSON.
//!
//! Usage:
//! ```sh
//! cargo run --example single_fetch [URL]
//! ```

use std::time::Duration;

use anyhow::{Context, Result};
use bms_table::{BmsTable, BmsTableData, BmsTableHeader, BmsTableHtml};
use reqwest::Client;
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "https://stellabms.xyz/sl/table.html".to_string());
    let url = Url::parse(&url)?;

    let client = build_client()?;

    match fetch_table(&client, &url).await {
        Ok(table) => {
            println!(
                "{}: {} charts, {} courses",
                table.header.name,
                table.data.charts.len(),
                table.header.course.flatten().len(),
            );
        }
        Err(e) => {
            eprintln!("Failed to fetch {url}");
            eprintln!("Error: {e:#}");
        }
    }

    Ok(())
}

/// Fetch a complete BMS difficulty table from a URL.
///
/// Handles two scenarios:
/// 1. The URL returns header JSON directly — parses it immediately.
/// 2. The URL returns HTML — extracts the `bmstable` meta tag URL,
///    fetches that, and parses the header JSON.
///
/// In both cases, the header's `data_url` is resolved and fetched to
/// produce the final [`BmsTable`].
async fn fetch_table(client: &Client, url: &Url) -> Result<BmsTable> {
    let text = fetch_text(client, url, "table page").await?;

    // Try to parse the response as header JSON directly.
    // If that fails, assume the page is HTML containing a bmstable link.
    let header: BmsTableHeader;
    let header_base_url: Url;

    if let Ok(h) = parse_json_clean::<BmsTableHeader>(&text) {
        header = h;
        header_base_url = url.clone();
    } else {
        let meta_url = BmsTableHtml::extract_url(&text)
            .context("Response is neither header JSON nor HTML with a bmstable meta tag")?;
        let header_url = url.join(meta_url)?;
        let header_text = fetch_text(client, &header_url, "header JSON").await?;
        header = parse_json_clean(&header_text).context("Failed to parse header JSON")?;
        header_base_url = header_url;
    }

    // Fetch the chart data referenced by the header.
    let data_url = header_base_url.join(&header.data_url)?;
    let data_text = fetch_text(client, &data_url, "chart data JSON").await?;
    let data: BmsTableData =
        parse_json_clean(&data_text).context("Failed to parse chart data JSON")?;

    Ok(BmsTable::new(header, data))
}

/// Fetch a URL's response body as text.
///
/// `label` is used in error messages to identify which fetch failed.
async fn fetch_text(client: &Client, url: &Url, label: &str) -> Result<String> {
    client
        .get(url.as_str())
        .send()
        .await
        .with_context(|| format!("Failed to fetch {label}"))?
        .text()
        .await
        .with_context(|| format!("Failed to read {label} body"))
}

/// Parse JSON, falling back to stripping control characters.
///
/// Some servers embed control characters in their JSON output, which causes
/// `serde_json` to reject them. This function cleans the input and retries
/// when a direct parse fails.
fn parse_json_clean<T: serde::de::DeserializeOwned>(raw: &str) -> Result<T> {
    Ok(serde_json::from_str(raw).or_else(|_| {
        let cleaned: String = raw.chars().filter(|c| !c.is_control()).collect();
        serde_json::from_str(&cleaned)
    })?)
}

/// Build a lenient HTTP client suitable for fetching BMS tables.
///
/// BMS table servers are a mixed ecosystem — some use self-signed
/// certificates, expired certs, or non-standard hostnames. The settings
/// below keep the examples working without requiring users to debug
/// TLS issues.
fn build_client() -> Result<Client> {
    Client::builder()
        .user_agent("Mozilla/5.0 bms-table-rs example")
        .timeout(Duration::from_secs(60))
        .redirect(reqwest::redirect::Policy::limited(20))
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build()
        .context("Failed to build HTTP client")
}
