//! Unit tests for serialization behavior of header, chart items, and table data

use bms_table::{BmsTableData, BmsTableHeader, BmsTableList, ChartItem, CourseGroup};
use std::collections::BTreeMap;

#[test]
fn header_serialize_flattens_extra_fields() {
    let header = BmsTableHeader {
        name: "Test Table".to_string(),
        symbol: "tt".to_string(),
        data_url: "charts.json".to_string(),
        tag: None,
        mode: None,
        course: CourseGroup::SubGroups(vec![CourseGroup::Courses(vec![])]),
        level_order: vec!["0".to_string(), "1".to_string()],
        extra: {
            let mut m = BTreeMap::new();
            m.insert("extra_field".to_string(), serde_json::json!("extra_value"));
            m.insert("another_field".to_string(), serde_json::json!(123));
            m
        },
    };

    let value = serde_json::to_value(&header).unwrap();
    let obj = value.as_object().expect("header must serialize to object");
    assert!(obj.contains_key("name"));
    assert!(obj.contains_key("symbol"));
    assert!(obj.contains_key("data_url"));
    assert!(obj.contains_key("course"));
    assert!(obj.contains_key("level_order"));
    assert!(obj.contains_key("extra_field"));
    assert!(obj.contains_key("another_field"));
    assert!(!obj.contains_key("extra"));

    let parsed: BmsTableHeader = serde_json::from_value(value).unwrap();
    assert_eq!(
        parsed.extra.get("extra_field"),
        Some(&serde_json::json!("extra_value"))
    );
    assert_eq!(
        parsed.extra.get("another_field"),
        Some(&serde_json::json!(123))
    );
}

#[test]
fn chart_item_serialize_flattens_extra_fields() {
    let item = ChartItem {
        level: "1".to_string(),
        md5: Some("md5hash".to_string()),
        sha256: None,
        title: Some("Song Title".to_string()),
        artist: None,
        url: Some("http://example.com".to_string()),
        url_diff: None,
        comment: None,
        extra: {
            let mut m = BTreeMap::new();
            m.insert("custom_field".to_string(), serde_json::json!("value"));
            m.insert("rating".to_string(), serde_json::json!(4.5));
            m
        },
    };

    let value = serde_json::to_value(&item).unwrap();
    let obj = value
        .as_object()
        .expect("chart item must serialize to object");
    assert!(obj.contains_key("level"));
    assert!(obj.contains_key("custom_field"));
    assert!(obj.contains_key("rating"));
    assert!(!obj.contains_key("extra"));

    let parsed: ChartItem = serde_json::from_value(value).unwrap();
    assert_eq!(
        parsed.extra.get("custom_field"),
        Some(&serde_json::json!("value"))
    );
    assert_eq!(parsed.extra.get("rating"), Some(&serde_json::json!(4.5)));
}

#[test]
fn bms_table_data_serializes_as_array() {
    let item1 = ChartItem {
        level: "0".to_string(),
        md5: None,
        sha256: None,
        title: None,
        artist: None,
        url: None,
        url_diff: None,
        comment: None,
        extra: BTreeMap::new(),
    };
    let item2 = ChartItem {
        level: "1".to_string(),
        md5: None,
        sha256: None,
        title: None,
        artist: None,
        url: None,
        url_diff: None,
        comment: None,
        extra: BTreeMap::new(),
    };
    let data = BmsTableData {
        charts: vec![item1, item2],
    };

    let value = serde_json::to_value(&data).unwrap();
    assert!(value.is_array());

    let parsed: BmsTableData = serde_json::from_value(value).unwrap();
    assert_eq!(parsed.charts.len(), 2);
    let [c0, c1] = parsed.charts.as_slice() else {
        panic!(
            "expected two charts, got {}: {:?}",
            parsed.charts.len(),
            parsed.charts
        );
    };
    assert_eq!(c0.level.as_str(), "0");
    assert_eq!(c1.level.as_str(), "1");
}

#[test]
fn optional_fields_except_comment_serialize_as_null() {
    let item = ChartItem {
        level: "12".to_string(),
        md5: Some("hash".to_string()),
        sha256: None,
        title: None,
        artist: None,
        url: None,
        url_diff: None,
        comment: None,
        extra: BTreeMap::new(),
    };
    let value = serde_json::to_value(&item).unwrap();
    let obj = value
        .as_object()
        .expect("chart item must serialize to object");
    assert!(obj.contains_key("level"));
    assert!(obj.contains_key("md5"));
    // Fields without skip_serializing_if serialize None as null
    for key in &["sha256", "title", "artist", "url", "url_diff"] {
        assert!(obj.contains_key(*key), "expected key '{key}' to be present");
        assert_eq!(obj.get(*key).unwrap(), &serde_json::Value::Null);
    }
    // comment has skip_serializing_if = "Option::is_none"
    assert!(!obj.contains_key("comment"));
    assert!(!obj.contains_key("extra"));
}

#[test]
fn header_tag_mode_roundtrip_preserves_values() {
    let header = BmsTableHeader {
        name: "Table".to_string(),
        symbol: "t".to_string(),
        data_url: "d.json".to_string(),
        tag: Some("★".to_string()),
        mode: Some("7K".to_string()),
        course: CourseGroup::Courses(vec![]),
        level_order: vec!["1".to_string(), "2".to_string()],
        extra: BTreeMap::new(),
    };

    let value = serde_json::to_value(&header).unwrap();
    assert_eq!(value.get("tag").unwrap(), &serde_json::json!("★"));
    assert_eq!(value.get("mode").unwrap(), &serde_json::json!("7K"));

    let parsed: BmsTableHeader = serde_json::from_value(value).unwrap();
    assert_eq!(parsed.tag.as_deref(), Some("★"));
    assert_eq!(parsed.mode.as_deref(), Some("7K"));
}

#[test]
fn header_tag_mode_none_skipped_in_serialization() {
    let header = BmsTableHeader {
        name: "Table".to_string(),
        symbol: "t".to_string(),
        data_url: "d.json".to_string(),
        tag: None,
        mode: None,
        course: CourseGroup::Courses(vec![]),
        level_order: vec![],
        extra: BTreeMap::new(),
    };

    let value = serde_json::to_value(&header).unwrap();
    let obj = value.as_object().unwrap();
    assert!(!obj.contains_key("tag"), "tag should be absent when None");
    assert!(!obj.contains_key("mode"), "mode should be absent when None");
}

#[test]
fn bms_table_data_new_constructor_sets_charts() {
    let item = ChartItem::new("12".to_string());
    let data = BmsTableData::new(vec![item]);
    assert_eq!(data.charts.len(), 1);
}

#[test]
fn bms_table_list_new_constructor_sets_entries() {
    use bms_table::BmsTableInfo;
    use url::Url;

    let info = BmsTableInfo {
        name: "Test".into(),
        symbol: "t".into(),
        url: Url::parse("https://example.com/table.html").unwrap(),
        extra: BTreeMap::new(),
    };
    let list = BmsTableList::new(vec![info]);
    assert_eq!(list.entries.len(), 1);
}
