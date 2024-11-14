use rand::seq::SliceRandom;
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
                '!' | '?' | '.' => {
                    output.push(c);
                    output.push(' ');
                    if rand::thread_rng().gen_bool(self.emoji_probability) {
                        output.push_str(
                            self.emoticons
                                .choose(&mut rand::thread_rng())
                                .unwrap_or(&"uwu"),
                        );
                    } else {
                        output.push_str(
                            self.interjections
                                .choose(&mut rand::thread_rng())
                                .unwrap_or(&"uwu"),
                        );
                    }
                }
                _ => output.push(c),
            }
        }

        let mut final_output = String::with_capacity(output.len());
        for word in output.split_whitespace() {
            let transformed_word = self.apply_custom_mappings(word);
            let word_with_stutter = self.apply_stutter(&transformed_word);
            final_output.push_str(&word_with_stutter);
            final_output.push(' ');
        }

        final_output.trim_end().to_string()
    }

    fn apply_custom_mappings(&self, word: &str) -> String {
        if let Some(replacement) = self.custom_map.get(word) {
            return replacement.clone();
        }
        if let Some(emoji) = self.emoji_map.get(word) {
            return emoji.clone();
        }
        word.to_string()
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
        input
            .replace("a", "4")
            .replace("e", "3")
            .replace("l", "1")
            .replace("o", "0")
            .replace("t", "7")
            .replace("s", "5")
    }

    pub fn reverse_text(&self, input: &str) -> String {
        input.chars().rev().collect()
    }
}
