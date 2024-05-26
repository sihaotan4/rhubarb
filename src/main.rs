use rhubarb::parse_set;
use rhubarb::database_pipeline::new_database_from_files;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let database = new_database_from_files(
        Path::new("database_config.toml"),
        Path::new("mock_data/assets.csv"),
        Path::new("mock_data/employees.csv"),
    )
    .unwrap();

    database.status_report();

    //dbg!(registry.data.keys());

    loop {
        print!("Enter query: ");
        io::stdout().flush().unwrap();

        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap();

        query = query.trim().to_string();

        if query == "exit" {
            break;
        }

        let result = parse_set::parse(&query, &database.asset_registry.data);
        match result {
            Ok(set) => {
                println!("{:?}", set);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
        println!();
    }
}
