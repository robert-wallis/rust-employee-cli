use super::error::EmployeeError;
use super::tokenizer;
use std::str::FromStr;
mod add;
mod list;
mod quit;

#[derive(Debug, PartialEq)]
pub enum Command {
    Add { person: String, department: String },
    List, //(ListType),
    Quit,
}

// #[derive(Debug, PartialEq)]
// pub enum ListType {
//     Person(String),
//     Department(String),
// }

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
            Ok(list::command())
        } else if quit::looks_like(&first) {
            Ok(quit::command())
        } else {
            Err(EmployeeError::DontUnderstand(format!("Try add, list or quit.")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(
            Err(EmployeeError::DontUnderstand(String::from("Missing \"to\" seperator. (ex. Add Alice to Engineering)."))),
            "add".parse::<Command>()
        );
        assert_eq!(
            Err(EmployeeError::DontUnderstand(String::from("Missing \"to\" seperator. (ex. Add Alice to Engineering)."))),
            "add Alice".parse::<Command>()
        );
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
