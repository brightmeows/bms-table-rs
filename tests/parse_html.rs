//! Unit tests for HTML parsing and bmstable URL extraction
//!
//! Verifies reading the `content` from `<meta name="bmstable">`.

use bms_table::BmsTableHtml;

#[test]
fn extract_bmstable_from_meta_name_returns_url() {
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

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "header.json");
}

#[test]
fn extract_bmstable_from_meta_property_returns_url() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta property="bmstable" content="https://example.com/header.json">
    </head>
    <body></body>
    </html>
    "#;

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "https://example.com/header.json");
}

#[test]
fn no_bmstable_returns_error() {
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

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_err());
}

#[test]
fn first_matching_meta_is_returned_when_multiple_present() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta name="bmstable" content="first.json">
        <meta name="bmstable" content="second.json">
    </head>
    </html>
    "#;

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "first.json");
}

#[test]
fn matching_meta_before_non_matching_returns_correct_url() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta name="other" content="other.json">
        <meta name="bmstable" content="header.json">
    </head>
    </html>
    "#;

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "header.json");
}

#[test]
fn empty_content_returns_error() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta name="bmstable" content="">
    </head>
    </html>
    "#;

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_err());
}

#[test]
fn case_insensitive_tag_and_attribute_names() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <META NAME="BMSTABLE" CONTENT="header.json">
    </head>
    </html>
    "#;

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "header.json");
}

#[test]
fn mixed_case_meta_name() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <Meta Name="Bmstable" Content="header.json">
    </head>
    </html>
    "#;

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "header.json");
}

#[test]
fn single_quoted_attributes() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta name='bmstable' content='header.json'>
    </head>
    </html>
    "#;

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "header.json");
}

#[test]
fn attribute_order_variation() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta content="header.json" name="bmstable">
    </head>
    </html>
    "#;

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "header.json");
}

#[test]
fn extra_attributes_do_not_interfere() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="utf-8" name="bmstable" content="header.json">
    </head>
    </html>
    "#;

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "header.json");
}

#[test]
fn commented_out_meta_tag_is_ignored() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <!-- <meta name="bmstable" content="ignored.json"> -->
        <meta name="bmstable" content="real.json">
    </head>
    </html>
    "#;

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "real.json");
}

#[test]
fn meta_only_in_comment_returns_error() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <!-- <meta name="bmstable" content="ignored.json"> -->
    </head>
    </html>
    "#;

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_err());
}

#[test]
fn self_closing_meta_tag() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta name="bmstable" content="header.json" />
    </head>
    </html>
    "#;

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "header.json");
}

#[test]
fn url_with_colon_and_slashes() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta name="bmstable" content="https://example.com/path/to/header.json">
    </head>
    </html>
    "#;

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_ok());
    let url = result.unwrap();
    assert_eq!(url, "https://example.com/path/to/header.json");
}

#[test]
fn property_attribute_mixed_case() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <META PROPERTY="BMSTABLE" CONTENT="header.json">
    </head>
    </html>
    "#;

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "header.json");
}

#[test]
fn only_bmstable_content_attribute_returns_error() {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta content="header.json">
    </head>
    </html>
    "#;

    let result = BmsTableHtml::extract_url(html);
    assert!(result.is_err());
}
