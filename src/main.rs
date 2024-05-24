use rhubarb::parser::execute;
use rhubarb::{data_pipeline, set_registry::SetRegistry};
use std::path::Path;
use std::io::{self, Write};

fn main() {
    println!("Loading set registry...");
    
    let registry: SetRegistry = 
        data_pipeline::load_role_registry_from_csv(Path::new("mock_data.csv")).unwrap();

    dbg!(registry.data.keys());

    loop {
        print!("Enter query: ");
        io::stdout().flush().unwrap();

        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap();

        query = query.trim().to_string();

        if query== "exit" {
            break;
        }

        let result = execute(&query, &registry.data);
        match result {
            Ok(set) => {println!("{:?}", set);}
            Err(err) => {println!("{}", err);}
        }
        println!();
    }
}