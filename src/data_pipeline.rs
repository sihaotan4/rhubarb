use std::path::Path;
use std::fs::File;
use crate::role_registry::RoleRegistry;

pub fn load_role_registry_from_csv(csv_file_path: &Path) -> anyhow::Result<RoleRegistry> {
    let file = File::open(csv_file_path)?;
    let mut csv_rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(file);

    let headers = csv_rdr.headers()?;
    let transformed_headers: Vec<String> = headers.iter()
        .map(|header| {
            header
                .trim()
                .to_lowercase()
                .replace(|c: char| !c.is_alphanumeric(), "_")
        })
        .collect();

    if transformed_headers.get(0) != Some(&"ID".to_string()) {
        return Err(anyhow::anyhow!("First column is not 'ID'"));
    }
    
    // process rows by processing each entry
    let mut registry = RoleRegistry::new();

    while let Some(result) = csv_rdr.records().next() {
        // extract the row id
        let record = result?;
        let id = record.get(0).unwrap().to_string();

        // iterate through each entry in the row
        // additionally zip this with the headers for key creation
        record
            .into_iter()
            .zip(transformed_headers.clone())
            .for_each(|(entry, transformed_header)| {
                let transformed_entry = entry
                    .trim()
                    .to_lowercase()
                    .replace(|c: char| c.is_whitespace(), "_");

                let key = format!("{transformed_header}:{transformed_entry}");

                registry.insert(key, id.clone());
            })
    }

    Ok(registry)
}