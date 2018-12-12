// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved.

use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Department {
    pub name: String,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Person {
    pub name: String,
}

pub struct Store {
    departments: HashMap<Department, Vec<Person>>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            departments: HashMap::new(),
        }
    }

    pub fn add_assignment(&mut self, person: Person, department: &Department) {
        let team = self
            .departments
            .entry(department.clone())
            .or_insert_with(|| vec![]);
        if !(*team).contains(&person) {
            (*team).push(person);
        }
    }

    pub fn list_department(&self, department: &Department) -> Vec<&Person> {
        match self.departments.get(&department) {
            None => Vec::new(),
            Some(team) => team.iter().collect::<Vec<&Person>>(),
        }
    }

    pub fn list(&self) -> Vec<(&Department, Vec<&Person>)> {
        let mut all = Vec::new();
        for (department, team) in self.departments.iter() {
            all.push((department, team.iter().collect::<Vec<&Person>>()));
        }
        all
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage() {
        let mut store = Store::new();
        {
            let alice = Person {
                name: "Alice".to_string(),
            };
            let engineering = Department {
                name: "Engineering".to_string(),
            };
            store.add_assignment(alice.clone(), &engineering);
            assert!(store.departments.len() == 1);
            assert!(store.departments.iter().any(|(_d, t)| t.contains(&alice)));
        }
        {
            let sally = Person {
                name: "Sally".to_string(),
            };
            let engineering = Department {
                name: "Engineering".to_string(),
            };
            let amir = Person {
                name: "Amir".to_string(),
            };
            let sales = Department {
                name: "Sales".to_string(),
            };
            store.add_assignment(sally, &engineering);
            store.add_assignment(amir, &sales);
            assert!(store.list_department(&engineering).len() == 2);
            assert!(store.list_department(&sales).len() == 1);
            let empty = store.list_department(&Department {
                name: String::new(),
            });
            assert!(empty.len() == 0);
        }
        {
            let all = store.list();
            assert!(all.len() == 2);
            assert!(all.get(0).expect("first").0.name == "Engineering");
            assert!(all.get(0).expect("first").1.len() == 2);
        }
    }
}
