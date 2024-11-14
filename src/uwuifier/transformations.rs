use rand::Rng;

pub fn apply_character_transformations(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

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

pub fn apply_stutter(stutter_probability: f64, word: &str) -> String {
    if rand::thread_rng().gen_bool(stutter_probability) {
        let first_char = word.chars().next().unwrap_or_default();
        format!("{}-{}", first_char, word)
    } else {
        word.to_string()
    }
}

pub fn leetify(input: &str) -> String {
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

pub fn reverse_text(input: &str) -> String {
    input.chars().rev().collect()
}
