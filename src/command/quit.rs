// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved.

use super::Command;

pub fn looks_like(first: &str) -> bool {
    first == "quit" || first == "exit" || first == "q"
}

pub fn command() -> Command {
    Command::Quit
}
