// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved.

use std::collections::HashMap;
pub mod department;
pub mod person;
use self::department::Department;
use self::person::Person;

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
            (*team).sort_by(|a, b| a.name.cmp(&b.name));
        }
    }

    pub fn list_department(&self, department: &Department) -> Vec<&Person> {
        match self.departments.get(&department) {
            None => Vec::new(),
            Some(team) => team_vec(&team),
        }
    }

    pub fn list(&self) -> Vec<(&Department, Vec<&Person>)> {
        let mut all = Vec::new();
        for (department, team) in self.departments.iter() {
            all.push((department, team_vec(&team)));
        }
        all.sort_by(|a, b| a.0.name.cmp(&b.0.name));
        all
    }
}

fn team_vec<'a>(team: &'a [Person]) -> Vec<&'a Person> {
    team.iter().collect::<Vec<&'a Person>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage() {
        let mut store = Store::new();
        {
            // GIVEN an empty store
            // WHEN Alice is added to Engineering
            let alice = Person {
                name: "Alice".to_string(),
            };
            let engineering = Department {
                name: "Engineering".to_string(),
            };
            store.add_assignment(alice.clone(), &engineering);
            // THEN the number of departments should be 1, and it should contain Alice
            assert_eq!(store.departments.len(), 1);
            assert!(store.departments.iter().any(|(_d, t)| t.contains(&alice)));
        }
        {
            // GIVEN Alice was already added to Engineering
            let alice = Person {
                name: "Alice".to_string(),
            };
            let lower_case = Department {
                name: "engineering".to_string(),
            };
            // WHEN Alice is added to lower case "engineering"
            store.add_assignment(alice.clone(), &lower_case);
            // THEN Alice should still be in the original department, and no new department added
            assert_eq!(store.departments.len(), 1);
            assert!(store.departments.iter().next().unwrap().0.name == "Engineering");
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
