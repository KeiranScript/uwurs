use rand::Rng;
use std::collections::HashMap;

pub struct UwUifier {
    emoticons: Vec<&'static str>,
    interjections: Vec<&'static str>,
    stutter_probability: f64,
    emoji_probability: f64,
    custom_map: HashMap<String, String>,
    emoji_map: HashMap<String, String>,
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

        for word in words {
            let (base_word, punctuation) = Self::split_word_punctuation(word);
            let transformed_word = self.apply_custom_mappings(&base_word);
            let transformed_word = self.apply_character_transformations(&transformed_word);
            let word_with_stutter = self.apply_stutter(&transformed_word);
            output.push_str(&word_with_stutter);
            output.push_str(&punctuation);
            output.push(' ');
        }

        output.trim_end().to_string()
    }

    fn split_word_punctuation(word: &str) -> (String, String) {
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

    fn apply_custom_mappings(&self, word: &str) -> String {
        if let Some(replacement) = self.custom_map.get(word) {
            replacement.clone()
        } else if let Some(emoji) = self.emoji_map.get(word) {
            emoji.clone()
        } else {
            word.to_string()
        }
    }

    fn apply_character_transformations(&self, input: &str) -> String {
        let mut output = String::with_capacity(input.len());
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                'r' | 'l' => output.push('w'),
                'R' | 'L' => output.push('W'),
                'n' | 'N' => {
                    if let Some(&next_char) = chars.peek() {
                        if next_char.eq_ignore_ascii_case(&'a')
                            || next_char.eq_ignore_ascii_case(&'o')
                        {
                            output.push(c);
                            output.push('y');
                            chars.next();
                            output.push(next_char);
                        } else {
                            output.push(c);
                        }
                    } else {
                        output.push(c);
                    }
                }
                't' | 'T' => {
                    if let Some(&next_char) = chars.peek() {
                        if next_char.eq_ignore_ascii_case(&'h') {
                            output.push(if c.is_uppercase() { 'D' } else { 'd' });
                            chars.next();
                        } else {
                            output.push(c);
                        }
                    } else {
                        output.push(c);
                    }
                }
                's' | 'S' => {
                    if let Some(&next_char) = chars.peek() {
                        if next_char.eq_ignore_ascii_case(&'h') {
                            output.push(c);
                            output.push('h');
                            chars.next();
                        } else {
                            output.push(c);
                        }
                    } else {
                        output.push(c);
                    }
                }
                _ => output.push(c),
            }
        }

        output
    }

    fn apply_stutter(&self, word: &str) -> String {
        if rand::thread_rng().gen_bool(self.stutter_probability) {
            let first_char = word.chars().next().unwrap_or_default();
            format!("{}-{}", first_char, word)
        } else {
            word.to_string()
        }
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
        self.custom_map.insert(from.to_string(), to.to_string());
    }

    pub fn add_emoji_mapping(&mut self, word: &str, emoji: &str) {
        self.emoji_map.insert(word.to_string(), emoji.to_string());
    }

    pub fn leetify(&self, input: &str) -> String {
        let leet_map = [
            ('a', '4'),
            ('e', '3'),
            ('i', '1'),
            ('l', '1'),
            ('o', '0'),
            ('t', '7'),
            ('s', '5'),
        ];
        input
            .chars()
            .map(|c| {
                let lower_c = c.to_ascii_lowercase();
                leet_map
                    .iter()
                    .find(|&&(k, _)| k == lower_c)
                    .map(|&(_, v)| v)
                    .unwrap_or(c)
            })
            .collect()
    }

    pub fn reverse_text(&self, input: &str) -> String {
        input.chars().rev().collect()
    }
}
