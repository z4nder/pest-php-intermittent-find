use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

use regex::Regex;

fn main() {
    let mut file = File::create("output.txt").unwrap();
    let project_path = "/home/zander/projects/souk/rocinha";
    let command = r#"./vendor/bin/pest"#;
    let filter = r#"passa as validacoes e execute os side effects"#;

    loop {
        let output = Command::new(command)
            .current_dir(project_path)
            .arg("--compact")
            // .arg("--filter")
            // .arg(filter)
            .output();

        let command_response = match output {
            Ok(value) => value,
            Err(err) => panic!("Error at run command {}", err),
        };

        println!("Run tests...");

        let command_response = String::from_utf8(command_response.stdout)
            .unwrap()
            .replace("\n", "");

        let re: Regex = Regex::new(r"FAILED\s+(.*?)\.").unwrap();
        let extracted_text = re
            .captures(&command_response)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .trim();

        println!("Errors write...  {}", extracted_text);

        let line = format!("{} \n", extracted_text);
        file.write_all(line.as_bytes()).unwrap();

        println!("Finish run...");
    }
}
