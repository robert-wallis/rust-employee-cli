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
}
