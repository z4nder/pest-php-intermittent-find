use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Read;

use crate::errors::AppErrors;

pub fn read_json(file_name: &String) -> Result<File, AppErrors> {
    match File::open(&file_name) {
        Ok(file) => file,
        Err(_) => OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_name)
            .map_err(|_| AppErrors::ErrorToReadOutputFile)?,
    };

    File::open(file_name).map_err(|_| AppErrors::ErrorToReadOutputFile)
}

pub fn get_file_content(file: &mut File) -> HashMap<String, String> {
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let json_content: Result<HashMap<String, String>, AppErrors> =
        serde_json::from_str(&contents).map_err(|_| AppErrors::ErrorToReadOutputFile);

    match json_content {
        Ok(value) => value,
        Err(_) => HashMap::new(),
    }
}

pub fn insert_erorrs(errors_to_insert: HashMap<String, String>, file_path: &String) {
    let json = serde_json::to_string(&errors_to_insert).unwrap();
    std::fs::write(file_path, json).unwrap();
}
