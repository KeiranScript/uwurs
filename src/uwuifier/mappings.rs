use std::collections::HashMap;

pub fn add_custom_mapping(custom_map: &mut HashMap<String, String>, from: &str, to: &str) {
    custom_map.insert(from.to_string(), to.to_string());
}

pub fn add_emoji_mapping(emoji_map: &mut HashMap<String, String>, word: &str, emoji: &str) {
    emoji_map.insert(word.to_string(), emoji.to_string());
}

pub fn apply_custom_mappings(
    custom_map: &HashMap<String, String>,
    emoji_map: &HashMap<String, String>,
    word: &str,
) -> String {
    if let Some(replacement) = custom_map.get(word) {
        replacement.clone()
    } else if let Some(emoji) = emoji_map.get(word) {
        emoji.clone()
    } else {
        word.to_string()
    }
}
