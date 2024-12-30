use crate::input::{create_todo, int_input, status_input, string_input};
use crate::utility::empty_json;
use crate::ToDo;
use std::collections::BTreeMap;
use std::fs;

pub struct TaskManager {
    pub id: i32,
    pub task: BTreeMap<i32, ToDo>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            id: 1,
            task: BTreeMap::new(),
        }
    }

    pub fn increase_id(&mut self) {
        self.id += 1;
    }

    pub fn add_task(&mut self) {
        let task = create_todo();
        self.task.insert(self.id, task);
        self.increase_id();
    }

    pub fn list_task(&self) {
        if self.task.is_empty() {
            println!("No tasks available.");
        } else {
            for (id, task) in &self.task {
                println!("ID: {}, Task: {:?}", id, task);
            }
        }
    }
    pub fn delete_task(&mut self) {
        if self.task.is_empty() {
            println!("No tasks available to delete.");
            return;
        }

        println!("Get ID of your task to delete:");
        for (id, task) in &self.task {
            println!("ID: {}, Task: {}", id, task.title);
        }
        let task_id = int_input();

        if self.task.remove(&task_id).is_some() {
            println!("Task with ID {} has been deleted.", task_id);
        } else {
            println!("Task with ID {} not found.", task_id);
        }
    }

    pub fn modify_task(&mut self) {
        if self.task.is_empty() {
            println!("No tasks available to modify.");
            return;
        }

        println!("Get ID of your task to modify :");
        for (id, task) in &self.task {
            println!(
                "ID: {}, Task: {} : {}, Status : {:?}",
                id, task.title, task.information, task.status
            );
        }
        let task_id = int_input();

        println!(
            "
        1 : Modify title
        2 : Modify information
        3 : Modify status
        "
        );
        let field = int_input();
        match field {
            1 => {
                if let Some(x) = self.task.get_mut(&task_id) {
                    println!("New title");
                    x.title = string_input();
                }
            }
            2 => {
                if let Some(x) = self.task.get_mut(&task_id) {
                    println!("New information");
                    x.information = string_input();
                }
            }
            3 => {
                if let Some(x) = self.task.get_mut(&task_id) {
                    println!(
                        "Select the Status:
                        1: To Do
                        2: In Progress
                        3: Done"
                    );
                    x.status = status_input();
                }
            }
            _ => {}
        };
    }

    pub fn load_from_file(&mut self, path: &str) {
        let data = fs::read_to_string(path);
        match data {
            Ok(content) => match serde_json::from_str::<Vec<ToDo>>(&content) {
                Ok(tasks) => {
                    for task in tasks {
                        self.add_task_from_json(&task);
                    }
                    println!("Tasks successfully loaded from {}", path);
                }
                Err(err) => {
                    println!("Invalid JSON format : {}. Clearing file …", err);
                    empty_json(path);
                }
            },
            Err(_) => {
                println!("File not found or unreadable: {}", path);
            }
        }
    }

    pub fn add_task_from_json(&mut self, task: &ToDo) {
        self.task.insert(self.id, task.clone());
        self.increase_id();
    }

    pub fn save_to_file(&mut self, path: &str) {
        empty_json(path);
        let vec_todo: Vec<ToDo> = self.task.values().cloned().collect();
        let json = serde_json::to_string(&vec_todo).expect("Unable to write to file");
        fs::write(path, json).expect("Unable to write to file");
    }
}
