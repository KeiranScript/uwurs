use uwurs::uwuifier::{transformations, UwUifier, UwUifierError};

#[test]
fn test_uwuify_basic() {
    let uwuifier = UwUifier::new();
    let input = "Hello World!";
    let output = uwuifier.uwuify(input).unwrap();
    assert!(output.contains("Hewwo") && output.contains("Wowwd"));
}

#[test]
fn test_uwuify_stuttering() {
    let uwuifier = UwUifier::new();
    let input = "This is a test.";
    let output = uwuifier.uwuify(input).unwrap();
    assert!(!output.is_empty());
}

#[test]
fn test_uwuify_emoticons() {
    let uwuifier = UwUifier::new();
    let input = "Hello!";
    let output = uwuifier.uwuify(input).unwrap();
    assert!(output.contains('!'));
}

#[test]
fn test_custom_mappings() {
    let mut uwuifier = UwUifier::new();
    uwuifier.add_custom_mapping("Test", "UwU");
    let input = "This is a Test.";
    let output = uwuifier.uwuify(input).unwrap();
    assert!(output.contains("UwU"));
}

#[test]
fn test_emoji_mappings() {
    let mut uwuifier = UwUifier::new();
    uwuifier.add_emoji_mapping("love", "❤️");
    let input = "I love Rust!";
    let output = uwuifier.uwuify(input).unwrap();
    assert!(output.contains("❤️"));
}

#[test]
fn test_leetify() {
    let uwuifier = UwUifier::new();
    let input = "Let's leetify this text.";
    let output = uwuifier.leetify(input).unwrap();
    assert_eq!(output, "137'5 13371fy 7h15 73x7.");
}

#[test]
fn test_reverse_text() {
    let uwuifier = UwUifier::new();
    let input = "Hello World!";
    let output = uwuifier.reverse_text(input).unwrap();
    assert_eq!(output, "!dlroW olleH");
}

#[test]
fn test_random_caps() {
    let uwuifier = UwUifier::new();
    let input = "Random caps test.";
    let output = uwuifier.random_caps(input).unwrap();
    assert_ne!(output, input);
}

#[test]
fn test_add_emoticon() {
    let mut uwuifier = UwUifier::new();
    uwuifier.add_emoticon("UwU");
    assert!(uwuifier.get_emoticons().contains(&"UwU"));
}

#[test]
fn test_add_interjection() {
    let mut uwuifier = UwUifier::new();
    uwuifier.add_interjection("nya");
    assert!(uwuifier.get_interjections().contains(&"nya"));
}

#[test]
fn test_set_stutter_probability() {
    let mut uwuifier = UwUifier::new();
    uwuifier.set_stutter_probability(0.3);
    assert_eq!(uwuifier.get_stutter_probability(), 0.3);
}

#[test]
fn test_set_emoji_probability() {
    let mut uwuifier = UwUifier::new();
    uwuifier.set_emoji_probability(0.7);
    assert_eq!(uwuifier.get_emoji_probability(), 0.7);
}

#[test]
fn test_uwuify_empty_input() {
    let uwuifier = UwUifier::new();
    let result = uwuifier.uwuify("");
    assert!(result.is_err());
    if let Err(UwUifierError::InvalidInput(msg)) = result {
        assert_eq!(msg, "Input string is empty");
    }
}

#[test]
fn test_uwuify_valid_input() {
    let uwuifier = UwUifier::new();
    let input = "Hello World!";
    let result = uwuifier.uwuify(input);
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Hewwo"));
}

#[test]
fn test_apply_character_transformations() {
    let input = "Hello World!";
    let output = transformations::apply_character_transformations(input);
    assert!(output.contains("Hewwo") || output.contains("Wowwd"));
}

#[test]
fn test_apply_stutter() {
    let input = "test";
    let output = transformations::apply_stutter(1.0, input);
    assert_eq!(output, "t-test");
}

#[test]
fn test_apply_stutter_no_stutter() {
    let input = "test";
    let output = transformations::apply_stutter(0.0, input);
    assert_eq!(output, "test");
}

#[test]
fn test_add_regex_mapping() {
    let mut regex_mappings = vec![];
    let pattern = r"hello";
    let replacement = "hewwo";
    let result = transformations::add_regex_mapping(&mut regex_mappings, pattern, replacement);
    assert!(result.is_ok());
    assert_eq!(regex_mappings.len(), 1);
}

#[test]
fn test_apply_regex_mappings() {
    let mut regex_mappings = vec![];
    let pattern = r"hello";
    let replacement = "hewwo";
    transformations::add_regex_mapping(&mut regex_mappings, pattern, replacement).unwrap();
    let input = "hello world!";
    let output = transformations::apply_regex_mappings(input, &regex_mappings);
    assert_eq!(output, "hewwo world!");
}
