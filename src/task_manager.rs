use crate::input::{int_input, status_input, string_input, create_todo};
use crate::ToDo;
use std::collections::HashMap;
use std::fs;

pub struct TaskManager {
    pub task: HashMap<i32, ToDo>,
    pub next_id: i32,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            task: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn insert(&mut self, id: i32, task: ToDo) {
        self.task.insert(id, task);
    }

    // Ce posser la question si on ajoute les id dans le json car c’est plus simple pour la suite 

    pub fn load_from_file(&mut self, path: &str){
        let data = fs::read_to_string(path).expect("Unable to read file");
        let data_vec: Vec<ToDo> = serde_json::from_str(&data).expect("Failed to deserialize tasks");
        for task in data_vec {
            self.task.insert(self.next_id, task);
            self.next_id += 1;
        }
    }

    pub fn add_task(&mut self) {
        self.next_id += 1;

        let task = create_todo();
        self.task.insert(
            self.next_id,
            task,
        );
        println!("Hash : {:?}", self.task);
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
}
