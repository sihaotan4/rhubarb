use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};

// need to be able to deserialize this struct (at least the first 3 fields) into disc
#[derive(Debug, Clone)]
pub struct Database {
    // derived from database metadata or information_schema
    pub asset_registry: SetRegistry,
    // derived from employee databases or other employee registries
    pub user_registry: SetRegistry,
    // mutable map of permissions
    pub permission_log: HashMap<String, Permission>,
    // Access matrix WIP
    //pub access_matrix:
}

#[derive(Debug, Clone)]
pub struct Permission {
    // WIP
}

#[derive(Debug, Clone)]
pub struct SetRegistry {
    // this data representation is a bit like an inverted index
    // role maps to the set of IDs that hold that role
    pub data: HashMap<String, HashSet<String>>,
    // for convenience we also store the list of ids
    pub ids: HashSet<String>,
    pub etl_date: DateTime<Utc>,
}

impl SetRegistry {
    pub fn new() -> SetRegistry {
        SetRegistry {
            data: HashMap::new(),
            ids: HashSet::new(),
            etl_date: Utc::now(),
        }
    }

    pub fn insert(&mut self, k: String, v: String) {
        self.data
            .entry(k.clone())
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
