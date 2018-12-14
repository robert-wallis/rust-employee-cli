// Copyright (C) 2018 Robert A. Wallis, All Rights Reserved.

use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Department {
    pub name: String,
}

impl PartialEq for Department {
    fn eq(&self, other: &Department) -> bool {
        self.name.to_lowercase() == other.name.to_lowercase()
    }
}
impl Eq for Department {}
impl Hash for Department {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let lower = self.name.to_lowercase();
        lower.hash(state);
    }
}