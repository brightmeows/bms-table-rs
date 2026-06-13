//! Concurrent multi-table fetching example
//!
//! Fetches multiple BMS difficulty tables concurrently and emits an event
//! when each table finishes. If fetching fails for one table, the program
//! prints the error and continues with others.

#[path = "util/shared.rs"]
mod shared;

use anyhow::Result;
use bms_table::BmsTable;
use shared::Fetcher;
use std::env;
use tokio::sync::mpsc;
use url::Url;

/// Demonstrates concurrent fetching across multiple difficulty tables.
#[tokio::main]
async fn main() -> Result<()> {
    println!("Concurrent difficulty-table fetcher");
    println!("===================");

    let fetcher = Fetcher::lenient()?;

    let urls = table_urls();
    let url_count = urls.len();
    println!("Fetching {url_count} difficulty tables...");
    println!();

    let (tx, mut rx) = mpsc::channel::<FetchResult>(100);

    let event_handler = tokio::spawn(async move {
        while let Some(result) = rx.recv().await {
            match result.table {
                Ok(table) => {
                    println!(
                        "{} fetched successfully ({} charts, {} courses)",
                        result.name,
                        table.data.charts.len(),
                        table.header.course.flatten().len()
                    );
                }
                Err(e) => {
                    println!("{} fetch failed: {}", result.name, e);
                }
            }
        }
    });

    let fetch_tasks: Vec<_> = urls
        .into_iter()
        .map(|url| {
            let tx = tx.clone();
            let fetcher_cloned = fetcher.clone();
            tokio::spawn(async move {
                let result = fetch_single_table(&fetcher_cloned, &url).await;
                let _ = tx.send(result).await;
            })
        })
        .collect();

    for task in fetch_tasks {
        let _ = task.await;
    }

    drop(tx);
    let _ = event_handler.await;

    println!();
    println!("Fetch summary:");
    println!("  Concurrency: {url_count} tables");
    println!("  Processing: async concurrency");
    println!("  Event handling: real-time dispatch");

    Ok(())
}

/// Collect table URLs from command-line args or fall back to a default list.
fn table_urls() -> Vec<Url> {
    let args: Vec<String> = env::args().collect();

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

/// Fetch result for a single table, tagged with its display name.
#[derive(Debug)]
struct FetchResult {
    /// Display name (table name on success, URL on failure).
    name: String,
    /// Parsed table or error.
    table: anyhow::Result<BmsTable>,
}

/// Fetch a single table and wrap the outcome in a [`FetchResult`].
async fn fetch_single_table(fetcher: &Fetcher, url: &Url) -> FetchResult {
    match fetcher.fetch_table(url.clone()).await {
        Ok(bms_table) => FetchResult {
            name: bms_table.header.name.clone(),
            table: Ok(bms_table),
        },
        Err(e) => FetchResult {
            name: url.to_string(),
            table: Err(e),
        },
    }
}
