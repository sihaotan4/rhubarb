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
    println!();

    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        command = command.trim().to_string();

        if command == "exit" {
            break;
        }

        let result = database.resolve_command(command.as_str());
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
