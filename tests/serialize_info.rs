//! Unit tests for serialization of `BmsTableInfo` and `BmsTableList`
use std::collections::BTreeMap;

use bms_table::{BmsTableInfo, BmsTableList};
use url::Url;

fn make_info(name: &str, symbol: &str, url: Url) -> BmsTableInfo {
    BmsTableInfo::new(name.into(), symbol.into(), url)
}

#[test]
fn bms_table_list_serializes_as_array() {
    let mut item1 = make_info(
        ".WAS難易度表",
        "．",
        Url::parse("https://darksabun.club/table/archive/was/").unwrap(),
    );
    item1.extra = {
        let mut m = BTreeMap::new();
        m.insert("tag1".to_string(), serde_json::json!("SP"));
        m.insert(
            "tag2".to_string(),
            serde_json::json!("Self-made Chart Only"),
        );
        m.insert(
            "comment".to_string(),
            serde_json::json!("Converted by Ribbit"),
        );
        m.insert("date".to_string(), serde_json::json!(""));
        m.insert("state".to_string(), serde_json::json!(""));
        m.insert("tag_order".to_string(), serde_json::json!("1"));
        m
    };
    let mut item2 = make_info(
        "[F]",
        "[F]",
        Url::parse("https://bms.hexlataia.xyz/tables/convert/%5BF%5D/table.html").unwrap(),
    );
    item2.extra = {
        let mut m = BTreeMap::new();
        m.insert("tag1".to_string(), serde_json::json!("SP"));
        m.insert(
            "tag2".to_string(),
            serde_json::json!("Self-made Chart Only"),
        );
        m.insert("comment".to_string(), serde_json::json!("Converted by Hex"));
        m.insert("date".to_string(), serde_json::json!(""));
        m.insert("state".to_string(), serde_json::json!(""));
        m.insert("tag_order".to_string(), serde_json::json!("1"));
        m
    };
    let list = BmsTableList::new(vec![item1, item2]);

    let value = serde_json::to_value(&list).unwrap();
    assert!(value.is_array());

    let parsed: BmsTableList = serde_json::from_value(value).unwrap();
    assert_eq!(parsed.entries.len(), 2);
    let [i0, i1] = parsed.entries.as_slice() else {
        panic!(
            "expected two items, got {}: {:?}",
            parsed.entries.len(),
            parsed.entries
        );
    };
    assert_eq!(i0.name.as_str(), ".WAS難易度表");
    assert_eq!(i1.symbol.as_str(), "[F]");
}

#[test]
fn bms_table_info_minimal_fields_deserialized_correctly() {
    let json_data = r#"{
        "name": "Minimal Table",
        "symbol": "min",
        "url": "https://example.com/table.html"
    }"#;
    let info: bms_table::BmsTableInfo = serde_json::from_str(json_data).unwrap();
    assert_eq!(info.name, "Minimal Table");
    assert_eq!(info.symbol, "min");
    assert_eq!(info.url.as_str(), "https://example.com/table.html");
    assert!(
        info.extra.is_empty(),
        "extra should be empty for minimal fields"
    );
}

#[test]
fn bms_table_list_empty_array_has_no_entries() {
    let list: bms_table::BmsTableList = serde_json::from_value(serde_json::json!([])).unwrap();
    assert!(list.entries.is_empty());
}
