// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved.
///
/// Employee Database CLI
///
/// > Using a hash map and vectors,
/// > create a text interface to allow a user to
/// > add employee names to a department in a company.
/// > For example, “Add Sally to Engineering”
/// > or “Add Amir to Sales.”
/// > Then let the user retrieve a list of
/// > all people in a department
/// > or all people in the company by department,
/// > sorted alphabetically.
/// > https://doc.rust-lang.org/book/ch08-03-hash-maps.html
use std::io;

mod command;
use self::command::Command;
mod error;
use self::error::EmployeeError;
mod tokenizer;

fn main() -> Result<(), EmployeeError> {
    loop {
        let line = read_line()?;
        match line.parse::<command::Command>() {
            Ok(cmd) => match cmd {
                Command::Add { person, department } => println!("Adding {} to {}", person, department),
                Command::List => println!("Listing"),
                Command::Quit => break,
            },
            Err(err) => match err {
                EmployeeError::EmptyInput => break,
                _ => eprintln!("{}", err),
            },
        }
    }
    Ok(())
}

fn read_line() -> io::Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line)
}
