// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved.

use super::tokenizer;
use super::Command;
use super::EmployeeError;

const MISSING_TO: &str = "Missing \"to\" seperator. (ex. Add Alice to Engineering).";
const MISSING_NOUN: &str = "Missing person or department.";

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
        return Err(EmployeeError::DontUnderstand(String::from(MISSING_TO)));
    }
    let person: String = tokenizer::trim_non_alphabetic(&tokens[1..to - 1]).join("");
    let department: String = tokenizer::trim_non_alphabetic(&tokens[to + 1..]).join("");
    if person.is_empty() || department.is_empty() {
        Err(EmployeeError::DontUnderstand(String::from(MISSING_NOUN)))
    } else {
        Ok(Command::Add { person, department })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(
            Ok(Command::Add {
                person: String::from("Alice"),
                department: String::from("Administration")
            }),
            "add Alice to Administration".parse::<Command>()
        );
        assert_eq!(
            Ok(Command::Add {
                person: String::from("Sally"),
                department: String::from("Engineering")
            }),
            "Add Sally to Engineering".parse::<Command>()
        );
        assert_eq!(
            Ok(Command::Add {
                person: String::from("Amir"),
                department: String::from("Sales")
            }),
            "Add Amir to Sales.".parse::<Command>()
        );
        assert_eq!(
            Err(EmployeeError::DontUnderstand(String::from(MISSING_TO))),
            "add".parse::<Command>()
        );
        assert_eq!(
            Err(EmployeeError::DontUnderstand(String::from(MISSING_TO))),
            "add Alice".parse::<Command>()
        );
        assert_eq!(
            Err(EmployeeError::DontUnderstand(String::from(MISSING_NOUN))),
            "add to".parse::<Command>()
        );
    }
}
