use super::error::EmployeeError;
use super::tokenizer;
use std::str::FromStr;
mod add;
mod list;
mod quit;

const USAGE: &str = "Try add, list, or quit.";

#[derive(Debug, PartialEq)]
pub enum Command {
    Add { person: String, department: String },
    List(ListType),
    Quit,
}

#[derive(Debug, PartialEq)]
pub enum ListType {
    Noun(String),
    Everything,
}

impl FromStr for Command {
    type Err = EmployeeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = tokenizer::tokenize(s);
        if tokens.is_empty() {
            return Err(EmployeeError::EmptyInput);
        }
        let first = tokens[0].to_lowercase();
        if add::looks_like(&first) {
            add::command(&tokens)
        } else if list::looks_like(&first) {
            Ok(list::command(&tokens))
        } else if quit::looks_like(&first) {
            Ok(quit::command())
        } else {
            Err(EmployeeError::DontUnderstand(USAGE.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown() {
        assert_eq!(Err(EmployeeError::EmptyInput), "".parse::<Command>());
        assert_eq!(
            Err(EmployeeError::DontUnderstand(USAGE.to_string())),
            " ".parse::<Command>()
        );
        assert_eq!(
            Err(EmployeeError::DontUnderstand(USAGE.to_string())),
            "beep".parse::<Command>()
        );
    }
}
