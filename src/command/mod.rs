use super::error::EmployeeError;
use super::tokenizer;
use std::str::FromStr;
mod add;
mod list;
mod quit;

#[derive(Debug, PartialEq)]
pub enum Command {
    Add,
    List,
    Quit,
}

impl FromStr for Command {
    type Err = EmployeeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = tokenizer::tokenize(s);
        if tokens.is_empty() {
            return Err(EmployeeError::EmptyInput)
        }
        let first = tokens[0].to_lowercase();
        if add::looks_like(&first) {
            Ok(add::command())
        } else if list::looks_like(&first) {
            Ok(list::command())
        } else if quit::looks_like(&first) {
            Ok(quit::command())
        } else {
            Err(EmployeeError::DontUnderstand {
                command: String::from(s),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add() {
        assert_eq!(Ok(Command::Add), "add".parse::<Command>());
        assert_eq!(Ok(Command::Add), "Add".parse::<Command>());
        assert_eq!(Ok(Command::Add), "add Alice".parse::<Command>());
        assert_eq!(
            Ok(Command::Add),
            "add Alice to Administration".parse::<Command>()
        );
    }

    #[test]
    fn list() {
        assert_eq!(Ok(Command::List), "list".parse::<Command>());
        assert_eq!(Ok(Command::List), "List".parse::<Command>());
        assert_eq!(Ok(Command::List), "show".parse::<Command>());
        assert_eq!(Ok(Command::List), "display".parse::<Command>());
    }

    #[test]
    fn quit() {
        assert_eq!(Ok(Command::Quit), "quit".parse::<Command>());
        assert_eq!(Ok(Command::Quit), "Quit".parse::<Command>());
        assert_eq!(Ok(Command::Quit), "q".parse::<Command>());
        assert_eq!(Ok(Command::Quit), "exit".parse::<Command>());
    }
}
