use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum MappingError {
    EmptyKey,
    EmptyValue,
}

pub fn add_custom_mapping(
    custom_map: &mut HashMap<String, String>,
    from: &str,
    to: &str,
) -> Result<(), MappingError> {
    if from.is_empty() {
        return Err(MappingError::EmptyKey);
    }
    if to.is_empty() {
        return Err(MappingError::EmptyValue);
    }
    custom_map.insert(from.to_string(), to.to_string());
    Ok(())
}

pub fn add_emoji_mapping(
    emoji_map: &mut HashMap<String, String>,
    word: &str,
    emoji: &str,
) -> Result<(), MappingError> {
    if word.is_empty() {
        return Err(MappingError::EmptyKey);
    }
    if emoji.is_empty() {
        return Err(MappingError::EmptyValue);
    }
    emoji_map.insert(word.to_string(), emoji.to_string());
    Ok(())
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
