use rand::seq::SliceRandom;
use rand::Rng;
use regex;
use std::collections::HashMap;

pub struct UwUifier {
    pub emoticons: Vec<&'static str>,
    pub interjections: Vec<&'static str>,
    pub stutter_probability: f64,
    pub emoji_probability: f64,
    pub custom_map: HashMap<String, String>,
    pub emoji_map: HashMap<String, String>,
    pub regex_mappings: Vec<(regex::Regex, String)>,
}

pub enum TransformationType {
    UwUify,
    Leetify,
    ReverseText,
    RandomCaps,
}

#[derive(Debug)]
pub enum UwUifierError {
    InvalidInput(String),
    TransformationFailed(String),
    RegexError(regex::Error),
}

impl From<regex::Error> for UwUifierError {
    fn from(err: regex::Error) -> UwUifierError {
        UwUifierError::RegexError(err)
    }
}

pub type Result<T> = std::result::Result<T, UwUifierError>;

impl UwUifier {
    pub fn new() -> Self {
        Self {
            emoticons: vec!["(・`ω´・)", ";;w;;", "owo", "uwu", ">w<", "^w^"],
            interjections: vec!["rawr", "owo", "uwu", "hehe"],
            stutter_probability: 0.1,
            emoji_probability: 0.5,
            custom_map: HashMap::new(),
            emoji_map: HashMap::new(),
            regex_mappings: Vec::new(),
        }
    }

    pub fn uwuify(&self, input: &str) -> Result<String> {
        if input.is_empty() {
            return Err(UwUifierError::InvalidInput(
                "Input string is empty".to_string(),
            ));
        }

        let mut output = String::with_capacity(input.len());
        let words: Vec<&str> = input.split_whitespace().collect();
        let mut rng = rand::thread_rng();

        for word in words {
            let (base_word, punctuation) = utils::split_word_punctuation(word);
            let transformed_word =
                mappings::apply_custom_mappings(&self.custom_map, &self.emoji_map, &base_word);
            let transformed_word =
                transformations::apply_character_transformations(&transformed_word);
            let word_with_stutter =
                transformations::apply_stutter(self.stutter_probability, &transformed_word);

            output.push_str(&word_with_stutter);
            output.push_str(&punctuation);

            if rng.gen_bool(self.emoji_probability) {
                if let Some(emoji) = self.emoticons.choose(&mut rng) {
                    output.push(' ');
                    output.push_str(emoji);
                }
            }

            output.push(' ');

            if rng.gen_bool(self.emoji_probability) {
                if let Some(interjection) = self.interjections.choose(&mut rng) {
                    output.push_str(interjection);
                    output.push(' ');
                }
            }
        }

        Ok(output.trim_end().to_string())
    }

    pub fn set_stutter_probability(&mut self, probability: f64) {
        self.stutter_probability = probability;
    }

    pub fn get_stutter_probability(&self) -> f64 {
        self.stutter_probability
    }

    pub fn set_emoji_probability(&mut self, probability: f64) {
        self.emoji_probability = probability;
    }

    pub fn get_emoji_probability(&self) -> f64 {
        self.emoji_probability
    }

    pub fn add_emoticon(&mut self, emoticon: &'static str) {
        self.emoticons.push(emoticon);
    }

    pub fn get_emoticons(&self) -> &Vec<&'static str> {
        &self.emoticons
    }

    pub fn add_interjection(&mut self, interjection: &'static str) {
        self.interjections.push(interjection);
    }

    pub fn get_interjections(&self) -> &Vec<&'static str> {
        &self.interjections
    }

    pub fn add_custom_mapping(&mut self, from: &str, to: &str) {
        mappings::add_custom_mapping(&mut self.custom_map, from, to).unwrap();
    }

    pub fn get_custom_map(&self) -> &HashMap<String, String> {
        &self.custom_map
    }

    pub fn add_emoji_mapping(&mut self, word: &str, emoji: &str) {
        mappings::add_emoji_mapping(&mut self.emoji_map, word, emoji).unwrap();
    }

    pub fn get_emoji_map(&self) -> &HashMap<String, String> {
        &self.emoji_map
    }

    pub fn add_regex_mapping(&mut self, pattern: &str, replacement: &str) -> Result<()> {
        let re = regex::Regex::new(pattern)?;
        self.regex_mappings.push((re, replacement.to_string()));
        Ok(())
    }

    pub fn leetify(&self, input: &str) -> Result<String> {
        if input.is_empty() {
            return Err(UwUifierError::InvalidInput(
                "Input string is empty".to_string(),
            ));
        }
        Ok(transformations::leetify(input))
    }

    pub fn reverse_text(&self, input: &str) -> Result<String> {
        if input.is_empty() {
            return Err(UwUifierError::InvalidInput(
                "Input string is empty".to_string(),
            ));
        }
        Ok(transformations::reverse_text(input))
    }

    pub fn random_caps(&self, input: &str) -> Result<String> {
        if input.is_empty() {
            return Err(UwUifierError::InvalidInput(
                "Input string is empty".to_string(),
            ));
        }
        Ok(transformations::random_caps(input))
    }

    pub fn uppercase(&self, input: &str) -> String {
        input.to_uppercase()
    }

    pub fn lowercase(&self, input: &str) -> String {
        input.to_lowercase()
    }

    pub fn titlecase(&self, input: &str) -> String {
        input
            .split_whitespace()
            .map(|word| {
                let mut c = word.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn batch_uwuify(&self, inputs: Vec<&str>) -> Vec<Result<String>> {
        inputs.into_iter().map(|input| self.uwuify(input)).collect()
    }
}

impl Default for UwUifier {
    fn default() -> Self {
        Self::new()
    }
}

pub mod mappings;
pub mod transformations;
pub mod utils;
