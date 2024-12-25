use crate::input::{int_intput, status_input, string_input};
use crate::ToDo;
use std::collections::HashMap;

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

    pub fn add_task(&mut self) {
        self.next_id += 1;

        println!("What is your task?");
        let title = string_input();

        println!("Give specific information:");
        let information = string_input();

        println!(
            "Select the Status:
            1: To Do
            2: In Progress
            3: Done"
        );
        let status = status_input();

        self.task.insert(
            self.next_id,
            ToDo {
                title: title.trim().to_string(),
                information: information.trim().to_string(),
                status,
            },
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
        let task_id = int_intput();

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
        let task_id = int_intput();

        println!(
            "
        1 : Modify title
        2 : Modify information
        3 : Modify status
        "
        );
        let field = int_intput();
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
