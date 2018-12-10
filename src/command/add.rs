// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved.

use super::Command;

pub fn looks_like(first: &str) -> bool {
    first == "add"
}

pub fn command() -> Command {
    Command::Add
}