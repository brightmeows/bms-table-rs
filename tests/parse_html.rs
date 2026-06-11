//! Unit tests for HTML parsing and bmstable URL extraction
//!
//! Verifies reading the `content` from `<meta name="bmstable">`.

use bms_table::BmsTableHtml;

#[test]
fn test_extract_bmstable_from_meta() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta name="bmstable" content="header.json">
    </head>
    <body>
        <h1>BMS Table</h1>
    </body>
    </html>
    "#;

    let result = BmsTableHtml::try_extract_bmstable_from_html(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "header.json");
}

#[test]
fn test_extract_bmstable_from_meta_property() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta property="bmstable" content="https://example.com/header.json">
    </head>
    <body></body>
    </html>
    "#;

    let result = BmsTableHtml::try_extract_bmstable_from_html(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "https://example.com/header.json");
}

#[test]
fn test_no_bmstable_returns_error() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>BMS Table</title>
    </head>
    <body>
        <h1>BMS Table</h1>
    </body>
    </html>
    "#;

    let result = BmsTableHtml::try_extract_bmstable_from_html(html);
    assert!(result.is_err());
}
