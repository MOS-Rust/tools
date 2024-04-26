use std::collections::HashMap;
use regex::Regex;

// TODO: Improve the regex pattern
const PATTERN: &str = "const_export_[a-zA-Z0-9]+!\\(\\s{0,}([a-zA-Z_][a-zA-Z0-9_]+)\\s{0,},\\s{0,}\"?([a-zA-Z0-9\\$]+(:?\\s?,\\s?[0-9]+)?)\\s{0,}\"?\\);";

/// Find all `const_export_*` macros and return a map
pub fn get_const_export_map(lines: &Vec<String>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let re = Regex::new(PATTERN).unwrap();
    for line in lines {
        if let Some(caps) = re.captures(line) {
            let key = caps.get(1).unwrap().as_str().to_string();
            let value = caps.get(2).unwrap().as_str().to_string();
            map.insert(key, value);
        }
    }
    map
}

/// Replace all `const_export_*` macros with the corresponding values
pub fn replace_const_export(lines: &Vec<String>, map: &HashMap<String, String>) -> Vec<String> {
    let mut new_lines = Vec::new();
    for line in lines {
        let mut new_line = String::new();
        let tokens = tokenize(line);
        for token in tokens {
            match token {
                Token::Word(word) => {
                    if let Some(value) = map.get(&word) {
                        new_line.push_str(value);
                    } else {
                        new_line.push_str(&word);
                    }
                }
                Token::Other(other) => {
                    new_line.push_str(&other);
                }
            }
        }
        new_lines.push(new_line);
    }
    new_lines
}

enum Token {
    Word(String),
    Other(String),
}

fn tokenize(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut word = String::new();
    let mut in_word = false;
    for c in line.chars() {
        if c.is_alphanumeric() || c == '_' {
            word.push(c);
            in_word = true;
        } else {
            if in_word {
                tokens.push(Token::Word(word.clone()));
                word.clear();
                in_word = false;
            }
            tokens.push(Token::Other(c.to_string()));
        }
    }
    if in_word {
        tokens.push(Token::Word(word));
    }
    tokens
}