use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

pub struct UwUifier {
    pub emoticons: Vec<&'static str>,
    pub interjections: Vec<&'static str>,
    pub stutter_probability: f64,
    pub emoji_probability: f64,
    pub custom_map: HashMap<String, String>,
    pub emoji_map: HashMap<String, String>,
}

impl UwUifier {
    pub fn new() -> Self {
        Self {
            emoticons: vec!["(・`ω´・)", ";;w;;", "owo", "uwu", ">w<", "^w^"],
            interjections: vec!["rawr", "owo", "uwu", "hehe"],
            stutter_probability: 0.1,
            emoji_probability: 0.5,
            custom_map: HashMap::new(),
            emoji_map: HashMap::new(),
        }
    }

    pub fn uwuify(&self, input: &str) -> String {
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

        output.trim_end().to_string()
    }

    pub fn set_stutter_probability(&mut self, probability: f64) {
        self.stutter_probability = probability;
    }

    pub fn set_emoji_probability(&mut self, probability: f64) {
        self.emoji_probability = probability;
    }

    pub fn add_emoticon(&mut self, emoticon: &'static str) {
        self.emoticons.push(emoticon);
    }

    pub fn add_interjection(&mut self, interjection: &'static str) {
        self.interjections.push(interjection);
    }

    pub fn add_custom_mapping(&mut self, from: &str, to: &str) {
        mappings::add_custom_mapping(&mut self.custom_map, from, to);
    }

    pub fn add_emoji_mapping(&mut self, word: &str, emoji: &str) {
        mappings::add_emoji_mapping(&mut self.emoji_map, word, emoji);
    }

    pub fn leetify(&self, input: &str) -> String {
        transformations::leetify(input)
    }

    pub fn reverse_text(&self, input: &str) -> String {
        transformations::reverse_text(input)
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
