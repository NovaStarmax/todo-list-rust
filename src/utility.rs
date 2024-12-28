use crate::task_manager::TaskManager;
use std::fs;

pub fn get_json(path: &str, task_manager: &mut TaskManager) {
    if fs::metadata(path).is_ok() {
        println!("JSON file exists. Loading tasks...");
        task_manager.load_from_file(path);
    } else {
        create_json();
    }
}

pub fn create_json() {
    fs::File::create("tasks.json").expect("JSON File format is invalid");
    println!("JSON file is created to save your tasks");
}

pub fn empty_json(path: &str) {
    fs::write(path, "").expect("Unable to clear the file");
}
