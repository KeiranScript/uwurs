use uwurs::uwuifier::transformations::*;

#[test]
fn test_apply_character_transformations() {
    let input = "Hello World!";
    let output = apply_character_transformations(input);
    assert_eq!(output, "Hewwo Wowwd!");
}

#[test]
fn test_apply_stutter() {
    let input = "test";
    let output = apply_stutter(1.0, input);
    assert_eq!(output, "t-test");
}

#[test]
fn test_apply_stutter_no_stutter() {
    let input = "test";
    let output = apply_stutter(0.0, input);
    assert_eq!(output, "test");
}

#[test]
fn test_leetify() {
    let input = "Let's leetify this text.";
    let output = leetify(input);
    assert_eq!(output, "137'5 13371fy 7h15 73x7.");
}

#[test]
fn test_reverse_text() {
    let input = "Hello World!";
    let output = reverse_text(input);
    assert_eq!(output, "!dlroW olleH");
}

#[test]
fn test_random_caps() {
    let input = "Random caps test.";
    let output = random_caps(input);
    assert_ne!(output, input); // Output should be different from input due to random capitalization
}

#[test]
fn test_add_regex_mapping() {
    let mut regex_mappings = vec![];
    let pattern = r"hello";
    let replacement = "hewwo";
    let result = add_regex_mapping(&mut regex_mappings, pattern, replacement);
    assert!(result.is_ok());
    assert_eq!(regex_mappings.len(), 1);
    assert_eq!(regex_mappings[0].0.as_str(), pattern);
    assert_eq!(regex_mappings[0].1, replacement.to_string());
}

#[test]
fn test_apply_regex_mappings() {
    let mut regex_mappings = vec![];
    let pattern = r"hello";
    let replacement = "hewwo";
    add_regex_mapping(&mut regex_mappings, pattern, replacement).unwrap();
    let input = "hello world!";
    let output = apply_regex_mappings(input, &regex_mappings);
    assert_eq!(output, "hewwo world!");
}

#[test]
fn test_invalid_regex_mapping() {
    let mut regex_mappings = vec![];
    let pattern = r"(";
    let replacement = "invalid";
    let result = add_regex_mapping(&mut regex_mappings, pattern, replacement);
    assert!(result.is_err());
    if let Err(TransformationError::RegexError(_)) = result {
        // Expected error type
    } else {
        panic!("Expected RegexError");
    }
}
