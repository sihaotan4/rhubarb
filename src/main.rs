use rhubarb::{data_pipeline, role_registry::RoleRegistry};
use std::path::Path;

fn main() {
    println!("Loading role registry...");
    
    let registry: RoleRegistry = 
        data_pipeline::load_role_registry_from_csv(Path::new("mock_data.csv")).unwrap();

    dbg!(registry.data.keys());


}