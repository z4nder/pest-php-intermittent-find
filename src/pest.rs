use std::{collections::HashMap, process::Command};

use regex::Regex;

use crate::{
    errors::AppErrors,
    file_manager::{get_file_content, read_json},
};

pub fn run_tests(
    output_file_name: &String,
    project_path: &String,
    command: &String,
) -> Result<HashMap<String, String>, AppErrors> {
    let mut output_file = read_json(&output_file_name).unwrap();
    let output_file_content = get_file_content(&mut output_file);

    let output = Command::new(command)
        .current_dir(project_path)
        .arg("--compact")
        .output();

    let output = match output {
        Ok(value) => String::from_utf8(value.stdout).unwrap(),
        Err(err) => panic!("Error at run command {}", err),
    };

    let output = match validate_output(output) {
        Ok(value) => value,
        Err(err) => panic!("{}", err.to_message()),
    };

    let errors_vec: Vec<String> = convert_command_output_top_vec_errors(output);
    let error_map = convert_errors_vec_to_errors_hashmap(errors_vec);

    get_errors_to_insert(error_map, output_file_content)
}

fn validate_output(output: String) -> Result<String, AppErrors> {
    if output.contains("ERROR") {
        return Err(AppErrors::ErrorAtYourTestCase);
    }

    Ok(output)
}

fn convert_command_output_top_vec_errors(output: String) -> Vec<String> {
    let re = Regex::new(r"Failed.*\n*\n*.*php:\d+").unwrap();

    re.captures_iter(&output)
        .map(|captures| captures[0].to_string())
        .map(|line| line.to_owned().replace("\n", "").replace("  ", " "))
        .collect()
}

fn convert_errors_vec_to_errors_hashmap(errors_vec: Vec<String>) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    for s in errors_vec {
        if let Some(index) = s.find(". at") {
            if let Ok(error_file) = s[index + 1..].parse::<String>() {
                let error_text = s[..index].to_string();
                map.insert(error_file.replace(" at ", ""), error_text);
            }
        }
    }

    map
}

fn get_errors_to_insert(
    error_map: HashMap<String, String>,
    output_file_content: HashMap<String, String>,
) -> Result<HashMap<String, String>, AppErrors> {
    let mut result: HashMap<String, String> = HashMap::new();

    for (key, value) in error_map.iter() {
        if !result.contains_key(key) {
            result.insert(key.clone(), value.clone());
        }
    }

    for (key, value) in output_file_content.iter() {
        if !result.contains_key(key) {
            result.insert(key.clone(), value.clone());
        }
    }

    if result.is_empty() {
        return Err(AppErrors::NotHasErrors);
    }

    Ok(result)
}
