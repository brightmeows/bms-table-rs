//! Unit tests for serialization behavior of header, chart items, and table data

use bms_table::{BmsTableData, BmsTableHeader, BmsTableInfo, BmsTableList, ChartItem, CourseGroup};
use std::collections::BTreeMap;

#[test]
fn header_serialize_flattens_extra_fields() {
    let mut header = BmsTableHeader::new("Test Table".into(), "tt".into(), "charts.json".into());
    header.course = CourseGroup::SubGroups(vec![CourseGroup::Courses(vec![])]);
    header.level_order = vec!["0".to_string(), "1".to_string()];
    header.extra = {
        let mut m = BTreeMap::new();
        m.insert("extra_field".to_string(), serde_json::json!("extra_value"));
        m.insert("another_field".to_string(), serde_json::json!(123));
        m
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
    let mut item = ChartItem::new("1".into());
    item.md5 = Some("md5hash".to_string());
    item.title = Some("Song Title".to_string());
    item.url = Some("http://example.com".to_string());
    item.extra = {
        let mut m = BTreeMap::new();
        m.insert("custom_field".to_string(), serde_json::json!("value"));
        m.insert("rating".to_string(), serde_json::json!(4.5));
        m
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
    let data = BmsTableData::new(vec![ChartItem::new("0".into()), ChartItem::new("1".into())]);

    let value = serde_json::to_value(&data).unwrap();
    assert!(value.is_array());

    let parsed: BmsTableData = serde_json::from_value(value).unwrap();
    assert_eq!(parsed.len(), 2);
    let [c0, c1] = parsed.as_slice() else {
        panic!("expected two charts, got {}: {:?}", parsed.len(), parsed.0);
    };
    assert_eq!(c0.level.as_str(), "0");
    assert_eq!(c1.level.as_str(), "1");
}

#[test]
fn optional_fields_are_skipped_when_none() {
    let mut item = ChartItem::new("12".into());
    item.md5 = Some("hash".to_string());
    let value = serde_json::to_value(&item).unwrap();
    let obj = value
        .as_object()
        .expect("chart item must serialize to object");
    assert!(obj.contains_key("level"));
    assert!(obj.contains_key("md5"));
    // All optional fields use skip_serializing_if = "Option::is_none",
    // so None fields are absent, not null.
    for key in &["sha256", "title", "artist", "url", "url_diff", "comment"] {
        assert!(!obj.contains_key(*key), "expected key '{key}' to be absent");
    }
    assert!(!obj.contains_key("extra"));
}

#[test]
fn header_tag_mode_roundtrip_preserves_values() {
    let mut header = BmsTableHeader::new("Table".into(), "t".into(), "d.json".into());
    header.tag = Some("★".to_string());
    header.mode = Some("7K".to_string());
    header.level_order = vec!["1".to_string(), "2".to_string()];

    let value = serde_json::to_value(&header).unwrap();
    assert_eq!(value.get("tag").unwrap(), &serde_json::json!("★"));
    assert_eq!(value.get("mode").unwrap(), &serde_json::json!("7K"));

    let parsed: BmsTableHeader = serde_json::from_value(value).unwrap();
    assert_eq!(parsed.tag.as_deref(), Some("★"));
    assert_eq!(parsed.mode.as_deref(), Some("7K"));
}

#[test]
fn header_tag_mode_none_skipped_in_serialization() {
    let header = BmsTableHeader::new("Table".into(), "t".into(), "d.json".into());

    let value = serde_json::to_value(&header).unwrap();
    let obj = value.as_object().unwrap();
    assert!(!obj.contains_key("tag"), "tag should be absent when None");
    assert!(!obj.contains_key("mode"), "mode should be absent when None");
}

#[test]
fn bms_table_data_new_constructor_sets_charts() {
    let item = ChartItem::new("12".to_string());
    let data = BmsTableData::new(vec![item]);
    assert_eq!(data.len(), 1);
}

#[test]
fn bms_table_list_new_constructor_sets_entries() {
    use url::Url;

    let info = BmsTableInfo::new(
        "Test".into(),
        "t".into(),
        Url::parse("https://example.com/table.html").unwrap(),
    );
    let list = BmsTableList::new(vec![info]);
    assert_eq!(list.len(), 1);
}

#[test]
fn chart_item_default_has_zero_level() {
    let item = ChartItem::default();
    assert_eq!(item.level, "0");
    assert!(item.md5.is_none());
    assert!(item.title.is_none());
}

#[test]
fn bms_table_data_default_is_empty() {
    let data = BmsTableData::default();
    assert!(data.is_empty());
}

#[test]
fn trophy_new_sets_all_fields() {
    use bms_table::Trophy;
    let trophy = Trophy::new("goldmedal".into(), 2.5, 85.0);
    assert_eq!(trophy.name, "goldmedal");
    assert!((trophy.missrate - 2.5).abs() <= 1e-12);
    assert!((trophy.scorerate - 85.0).abs() <= 1e-12);
}
