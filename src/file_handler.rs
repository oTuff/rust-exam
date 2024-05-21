use crate::task_manager::TaskManager;
use serde::de::Error;
use serde_json::{Error as JsonError, Result as JsonResult};
use std::fs::File;
use std::io::{Read, Write};

pub fn load_from_file(file_path: &str) -> JsonResult<TaskManager> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => return Err(JsonError::custom(format!("Failed to open file: {}", e))),
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(JsonError::custom(format!("Failed to read file: {}", e)));
    }

    let manager = serde_json::from_str(&contents)
        .map_err(|e| JsonError::custom(format!("Failed to parse JSON: {}", e)))?;
    Ok(manager)
}

pub fn save_to_file(manager: &TaskManager, file_path: &str) -> JsonResult<()> {
    let data = serde_json::to_string(manager)
        .map_err(|e| JsonError::custom(format!("Failed to serialize data: {}", e)))?;

    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => return Err(JsonError::custom(format!("Failed to create file: {}", e))),
    };

    if let Err(e) = file.write_all(data.as_bytes()) {
        return Err(JsonError::custom(format!("Failed to write to file: {}", e)));
    }

    Ok(())
}
