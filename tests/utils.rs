use uwurs::utils::split_word_punctuation;

#[test]
fn test_split_word_punctuation_empty() {
    let input = "";
    let (base_word, punctuation) = split_word_punctuation(input);
    assert_eq!(base_word, "");
    assert_eq!(punctuation, "");
}

#[test]
fn test_split_word_punctuation_no_punctuation() {
    let input = "hello";
    let (base_word, punctuation) = split_word_punctuation(input);
    assert_eq!(base_word, "hello");
    assert_eq!(punctuation, "");
}

#[test]
fn test_split_word_punctuation_with_punctuation() {
    let input = "hello!";
    let (base_word, punctuation) = split_word_punctuation(input);
    assert_eq!(base_word, "hello");
    assert_eq!(punctuation, "!");
}

#[test]
fn test_split_word_punctuation_only_punctuation() {
    let input = "!!!";
    let (base_word, punctuation) = split_word_punctuation(input);
    assert_eq!(base_word, "");
    assert_eq!(punctuation, "!!!");
}

#[test]
fn test_split_word_punctuation_mixed() {
    let input = "hello!!!world";
    let (base_word, punctuation) = split_word_punctuation(input);
    assert_eq!(base_word, "hello");
    assert_eq!(punctuation, "!!!world");
}

#[test]
fn test_split_word_punctuation_ends_with_punctuation() {
    let input = "word.";
    let (base_word, punctuation) = split_word_punctuation(input);
    assert_eq!(base_word, "word");
    assert_eq!(punctuation, ".");
}
