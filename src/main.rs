mod errors;
mod file_manager;
mod pest;
use clap::Parser;

use crate::{file_manager::insert_erorrs, pest::run_tests};

#[derive(Parser)]
struct Cli {
    repeat: u64,
    path: String,
}

fn main() {
    let args = Cli::parse();
    let command = String::from(r#"./vendor/bin/pest"#);
    let file = String::from("output.json");

    let pb = indicatif::ProgressBar::new(args.repeat);

    for _i in 0..args.repeat {
        println!("Run tests...");

        let error_to_insert = run_tests(&file, &args.path, &command);

        match error_to_insert {
            Ok(values) => {
                insert_erorrs(values, &file);
            }
            Err(error) => println!("{}", error.to_message()),
        }

        pb.inc(1);
    }

    pb.finish_with_message("done");
}
