// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved.

use super::Command;
use super::EmployeeError;

pub fn looks_like(first: &str) -> bool {
    first == "add"
}

pub fn command(tokens: &[&str]) -> Result<Command, EmployeeError> {
    let mut to = 0;
    for (i, token) in tokens.iter().enumerate() {
        if token.to_lowercase() == "to" {
            to = i
        }
    }
    if to == 0 {
        return Err(EmployeeError::DontUnderstand(String::from(
            "Missing \"to\" seperator. (ex. Add Alice to Engineering).",
        )));
    }
    let person: String = trim_non_alphabetic(&tokens[1..to - 1]).join("");
    let department: String = trim_non_alphabetic(&tokens[to + 1..]).join("");
    return if person.is_empty() || department.is_empty() {
        Err(EmployeeError::DontUnderstand(String::from("Missing person or department.")))
    } else {
        Ok(Command::Add { person, department })
    }
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
mod test {
    use super::*;

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
        assert_eq!(vec!["A", "brown", "cow"], trim_non_alphabetic(&vec!["A", "brown", "cow"]));
        assert_eq!(vec!["A", "brown", "cow"], trim_non_alphabetic(&vec!["A", "brown", "cow", "."]));
        assert_eq!(vec!["A", " ", "brown", " ", "cow"], trim_non_alphabetic(&vec!["A", " ", "brown", " ", "cow", "."]));
        assert_eq!(vec!["A", " ", "brown", " ", "cow"], trim_non_alphabetic(&vec![" ", "A", " ", "brown", " ", "cow", "."]));
        assert_eq!(vec!["A", " ", "brown", " ", "cow"], trim_non_alphabetic(&vec![" ", "A", " ", "brown", " ", "cow", ".", "\n"]));
    }
}