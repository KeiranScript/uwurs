use rand::seq::SliceRandom;
use rand::Rng;

pub fn uwuify(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();
    let emoticons = ["(・`ω´・)", ";;w;;", "owo", "uwu", ">w<", "^w^"];
    let interjections = ["rawr", "owo", "uwu", "hehe"];

    while let Some(c) = chars.next() {
        match c {
            'r' | 'l' => output.push('w'),
            'R' | 'L' => output.push('W'),
            'n' | 'N' => {
                if let Some(&next_char) = chars.peek() {
                    if next_char.eq_ignore_ascii_case(&'a') || next_char.eq_ignore_ascii_case(&'o')
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
                if let Some(&'h') | Some(&'H') = chars.peek() {
                    output.push(if c.is_uppercase() { 'D' } else { 'd' });
                    chars.next();
                } else {
                    output.push(c);
                }
            }
            's' if chars.peek() == Some(&'h') => {
                output.push('s');
                output.push('h');
                chars.next();
            }
            'S' if chars.peek() == Some(&'h') => {
                output.push('S');
                output.push('h');
                chars.next();
            }
            '!' | '?' | '.' => {
                output.push(c);
                output.push(' ');
                if rand::thread_rng().gen_bool(0.5) {
                    output.push_str(emoticons.choose(&mut rand::thread_rng()).unwrap());
                } else {
                    output.push_str(interjections.choose(&mut rand::thread_rng()).unwrap());
                }
            }
            _ => output.push(c),
        }
    }

    let mut final_output = String::new();
    for word in output.split_whitespace() {
        if rand::thread_rng().gen_bool(0.1) {
            // 10% chance to stutter
            let first_char = word.chars().next().unwrap_or_default();
            final_output.push_str(&format!("{}-{} ", first_char, word));
        } else {
            final_output.push_str(word);
            final_output.push(' ');
        }
    }

    final_output.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uwuify_basic() {
        let input = "Hello World!";
        let output = uwuify(input);
        assert!(output.contains("Hewwo Wowwd"));
    }

    #[test]
    fn test_uwuify_stuttering() {
        let input = "This is a test.";
        let output = uwuify(input);
        assert!(!output.is_empty());
    }

    #[test]
    fn test_uwuify_emoticons() {
        let input = "Hello!";
        let output = uwuify(input);
        assert!(output.contains('!'));
    }
}
