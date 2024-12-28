use crate::task_manager::TaskManager;
use std::fs;
use std::fs::File;

pub fn get_json(path: &str, task_manager: &mut TaskManager) {
    if fs::metadata(path).is_ok() {
        println!("JSON file exists. Loading tasks...");
        task_manager.load_from_file(path);
    } else {
        let _ = File::create("tasks.json");
        println!("JSON file is created to save your tasks");
    }
}
