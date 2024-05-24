use std::collections::{HashMap, HashSet};

// this data representation is a bit like an inverted index - mapping role to the holders of that role
#[derive(Debug, Clone)]
pub struct SetRegistry {
    pub data: HashMap<String, HashSet<String>>
}

impl SetRegistry {
    pub fn new() -> SetRegistry {
        SetRegistry { data: HashMap::new() }
    }

    pub fn insert(&mut self, k: String, v: String) {
        self.data
            .entry(k)
            .or_insert(HashSet::new())
            .insert(v);
    }

    pub fn delete(&mut self, k: &String, v: &String) {
        if let Some(values) = self.data.get_mut(k) {
            values.remove(v);
            if values.is_empty() {
                self.data.remove(k);
            }
        }
    }
}