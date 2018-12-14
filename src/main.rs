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
use std::io::{self, Write};

mod command;
use self::command::{Command, ListType};
mod error;
use self::error::EmployeeError;
mod store;
mod tokenizer;
use self::store::{Store, department::Department, person::Person};

fn main() -> Result<(), EmployeeError> {
    let mut store = Store::new();
    loop {
        let line = read_line()?;
        match line.parse::<command::Command>() {
            Ok(cmd) => match cmd {
                Command::Add { person, department } => {
                    let person = Person { name: person };
                    let department = Department { name: department };
                    store.add_assignment(person, &department);
                }
                Command::List(list_type) => match list_type {
                    ListType::Noun(noun) => {
                        let department = Department { name: noun };
                        print_department(&department, &store.list_department(&department));
                    }
                    ListType::Everything => {
                        for (department, team) in store.list() {
                            print_department(department, &team);
                        }
                    }
                },
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
    io::stdout().write_all(b"> ")?;
    io::stdout().flush()?;
    io::stdin().read_line(&mut line)?;
    Ok(line)
}

fn print_department(department: &Department, people: &[&Person]) {
    for person in people {
        println!("{} in {}", person.name, department.name);
    }
}
