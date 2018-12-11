// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved.

/// Given a string, splits it into tokens, keeping the characters.
pub fn tokenize(line: &str) -> Vec<&str> {
    let mut words: Vec<&str> = vec![];
    let mut start = 0;
    for (i, c) in line.char_indices() {
        if !c.is_alphabetic() {
            if start != i {
                words.push(&line[start..i]);
            }
            words.push(&line[i..=i]);
            start = i + 1;
        }
    }
    if start < line.len() {
        words.push(&line[start..]);
    }
    words
}

pub fn trim_non_alphabetic<'a>(tokens: &'a [&str]) -> &'a [&'a str] {
    if tokens.is_empty() {
        return &tokens;
    }
    let mut start = 0;
    let mut end = tokens.len() - 1;
    for (i, token) in tokens.iter().enumerate() {
        if is_alphabetic(&token) {
            start = i;
            break;
        }
    }
    for (i, token) in tokens.iter().rev().enumerate() {
        if is_alphabetic(&token) {
            end = tokens.len() - i;
            break;
        }
    }
    &tokens[start..end]
}

fn is_alphabetic(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphabetic())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_happy() {
        assert_eq!(
            vec!["A", " ", "neat", "-", "o", " ", "sentence", "."],
            tokenize("A neat-o sentence.")
        );
    }
    #[test]
    fn test_is_alphabetic() {
        assert!(is_alphabetic("a"));
        assert!(is_alphabetic("abc"));
        assert!(is_alphabetic("AbC"));
        assert!(!is_alphabetic(""));
        assert!(!is_alphabetic(" "));
        assert!(!is_alphabetic("1"));
        assert!(!is_alphabetic("a-a"));
        assert!(!is_alphabetic("."));
    }

    #[test]
    fn test_trim_non_alphabetic() {
        assert_eq!(
            vec!["A", "brown", "cow"],
            trim_non_alphabetic(&vec!["A", "brown", "cow"])
        );
        assert_eq!(
            vec!["A", "brown", "cow"],
            trim_non_alphabetic(&vec!["A", "brown", "cow", "."])
        );
        assert_eq!(
            vec!["A", " ", "brown", " ", "cow"],
            trim_non_alphabetic(&vec!["A", " ", "brown", " ", "cow", "."])
        );
        assert_eq!(
            vec!["A", " ", "brown", " ", "cow"],
            trim_non_alphabetic(&vec![" ", "A", " ", "brown", " ", "cow", "."])
        );
        assert_eq!(
            vec!["A", " ", "brown", " ", "cow"],
            trim_non_alphabetic(&vec![" ", "A", " ", "brown", " ", "cow", ".", "\n"])
        );
    }
}
