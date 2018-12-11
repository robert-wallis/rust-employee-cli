// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved.

use super::Command;

pub fn looks_like(first: &str) -> bool {
    first == "quit" || first == "exit" || first == "q"
}

pub fn command() -> Command {
    Command::Quit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quit() {
        use super::Command::Quit;
        assert_eq!(Ok(Quit), "quit".parse::<Command>());
        assert_eq!(Ok(Quit), "Quit".parse::<Command>());
        assert_eq!(Ok(Quit), "q".parse::<Command>());
        assert_eq!(Ok(Quit), "exit".parse::<Command>());
    }
}
