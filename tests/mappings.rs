use std::collections::HashMap;
use uwurs::uwuifier::mappings::*;

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

#[test]
fn test_add_custom_mapping() {
    let mut custom_map = HashMap::new();
    assert_eq!(add_custom_mapping(&mut custom_map, "hello", "hi"), Ok(()));
    assert_eq!(custom_map.get("hello"), Some(&"hi".to_string()));
}

#[test]
fn test_add_emoji_mapping() {
    let mut emoji_map = HashMap::new();
    assert_eq!(add_emoji_mapping(&mut emoji_map, "love", "❤️"), Ok(()));
    assert_eq!(emoji_map.get("love"), Some(&"❤️".to_string()));
}

#[test]
fn test_apply_custom_mappings() {
    let mut custom_map = HashMap::new();
    let mut emoji_map = HashMap::new();
    add_custom_mapping(&mut custom_map, "hello", "hi").unwrap();
    add_emoji_mapping(&mut emoji_map, "love", "❤️").unwrap();
    assert_eq!(
        apply_custom_mappings(&custom_map, &emoji_map, "hello"),
        "hi"
    );
    assert_eq!(apply_custom_mappings(&custom_map, &emoji_map, "love"), "❤️");
    assert_eq!(
        apply_custom_mappings(&custom_map, &emoji_map, "world"),
        "world".to_string()
    );
}

#[test]
fn test_add_custom_mapping_empty_key() {
    let mut custom_map = HashMap::new();
    assert_eq!(
        add_custom_mapping(&mut custom_map, "", "hi"),
        Err(MappingError::EmptyKey)
    );
}

#[test]
fn test_add_custom_mapping_empty_value() {
    let mut custom_map = HashMap::new();
    assert_eq!(
        add_custom_mapping(&mut custom_map, "hello", ""),
        Err(MappingError::EmptyValue)
    );
}

#[test]
fn test_add_emoji_mapping_empty_key() {
    let mut emoji_map = HashMap::new();
    assert_eq!(
        add_emoji_mapping(&mut emoji_map, "", "❤️"),
        Err(MappingError::EmptyKey)
    );
}

#[test]
fn test_add_emoji_mapping_empty_value() {
    let mut emoji_map = HashMap::new();
    assert_eq!(
        add_emoji_mapping(&mut emoji_map, "love", ""),
        Err(MappingError::EmptyValue)
    );
}
