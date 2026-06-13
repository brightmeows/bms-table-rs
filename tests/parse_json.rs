//! Unit tests for JSON parsing and data structure deserialization
//!
//! Covers common and edge inputs for headers, courses, and chart data to ensure deserialization and field compatibility behave correctly.

use bms_table::{BmsTable, BmsTableData, BmsTableHeader, CourseGroup, CourseInfo};
use serde_json::json;
use std::collections::BTreeMap;

// JSON parsing related tests: derived from original lib_tests.rs and fetch_tests.rs

#[test]
fn build_bms_table_from_json_parses_all_fields() {
    let header_json = json!({
        "name": "Test Table",
        "symbol": "test",
        "data_url": "charts.json",
        "course": [
            [
                {
                    "name": "Test Course",
                    "constraint": ["grade_mirror"],
                    "trophy": [
                        {
                            "name": "goldmedal",
                            "missrate": 1.0,
                            "scorerate": 90.0
                        }
                    ],
                    "md5": ["test_md5_1", "test_md5_2"]
                }
            ]
        ],
        "level_order": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, "!i"],
        "extra_field": "extra_value",
        "another_field": 123
    });
    let data_json = json!([
        {
            "level": "1",
            "id": 1,
            "md5": "test_md5_1",
            "sha256": "test_sha256_1",
            "title": "Test Song",
            "artist": "Test Artist",
            "url": "https://example.com/test.bms",
            "url_diff": "https://example.com/test_diff.bms",
            "custom_field": "custom_value",
            "rating": 5.0
        }
    ]);
    let header: BmsTableHeader = serde_json::from_value(header_json).unwrap();
    let data: BmsTableData = serde_json::from_value(data_json).unwrap();
    let bms_table = BmsTable { header, data };
    assert_eq!(bms_table.header.name, "Test Table");
    assert_eq!(bms_table.header.symbol, "test");
    assert_eq!(bms_table.header.data_url, "charts.json");
    assert_eq!(bms_table.data.charts.len(), 1);

    // Input is `"course": [[{...}]]` — nested structure
    let course = match &bms_table.header.course {
        CourseGroup::SubGroups(groups) => {
            let [group] = groups.as_slice() else {
                panic!("expected one group, got {}: {:?}", groups.len(), groups);
            };
            match group {
                CourseGroup::Courses(courses) => {
                    let [course] = courses.as_slice() else {
                        panic!("expected one course, got {}: {:?}", courses.len(), courses);
                    };
                    course
                }
                other => panic!("expected Courses, got {:?}", other),
            }
        }
        other => panic!("expected SubGroups, got {:?}", other),
    };
    assert_eq!(course.name, "Test Course");
    assert_eq!(course.constraint, vec!["grade_mirror"]);
    assert_eq!(course.trophy.len(), 1);
    let [trophy0] = course.trophy.as_slice() else {
        panic!(
            "expected one trophy, got {}: {:?}",
            course.trophy.len(),
            course.trophy
        );
    };
    assert_eq!(trophy0.name, "goldmedal");
    assert!((trophy0.missrate - 1.0).abs() <= 1e-12);
    assert!((trophy0.scorerate - 90.0).abs() <= 1e-12);
    assert_eq!(course.charts.len(), 2);
    let [c0, c1] = course.charts.as_slice() else {
        panic!(
            "expected two charts, got {}: {:?}",
            course.charts.len(),
            course.charts
        );
    };
    assert_eq!(c0.md5.as_deref(), Some("test_md5_1"));
    assert_eq!(c1.md5.as_deref(), Some("test_md5_2"));

    let [score] = bms_table.data.charts.as_slice() else {
        panic!(
            "expected one chart, got {}: {:?}",
            bms_table.data.charts.len(),
            bms_table.data.charts
        );
    };
    assert_eq!(score.level, "1");
    assert_eq!(score.md5, Some("test_md5_1".to_string()));
    assert_eq!(score.sha256, Some("test_sha256_1".to_string()));
    assert_eq!(score.title, Some("Test Song".to_string()));
    assert_eq!(score.artist, Some("Test Artist".to_string()));
    assert_eq!(score.url, Some("https://example.com/test.bms".to_string()));
    assert_eq!(
        score.url_diff,
        Some("https://example.com/test_diff.bms".to_string())
    );

    assert_eq!(
        bms_table.header.extra.get("extra_field"),
        Some(&json!("extra_value"))
    );
    assert_eq!(
        bms_table.header.extra.get("another_field"),
        Some(&json!(123))
    );
    assert!(!bms_table.header.extra.contains_key("name"));

    assert_eq!(
        score.extra.get("custom_field"),
        Some(&json!("custom_value"))
    );
    assert_eq!(score.extra.get("rating"), Some(&json!(5.0)));
    assert!(!score.extra.contains_key("level"));

    assert_eq!(bms_table.header.level_order.len(), 22);
    let [first, .., penultimate, last] = bms_table.header.level_order.as_slice() else {
        panic!(
            "expected at least two level_order entries, got {}: {:?}",
            bms_table.header.level_order.len(),
            bms_table.header.level_order
        );
    };
    assert_eq!(first.as_str(), "0");
    assert_eq!(penultimate.as_str(), "20");
    assert_eq!(last.as_str(), "!i");
    assert!(!bms_table.header.extra.contains_key("level_order"));
}

#[test]
fn build_bms_table_with_empty_fields_keeps_empty_strings() {
    let header_json = json!({
        "name": "Test Table",
        "symbol": "test",
        "data_url": "charts.json",
        "course": []
    });
    let data_json = json!([
        {
            "level": "1",
            "id": 1,
            "md5": "",
            "sha256": "",
            "title": "",
            "artist": "",
            "url": "",
            "url_diff": ""
        }
    ]);
    let header: BmsTableHeader = serde_json::from_value(header_json).unwrap();
    let data: BmsTableData = serde_json::from_value(data_json).unwrap();
    let bms_table = BmsTable { header, data };
    let [score] = bms_table.data.charts.as_slice() else {
        panic!(
            "expected one chart, got {}: {:?}",
            bms_table.data.charts.len(),
            bms_table.data.charts
        );
    };
    assert_eq!(score.level, "1");
    // Current behavior keeps empty strings as Some("") for optional string fields
    assert_eq!(score.md5, Some("".to_string()));
    assert_eq!(score.sha256, Some("".to_string()));
    assert_eq!(score.title, Some("".to_string()));
    assert_eq!(score.artist, Some("".to_string()));
    assert_eq!(score.url, Some("".to_string()));
    assert_eq!(score.url_diff, Some("".to_string()));
}

#[test]
fn bms_table_creation_sets_fields_correctly() {
    let header = BmsTableHeader {
        name: "Test Table".to_string(),
        symbol: "test".to_string(),
        data_url: "https://example.com/charts.json".to_string(),
        tag: None,
        mode: None,
        course: CourseGroup::Courses(vec![]),
        level_order: vec!["0".to_string(), "1".to_string()],
        extra: BTreeMap::new(),
    };
    let data = BmsTableData { charts: vec![] };
    let bms_table = BmsTable { header, data };

    assert_eq!(bms_table.header.name, "Test Table");
    assert_eq!(bms_table.header.symbol, "test");
    assert!(bms_table.header.course.flatten().is_empty());
    assert_eq!(bms_table.data.charts.len(), 0);
    assert_eq!(bms_table.header.level_order.len(), 2);
}

#[test]
fn bms_table_partial_eq_matches_identical_tables() {
    let header1 = BmsTableHeader {
        name: "Test Table".to_string(),
        symbol: "test".to_string(),
        data_url: "https://example.com/charts.json".to_string(),
        tag: None,
        mode: None,
        course: CourseGroup::Courses(vec![]),
        level_order: vec!["0".to_string(), "1".to_string()],
        extra: BTreeMap::new(),
    };
    let data1 = BmsTableData { charts: vec![] };
    let table1 = BmsTable {
        header: header1.clone(),
        data: data1,
    };

    let header2 = header1;
    let data2 = BmsTableData { charts: vec![] };
    let table2 = BmsTable {
        header: header2,
        data: data2,
    };

    assert_eq!(table1, table2);
}

#[test]
fn chart_item_numeric_fields_convert_correctly() {
    let data_json = json!([
        {
            "level": 0,
            "id": 1,
            "md5": "12345",
            "sha256": "67890",
            "title": "987",
            "artist": "654",
            "url": "321",
            "url_diff": "111"
        }
    ]);
    let data: BmsTableData = serde_json::from_value(data_json).unwrap();
    let [score] = data.charts.as_slice() else {
        panic!(
            "expected one chart, got {}: {:?}",
            data.charts.len(),
            data.charts
        );
    };
    assert_eq!(score.level, "0");
    assert_eq!(score.md5, Some("12345".to_string()));
    assert_eq!(score.sha256, Some("67890".to_string()));
    assert_eq!(score.title, Some("987".to_string()));
    assert_eq!(score.artist, Some("654".to_string()));
    assert_eq!(score.url, Some("321".to_string()));
    assert_eq!(score.url_diff, Some("111".to_string()));
}

#[test]
fn build_bms_table_extra_fields_ignored() {
    let header_json = json!({
        "name": "Test Table",
        "symbol": "test",
        "data_url": "charts.json"
    });
    let data_json = json!([
        {
            "level": "1",
            "id": 1
        }
    ]);
    let header: BmsTableHeader = serde_json::from_value(header_json).unwrap();
    let data: BmsTableData = serde_json::from_value(data_json).unwrap();
    let _bms_table = BmsTable { header, data };
}

#[test]
fn course_info_flat_list_deserialized_correctly() {
    let json_data = r#"{
        "name": "Test Table",
        "symbol": "test",
        "data_url": "score.json",
        "course": [
            {
                "name": "Course 1",
                "constraint": ["grade_mirror"],
                "trophy": [
                    {
                        "name": "goldmedal",
                        "missrate": 5.0,
                        "scorerate": 70.0
                    }
                ],
                "md5": ["abc123", "def456"]
            }
        ]
    }"#;

    let result: serde_json::Result<BmsTableHeader> = serde_json::from_str(json_data);
    assert!(result.is_ok());

    let header = result.unwrap();
    assert_eq!(header.name, "Test Table");
    assert_eq!(header.symbol, "test");
    assert_eq!(header.data_url, "score.json");

    // Input is `"course": [{...}]` — flat structure → Courses leaf with one element
    let course = match &header.course {
        CourseGroup::Courses(courses) => {
            let [course] = courses.as_slice() else {
                panic!("expected one course, got {}: {:?}", courses.len(), courses);
            };
            course
        }
        other => panic!("expected Courses, got {:?}", other),
    };
    assert_eq!(course.name.as_str(), "Course 1");
    assert_eq!(course.charts.len(), 2);
    let [c0, c1] = course.charts.as_slice() else {
        panic!(
            "expected two charts, got {}: {:?}",
            course.charts.len(),
            course.charts
        );
    };
    assert_eq!(c0.md5.as_deref(), Some("abc123"));
    assert_eq!(c1.md5.as_deref(), Some("def456"));
}

#[test]
fn course_info_nested_lists_deserialized_correctly() {
    let json_data = r#"{
        "name": "Test Table",
        "symbol": "test",
        "data_url": "score.json",
        "course": [
            [
                {
                    "name": "Course 1",
                    "constraint": ["grade_mirror"],
                    "trophy": [
                        {
                            "name": "goldmedal",
                            "missrate": 5.0,
                            "scorerate": 70.0
                        }
                    ],
                    "md5": ["abc123", "def456"]
                }
            ],
            [
                {
                    "name": "Course 2",
                    "constraint": ["ln"],
                    "trophy": [
                        {
                            "name": "silvermedal",
                            "missrate": 10.0,
                            "scorerate": 60.0
                        }
                    ],
                    "md5": ["ghi789"]
                }
            ]
        ]
    }"#;

    let result: serde_json::Result<BmsTableHeader> = serde_json::from_str(json_data);
    assert!(result.is_ok());

    let header = result.unwrap();
    assert_eq!(header.name, "Test Table");
    assert_eq!(header.symbol, "test");
    assert_eq!(header.data_url, "score.json");

    // Input is `"course": [[{...}], [{...}]]` — nested with two groups
    let (course1, course2) = match &header.course {
        CourseGroup::SubGroups(groups) => {
            let [g1, g2] = groups.as_slice() else {
                panic!("expected two groups, got {}: {:?}", groups.len(), groups);
            };
            let info1 = match g1 {
                CourseGroup::Courses(courses) => {
                    let [c] = courses.as_slice() else {
                        panic!("expected one course, got {}: {:?}", courses.len(), courses);
                    };
                    c
                }
                other => panic!("expected Courses, got {:?}", other),
            };
            let info2 = match g2 {
                CourseGroup::Courses(courses) => {
                    let [c] = courses.as_slice() else {
                        panic!("expected one course, got {}: {:?}", courses.len(), courses);
                    };
                    c
                }
                other => panic!("expected Courses, got {:?}", other),
            };
            (info1, info2)
        }
        other => panic!("expected SubGroups, got {:?}", other),
    };
    assert_eq!(course1.name.as_str(), "Course 1");
    assert_eq!(course2.name.as_str(), "Course 2");
    assert_eq!(course1.charts.len(), 2);
    let [c10, c11] = course1.charts.as_slice() else {
        panic!(
            "expected two charts, got {}: {:?}",
            course1.charts.len(),
            course1.charts
        );
    };
    assert_eq!(c10.md5.as_deref(), Some("abc123"));
    assert_eq!(c11.md5.as_deref(), Some("def456"));

    assert_eq!(course2.charts.len(), 1);
    let [c20] = course2.charts.as_slice() else {
        panic!(
            "expected one chart, got {}: {:?}",
            course2.charts.len(),
            course2.charts
        );
    };
    assert_eq!(c20.md5.as_deref(), Some("ghi789"));
}

#[test]
fn course_info_charts_missing_level_defaults_to_zero() {
    let json_data = r#"{
        "name": "Test Course",
        "constraint": ["grade_mirror"],
        "trophy": [
            {
                "name": "goldmedal",
                "missrate": 5.0,
                "scorerate": 70.0
            }
        ],
        "charts": [
            {
                "title": "Test Song",
                "artist": "Test Artist",
                "url": "https://example.com/test.bms"
            },
            {
                "level": "1",
                "title": "Test Song 2",
                "artist": "Test Artist 2",
                "url": "https://example.com/test2.bms"
            }
        ]
    }"#;

    let result: serde_json::Result<CourseInfo> = serde_json::from_str(json_data);
    assert!(result.is_ok());

    let course_info = result.unwrap();
    assert_eq!(course_info.name, "Test Course");
    assert_eq!(course_info.constraint, vec!["grade_mirror"]);
    assert_eq!(course_info.trophy.len(), 1);
    assert_eq!(course_info.charts.len(), 2);

    let [first_chart, second_chart] = course_info.charts.as_slice() else {
        panic!(
            "expected two charts, got {}: {:?}",
            course_info.charts.len(),
            course_info.charts
        );
    };
    assert_eq!(first_chart.level, "0");
    assert_eq!(first_chart.title, Some("Test Song".to_string()));
    assert_eq!(first_chart.artist, Some("Test Artist".to_string()));

    assert_eq!(second_chart.level, "1");
    assert_eq!(second_chart.title, Some("Test Song 2".to_string()));
    assert_eq!(second_chart.artist, Some("Test Artist 2".to_string()));
}

#[test]
fn course_info_sha256_list_converts_to_charts() {
    let json_data = r#"{
        "name": "Test Course",
        "constraint": ["grade_mirror"],
        "trophy": [
            {
                "name": "goldmedal",
                "missrate": 5.0,
                "scorerate": 70.0
            }
        ],
        "sha256": ["sha256_hash_1", "sha256_hash_2"]
    }"#;

    let result: serde_json::Result<CourseInfo> = serde_json::from_str(json_data);
    assert!(result.is_ok());

    let course_info = result.unwrap();
    assert_eq!(course_info.name, "Test Course");
    assert_eq!(course_info.constraint, vec!["grade_mirror"]);
    assert_eq!(course_info.trophy.len(), 1);
    assert_eq!(course_info.charts.len(), 2);

    let [c0, c1] = course_info.charts.as_slice() else {
        panic!(
            "expected two charts, got {}: {:?}",
            course_info.charts.len(),
            course_info.charts
        );
    };
    assert_eq!(c0.sha256.as_deref(), Some("sha256_hash_1"));
    assert_eq!(c1.sha256.as_deref(), Some("sha256_hash_2"));
    assert_eq!(c0.md5.as_deref(), None);
    assert_eq!(c1.md5.as_deref(), None);
}

#[test]
fn course_info_md5_sha256_both_convert_to_charts() {
    let json_data = r#"{
        "name": "Test Course",
        "constraint": ["grade_mirror"],
        "trophy": [
            {
                "name": "goldmedal",
                "missrate": 5.0,
                "scorerate": 70.0
            }
        ],
        "md5": ["md5_hash_1"],
        "sha256": ["sha256_hash_1"],
        "charts": [
            {
                "level": "2",
                "title": "Existing Chart",
                "artist": "Test Artist"
            }
        ]
    }"#;

    let result: serde_json::Result<CourseInfo> = serde_json::from_str(json_data);
    assert!(result.is_ok());

    let course_info = result.unwrap();
    assert_eq!(course_info.name, "Test Course");
    assert_eq!(course_info.constraint, vec!["grade_mirror"]);
    assert_eq!(course_info.trophy.len(), 1);
    assert_eq!(course_info.charts.len(), 3);

    let [existing, from_md5, from_sha256] = course_info.charts.as_slice() else {
        panic!(
            "expected three charts, got {}: {:?}",
            course_info.charts.len(),
            course_info.charts
        );
    };
    assert_eq!(existing.level.as_str(), "2");
    assert_eq!(existing.title.as_deref(), Some("Existing Chart"));
    assert_eq!(existing.artist.as_deref(), Some("Test Artist"));

    assert_eq!(from_md5.md5.as_deref(), Some("md5_hash_1"));
    assert_eq!(from_md5.level.as_str(), "0");

    assert_eq!(from_sha256.sha256.as_deref(), Some("sha256_hash_1"));
    assert_eq!(from_sha256.level.as_str(), "0");
}

#[test]
fn header_json_roundtrip_preserves_fields() {
    let header = bms_table::BmsTableHeader {
        name: "Test Table".to_string(),
        symbol: "test".to_string(),
        data_url: "charts.json".to_string(),
        tag: None,
        mode: None,
        course: CourseGroup::Courses(vec![]),
        level_order: vec!["0".to_string(), "1".to_string(), "!i".to_string()],
        extra: BTreeMap::new(),
    };

    let json = serde_json::to_string(&header).unwrap();
    let parsed: bms_table::BmsTableHeader = serde_json::from_str(&json).unwrap();
    assert_eq!(header, parsed);
    assert!(parsed.course.flatten().is_empty());
}

fn make_course_info(name: &str) -> serde_json::Value {
    json!({
        "name": name,
        "constraint": [],
        "trophy": [],
        "charts": [],
    })
}

#[test]
fn course_empty_flat_roundtrips_correctly() {
    let raw = json!({"name":"T","symbol":"t","data_url":"c.json","course":[],"level_order":[]});
    let h: BmsTableHeader = serde_json::from_value(raw).unwrap();
    assert!(matches!(&h.course, CourseGroup::Courses(v) if v.is_empty()));
    let out = serde_json::to_value(&h).unwrap();
    assert_eq!(out.get("course").unwrap(), &json!([]));
}

#[test]
fn course_single_flat_roundtrips_correctly() {
    let course = json!([make_course_info("C1")]);
    let raw = json!({"name":"T","symbol":"t","data_url":"c.json","course":course,"level_order":[]});
    let h: BmsTableHeader = serde_json::from_value(raw).unwrap();
    assert!(matches!(&h.course, CourseGroup::Courses(v) if v.len() == 1));
    let out = serde_json::to_value(&h).unwrap();
    assert_eq!(out.get("course").unwrap(), &course);
}

#[test]
fn course_multi_flat_roundtrips_correctly() {
    let course = json!([make_course_info("C1"), make_course_info("C2")]);
    let raw = json!({"name":"T","symbol":"t","data_url":"c.json","course":course,"level_order":[]});
    let h: BmsTableHeader = serde_json::from_value(raw).unwrap();
    assert!(matches!(&h.course, CourseGroup::Courses(v) if v.len() == 2));
    let out = serde_json::to_value(&h).unwrap();
    assert_eq!(out.get("course").unwrap(), &course);
}

#[test]
fn course_single_nested_roundtrips_correctly() {
    let course = json!([[make_course_info("C1")]]);
    let raw = json!({"name":"T","symbol":"t","data_url":"c.json","course":course,"level_order":[]});
    let h: BmsTableHeader = serde_json::from_value(raw).unwrap();
    assert!(matches!(&h.course, CourseGroup::SubGroups(g) if g.len() == 1));
    let out = serde_json::to_value(&h).unwrap();
    assert_eq!(out.get("course").unwrap(), &course);
}

#[test]
fn course_multi_nested_roundtrips_correctly() {
    let course = json!([[make_course_info("C1")], [make_course_info("C2")]]);
    let raw = json!({"name":"T","symbol":"t","data_url":"c.json","course":course,"level_order":[]});
    let h: BmsTableHeader = serde_json::from_value(raw).unwrap();
    assert!(matches!(&h.course, CourseGroup::SubGroups(g) if g.len() == 2));
    let out = serde_json::to_value(&h).unwrap();
    assert_eq!(out.get("course").unwrap(), &course);
}

#[test]
fn course_empty_nested_roundtrips_correctly() {
    let course = json!([[]]);
    let raw = json!({"name":"T","symbol":"t","data_url":"c.json","course":course,"level_order":[]});
    let h: BmsTableHeader = serde_json::from_value(raw).unwrap();
    let g = match &h.course {
        CourseGroup::SubGroups(g) => g,
        _ => panic!("expected SubGroups"),
    };
    assert!(matches!(&g.first().unwrap(), CourseGroup::Courses(v) if v.is_empty()));
    let out = serde_json::to_value(&h).unwrap();
    assert_eq!(out.get("course").unwrap(), &course);
}

#[test]
fn course_deeply_nested_roundtrips_correctly() {
    let course = json!([[[make_course_info("C1")]]]);
    let raw = json!({"name":"T","symbol":"t","data_url":"c.json","course":course,"level_order":[]});
    let h: BmsTableHeader = serde_json::from_value(raw).unwrap();
    assert!(matches!(&h.course, CourseGroup::SubGroups(g) if g.len() == 1));
    let out = serde_json::to_value(&h).unwrap();
    assert_eq!(out.get("course").unwrap(), &course);
}

#[test]
fn roundtrip_course_missing_defaults_to_empty() {
    let raw = json!({"name":"T","symbol":"t","data_url":"c.json","level_order":[]});
    let h: BmsTableHeader = serde_json::from_value(raw).unwrap();
    assert!(h.course.flatten().is_empty());
    assert!(matches!(&h.course, CourseGroup::Courses(v) if v.is_empty()));
}

#[test]
fn level_index_returns_correct_index() {
    let header = BmsTableHeader {
        name: "Test".into(),
        symbol: "t".into(),
        data_url: "c.json".into(),
        tag: None,
        mode: None,
        course: CourseGroup::default(),
        level_order: vec!["1".into(), "2".into(), "3".into(), "11+".into()],
        extra: BTreeMap::new(),
    };
    assert_eq!(header.level_index("1"), Some(0));
    assert_eq!(header.level_index("2"), Some(1));
    assert_eq!(header.level_index("3"), Some(2));
    assert_eq!(header.level_index("11+"), Some(3));
}

#[test]
fn level_index_missing_level_returns_none() {
    let header = BmsTableHeader {
        name: "Test".into(),
        symbol: "t".into(),
        data_url: "c.json".into(),
        tag: None,
        mode: None,
        course: CourseGroup::default(),
        level_order: vec!["1".into(), "2".into()],
        extra: BTreeMap::new(),
    };
    assert_eq!(header.level_index("3"), None);
    assert_eq!(header.level_index("0"), None);
}

#[test]
fn level_index_empty_level_order_returns_none() {
    let header = BmsTableHeader {
        name: "Test".into(),
        symbol: "t".into(),
        data_url: "c.json".into(),
        tag: None,
        mode: None,
        course: CourseGroup::default(),
        level_order: vec![],
        extra: BTreeMap::new(),
    };
    assert_eq!(header.level_index("1"), None);
}

#[test]
fn chart_item_new_fields_land_in_extra() {
    let data_json = json!([
        {
            "level": "1",
            "md5": "abc",
            "comment": "hard chart",
            "url_pack": "https://example.com/pack.zip",
            "name_pack": "Example Pack",
            "org_md5": "def123",
            "mode": "7K",
            "custom_extra": "still in extra"
        }
    ]);
    let data: BmsTableData = serde_json::from_value(data_json).unwrap();
    let [chart] = data.charts.as_slice() else {
        panic!("expected one chart, got {}", data.charts.len());
    };
    assert_eq!(chart.comment.as_deref(), Some("hard chart"));
    assert_eq!(
        chart.extra.get("url_pack"),
        Some(&json!("https://example.com/pack.zip"))
    );
    assert_eq!(chart.extra.get("name_pack"), Some(&json!("Example Pack")));
    assert_eq!(chart.extra.get("org_md5"), Some(&json!("def123")));
    assert_eq!(chart.extra.get("mode"), Some(&json!("7K")));
    assert_eq!(
        chart.extra.get("custom_extra"),
        Some(&json!("still in extra"))
    );
}

#[test]
fn chart_item_new_fields_default_to_none() {
    let data_json = json!([
        {
            "level": "1",
            "md5": "abc"
        }
    ]);
    let data: BmsTableData = serde_json::from_value(data_json).unwrap();
    let [chart] = data.charts.as_slice() else {
        panic!("expected one chart, got {}", data.charts.len());
    };
    assert!(chart.comment.is_none());
    assert!(!chart.extra.contains_key("url_pack"));
    assert!(!chart.extra.contains_key("name_pack"));
    assert!(!chart.extra.contains_key("org_md5"));
    assert!(!chart.extra.contains_key("mode"));
}

#[test]
fn course_group_flat_into_flatten_returns_courses() {
    let info = CourseInfo {
        name: "C1".into(),
        constraint: vec![],
        trophy: vec![],
        charts: vec![],
    };
    let group = CourseGroup::Courses(vec![info]);
    let flat = group.into_flatten();
    assert_eq!(flat.len(), 1);
    assert_eq!(flat.first().unwrap().name, "C1");
}

#[test]
fn course_group_nested_into_flatten_returns_courses() {
    let c1 = CourseInfo {
        name: "C1".into(),
        constraint: vec![],
        trophy: vec![],
        charts: vec![],
    };
    let c2 = CourseInfo {
        name: "C2".into(),
        constraint: vec![],
        trophy: vec![],
        charts: vec![],
    };
    let group = CourseGroup::SubGroups(vec![
        CourseGroup::Courses(vec![c1]),
        CourseGroup::Courses(vec![c2]),
    ]);
    let flat = group.into_flatten();
    assert_eq!(flat.len(), 2);
    assert_eq!(flat.first().unwrap().name, "C1");
    assert_eq!(flat.get(1).unwrap().name, "C2");
}

#[test]
fn course_group_empty_into_flatten_returns_empty_vec() {
    let group = CourseGroup::Courses(vec![]);
    assert!(group.into_flatten().is_empty());
}

#[test]
fn course_group_deeply_nested_into_flatten_returns_courses() {
    let c1 = CourseInfo {
        name: "C1".into(),
        constraint: vec![],
        trophy: vec![],
        charts: vec![],
    };
    let group = CourseGroup::SubGroups(vec![CourseGroup::SubGroups(vec![CourseGroup::Courses(
        vec![c1],
    )])]);
    let flat = group.into_flatten();
    assert_eq!(flat.len(), 1);
    assert_eq!(flat.first().unwrap().name, "C1");
}

#[test]
fn chart_data_empty_array_has_no_charts() {
    let data: BmsTableData = serde_json::from_value(json!([])).unwrap();
    assert!(data.charts.is_empty());
}

#[test]
fn chart_item_level_null_defaults_to_zero() {
    let data: BmsTableData =
        serde_json::from_value(json!([{ "level": null, "md5": "abc" }])).unwrap();
    let [chart] = data.charts.as_slice() else {
        panic!("expected one chart, got {}", data.charts.len());
    };
    assert_eq!(chart.level, "0");
    assert_eq!(chart.md5.as_deref(), Some("abc"));
}

#[test]
fn chart_item_level_zero_converts_to_string() {
    let data: BmsTableData = serde_json::from_value(json!([{ "level": 0, "md5": "abc" }])).unwrap();
    let [chart] = data.charts.as_slice() else {
        panic!("expected one chart, got {}", data.charts.len());
    };
    assert_eq!(chart.level, "0");
}

#[test]
fn course_chart_level_null_defaults_to_zero() {
    // When a course chart has `"level": null`, it should be treated the same
    // as when `level` is absent — both default to "0" per spec.
    let json_data = r#"{
        "name": "Test Course",
        "constraint": [],
        "trophy": [],
        "charts": [
            { "level": null, "title": "Null Level Chart" },
            { "title": "Missing Level Chart" }
        ]
    }"#;

    let course: CourseInfo = serde_json::from_str(json_data).unwrap();
    let [c0, c1] = course.charts.as_slice() else {
        panic!("expected two charts, got {}", course.charts.len());
    };
    assert_eq!(c0.level, "0", "null level should default to \"0\"");
    assert_eq!(c1.level, "0", "missing level should default to \"0\"");
}

#[test]
fn course_very_deeply_nested_roundtrips_correctly() {
    // 4 levels: [[[[C1]]]]
    let course = json!([[[[make_course_info("C1")]]]]);
    let raw = json!({"name":"T","symbol":"t","data_url":"c.json","course":course,"level_order":[]});
    let h: BmsTableHeader = serde_json::from_value(raw).unwrap();
    assert!(matches!(&h.course, CourseGroup::SubGroups(g) if g.len() == 1));
    let out = serde_json::to_value(&h).unwrap();
    assert_eq!(out.get("course").unwrap(), &course);
}

#[test]
fn course_group_default_serializes_as_empty_array() {
    let header = BmsTableHeader {
        name: "T".into(),
        symbol: "t".into(),
        data_url: "c.json".into(),
        tag: None,
        mode: None,
        course: CourseGroup::default(),
        level_order: vec![],
        extra: BTreeMap::new(),
    };
    let value = serde_json::to_value(&header).unwrap();
    assert_eq!(value.get("course").unwrap(), &json!([]));
}

#[test]
fn level_order_with_boolean_reports_error() {
    let raw = json!({"name":"T","symbol":"t","data_url":"c.json","level_order":["1", true, "3"]});
    let result: Result<BmsTableHeader, _> = serde_json::from_value(raw);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("level_order") || msg.contains("expected string or number"),
        "msg: {msg}"
    );
}

#[test]
fn level_order_omitted_defaults_to_empty() {
    let raw = json!({"name":"T","symbol":"t","data_url":"c.json"});
    let h: BmsTableHeader = serde_json::from_value(raw).unwrap();
    assert!(h.level_order.is_empty());
}

#[test]
fn level_order_with_null_value_reports_error() {
    let raw = json!({"name":"T","symbol":"t","data_url":"c.json","level_order":["1", null]});
    let result: Result<BmsTableHeader, _> = serde_json::from_value(raw);
    assert!(result.is_err());
}

#[test]
fn course_group_from_course_info_converts_correctly() {
    let info = CourseInfo {
        name: "C1".into(),
        constraint: vec![],
        trophy: vec![],
        charts: vec![],
    };
    let group: CourseGroup = info.into();
    match group {
        CourseGroup::Courses(v) => {
            assert_eq!(v.len(), 1);
            assert_eq!(v.first().unwrap().name, "C1");
        }
        _ => panic!("expected Courses variant"),
    }
}

#[test]
fn de_numstring_rejects_boolean_level() {
    let raw = json!([{ "level": true, "md5": "abc" }]);
    let result: Result<BmsTableData, _> = serde_json::from_value(raw);
    assert!(result.is_err());
}
