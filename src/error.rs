// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved.
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum EmployeeError {
    IO(String),
    EmptyInput,
    DontUnderstand { command: String },
}

impl From<std::io::Error> for EmployeeError {
    fn from(err: std::io::Error) -> EmployeeError {
        EmployeeError::IO(err.to_string())
    }
}

impl fmt::Display for EmployeeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EmployeeError::IO(err) => write!(f, "{}", err),
            EmployeeError::EmptyInput => write!(f, "Command was empty."),
            EmployeeError::DontUnderstand { command } => {
                write!(f, "Command not understood: \"{}\"", command.trim_right())
            }
        }
    }
}
