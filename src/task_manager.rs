use crate::ToDo;
use crate::Status;
use std::collections::HashMap;
use std::io;

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

    pub fn add_task(&mut self) {
        self.next_id += 1;

        println!("What is your task?");
        let mut title = String::new();
        io::stdin()
            .read_line(&mut title)
            .expect("Error reading title");

        println!("Give specific information:");
        let mut information = String::new();
        io::stdin()
            .read_line(&mut information)
            .expect("Error reading information");

        println!(
            "Select the Status:
            1: To Do
            2: In Progress
            3: Done"
        );

        let mut status_input = String::new();
        io::stdin()
            .read_line(&mut status_input)
            .expect("Error reading status");

        let status = match status_input.trim().parse::<usize>() {
            Ok(1) => Status::ToDo,
            Ok(2) => Status::InProgress,
            Ok(3) => Status::Done,
            _ => {
                println!("Invalid status, defaulting to 'To Do'");
                Status::ToDo
            }
        };

        self.task.insert(self.next_id, ToDo {
            title: title.trim().to_string(),
            information: information.trim().to_string(),
            status,
        });
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
}