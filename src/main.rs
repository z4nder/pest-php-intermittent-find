mod errors;
mod file_manager;
mod pest;

use crate::{file_manager::insert_erorrs, pest::run_tests};

fn main() {
    let path = String::from("/home/zander/projects/personal/example-app");
    let command = String::from(r#"./vendor/bin/pest"#);
    let file = String::from("output.json");
    let repeat = 1;

    for _i in 0..repeat {
        println!("Run tests...");

        let error_to_insert = run_tests(&file, &path, &command);

        match error_to_insert {
            Ok(values) => {
                insert_erorrs(values, &file);
                println!("Finish, errors inserted...")
            }
            Err(error) => println!("{}", error.to_message()),
        }
    }
}
