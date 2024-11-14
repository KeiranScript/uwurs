pub fn split_word_punctuation(word: &str) -> (String, String) {
    let mut chars = word.chars().peekable();
    let mut base_word = String::new();
    let mut punctuation = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() {
            base_word.push(c);
            chars.next();
        } else {
            break;
        }
    }

    while let Some(c) = chars.next() {
        punctuation.push(c);
    }

    (base_word, punctuation)
}
