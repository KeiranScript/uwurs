use uwurs::UwUifier;

#[test]
fn test_custom_mappings() {
    let mut uwuifier = UwUifier::new();
    uwuifier.add_custom_mapping("Test", "UwU");
    let input = "This is a Test.";
    let output = uwuifier.uwuify(input);
    assert!(output.contains("UwU"));
}

#[test]
fn test_emoji_mappings() {
    let mut uwuifier = UwUifier::new();
    uwuifier.add_emoji_mapping("love", "❤️");
    let input = "I love Rust!";
    let output = uwuifier.uwuify(input);
    assert!(output.contains("❤️"));
}

#[test]
fn test_leetify() {
    let uwuifier = UwUifier::new();
    let input = "Let's leetify this text.";
    let output = uwuifier.leetify(input);
    assert_eq!(output, "L37'5 13371fy 7hi5 73x7.");
}

#[test]
fn test_reverse_text() {
    let uwuifier = UwUifier::new();
    let input = "Hello World!";
    let output = uwuifier.reverse_text(input);
    assert_eq!(output, "!dlroW olleH");
}
