//! Fetch multiple BMS difficulty tables concurrently.
//!
//! Demonstrates concurrent HTTP fetching with per-table result reporting.
//! Each table is fetched independently. When one finishes, a message is
//! printed immediately. Errors for individual tables are reported and
//! do not stop the remaining fetches.
//!
//! Usage:
//! ```sh
//! cargo run --example multi_fetch [URL ...]
//! ```

use std::time::Duration;

use anyhow::{Context, Result};
use bms_table::{BmsTable, BmsTableData, BmsTableHeader, BmsTableHtml};
use reqwest::Client;
use tokio::sync::mpsc;
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    let client = build_client()?;
    let urls = get_urls();
    println!("Fetching {} tables concurrently...\n", urls.len());

    // Channel for collecting results from concurrent fetch tasks.
    let (tx, mut rx) = mpsc::channel::<FetchResult>(32);

    // Background task: print results as they arrive.
    let handler = tokio::spawn(async move {
        while let Some(result) = rx.recv().await {
            match result.table {
                Ok(table) => {
                    println!(
                        "[OK]  {} ({} charts, {} courses)",
                        table.header.name,
                        table.data.charts.len(),
                        table.header.course.flatten().len(),
                    );
                }
                Err(e) => {
                    println!("[ERR] {}: {e:#}", result.label);
                }
            }
        }
    });

    // Spawn one concurrent fetch task per URL.
    for url in urls {
        let tx = tx.clone();
        let client = client.clone();
        tokio::spawn(async move {
            let result = fetch_one(&client, &url).await;
            let _ = tx.send(result).await;
        });
    }

    // Close our sender so the handler task exits when all fetches finish.
    drop(tx);
    handler.await?;

    println!("\nDone.");
    Ok(())
}

// Fetch helpers

/// Outcome of fetching a single BMS difficulty table.
struct FetchResult {
    /// Display label (table name on success, URL on failure).
    label: String,
    /// Parsed table or error.
    table: Result<BmsTable>,
}

/// Fetch one table, wrapping the outcome in a [`FetchResult`].
async fn fetch_one(client: &Client, url: &Url) -> FetchResult {
    match fetch_table(client, url).await {
        Ok(t) => FetchResult {
            label: t.header.name.clone(),
            table: Ok(t),
        },
        Err(e) => FetchResult {
            label: url.to_string(),
            table: Err(e),
        },
    }
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

// URL sources

/// Collect table URLs from command-line args, or fall back to a default list.
fn get_urls() -> Vec<Url> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        args.iter()
            .skip(1)
            .filter_map(|s| Url::parse(s).ok())
            .collect()
    } else {
        vec![
            "https://stellabms.xyz/sl/table.html",
            "https://stellabms.xyz/dp/table.html",
            "https://zris.work/bmstable/normal/normal_header.json",
            "https://zris.work/bmstable/insane/insane_header.json",
            "http://rattoto10.jounin.jp/table_overjoy.html",
            "http://rattoto10.jounin.jp/table.html",
            "http://rattoto10.jounin.jp/table_insane.html",
            "http://walkure.net/hakkyou/for_glassist/bms/?lamp=easy",
            "http://walkure.net/hakkyou/for_glassist/bms/?lamp=normal",
            "http://walkure.net/hakkyou/for_glassist/bms/?lamp=hard",
            "http://walkure.net/hakkyou/for_glassist/bms/?lamp=fc",
            "https://notmichaelchen.github.io/stella-table-extensions/stellalite.html",
            "https://iidxtool.kasacontent.com/homage/table.php",
            "https://zris.work/bmstable/dp_normal/dpn_header.json",
            "https://pmsdifficulty.xxxxxxxx.jp/PMSdifficulty.html",
            "https://pmsdifficulty.xxxxxxxx.jp/insane_PMSdifficulty.html",
            "http://zris.work/bmstable/pms_insane/insane_pmsdatabase_header.json",
            "https://pmsdifficulty.xxxxxxxx.jp/_pastoral_insane_table.html",
            "https://www.notepara.com/glassist/10k",
            "http://onzonium.at-ninja.jp/dp/",
            "https://lr2.sakura.ne.jp/overjoy.php",
        ]
        .into_iter()
        .filter_map(|s| Url::parse(s).ok())
        .collect()
    }
}
