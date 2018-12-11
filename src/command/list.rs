// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved.

use super::Command;
use super::ListType;

pub fn looks_like(first: &str) -> bool {
    first == "list" || first == "show" || first == "display"
}

pub fn command(tokens: &[&str]) -> Command {
    if tokens.len() < 3 {
        // space is a token, so we need at least 3
        Command::List(ListType::Everything)
    } else {
        let noun = tokens[2];
        if noun == "*" {
            Command::List(ListType::Everything)
        } else {
            Command::List(ListType::Noun(noun.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Command::List;
    use super::ListType::*;
    use super::*;

    #[test]
    fn list() {
        assert_eq!(Ok(List(Everything)), "list".parse::<Command>());
        assert_eq!(Ok(List(Everything)), "List".parse::<Command>());
        assert_eq!(Ok(List(Everything)), "show".parse::<Command>());
        assert_eq!(Ok(List(Everything)), "display".parse::<Command>());
    }

    #[test]
    fn noun() {
        assert_eq!(Ok(List(Everything)), "list".parse::<Command>());
        assert_eq!(
            Ok(List(Noun("engineering".to_string()))),
            "list engineering".parse::<Command>()
        );
        assert_eq!(
            Ok(List(Noun("Engineering".to_string()))),
            "list Engineering".parse::<Command>()
        );
        assert_eq!(
            Ok(List(Noun("alice".to_string()))),
            "list alice".parse::<Command>()
        );
        assert_eq!(Ok(List(Everything)), "list *".parse::<Command>());
    }
}
