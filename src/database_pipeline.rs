use crate::config::Config;
use crate::database::{Database, SetRegistry};
use std::fs::File;
use std::{collections::HashMap, path::Path};

pub fn new_database_from_files(
    config_filepath: &Path,
    asset_csv_filepath: &Path,
    users_csv_filepath: &Path,
) -> anyhow::Result<Database> { 
    let config_toml = std::fs::read_to_string(config_filepath)?;
    let config: Config = toml::from_str(&config_toml)?;

    let db = Database {
        asset_registry: load_set_registry_from_csv(asset_csv_filepath)?,
        user_registry: load_set_registry_from_csv(users_csv_filepath)?,
        valid_permissions: config.database_config.valid_permissions,
        statement_log: HashMap::new(),
    };

    anyhow::Ok(db)
}

fn load_set_registry_from_csv(csv_filepath: &Path) -> anyhow::Result<SetRegistry> {
    let file = File::open(csv_filepath)?;
    let mut csv_rdr = csv::ReaderBuilder::new().delimiter(b',').from_reader(file);

    let headers = csv_rdr.headers()?;
    let transformed_headers: Vec<String> = headers
        .iter()
        .map(|header| {
            header
                .trim()
                .to_lowercase()
                .replace(|c: char| c.is_whitespace() || c == '(' || c == ')', "_")
        })
        .collect();

    if transformed_headers.get(0) != Some(&"id".to_string()) {
        return Err(anyhow::anyhow!("First column is not 'id'"));
    }

    // process rows by processing each entry
    let mut registry = SetRegistry::new();

    while let Some(result) = csv_rdr.records().next() {
        // extract the row id
        let record = result?;
        let id = record.get(0).unwrap().to_string();

        registry.ids.insert(id.clone());

        // iterate through each entry in the row
        // additionally zip this with the headers for key creation
        record
            .into_iter()
            .zip(transformed_headers.clone())
            .for_each(|(entry, transformed_header)| {
                let transformed_entry = entry
                    .trim()
                    .to_lowercase()
                    .replace(|c: char| c.is_whitespace() || c == '(' || c == ')', "_");

                let key = format!("{transformed_header}:{transformed_entry}");

                registry.insert(key, id.clone());
            })
    }

    anyhow::Ok(registry)
}
