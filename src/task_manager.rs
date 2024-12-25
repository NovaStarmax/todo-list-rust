use crate::Status;
use crate::ToDo;
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

    pub fn insert(&mut self, id: i32, task: ToDo) {
        self.task.insert(id, task);
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

        let mut task_id_input = String::new();
        io::stdin()
            .read_line(&mut task_id_input)
            .expect("Failed to read input");

        let task_id = match task_id_input.trim().parse::<i32>() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid ID. Please enter a valid number.");
                return;
            }
        };

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

        let mut task_id_input = String::new();
        io::stdin()
            .read_line(&mut task_id_input)
            .expect("Failed to read input");

        let task_id = match task_id_input.trim().parse::<i32>() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid ID. Please enter a valid number.");
                return;
            }
        };

        println!(
            "
        1 : Modify title
        2 : Modify information
        3 : Modify status
        "
        );

        let mut field_input = String::new();
        io::stdin()
            .read_line(&mut field_input)
            .expect("Invalid number");

        let mut field = match field_input.trim().parse::<i32>() {
            Ok(int) => int,
            Err(_) => {
                println!("Invalid field. Please enter a valid number.");
                return;
            }
        };

        let modify = match field {
            1 => {
                if let Some(x) = self.task.get_mut(&task_id) {
                    x.title = Self::string_input();
                }
            }
            2 => {
                if let Some(x) = self.task.get_mut(&task_id) {
                    x.information = Self::string_input();
                }
            }
            3 => {
                if let Some(x) = self.task.get_mut(&task_id) {
                    x.status = Self::get_status();
                }
            }
            _ => {}
        };
    }

    fn get_status() -> Status {
        let mut new_status = String::new();
        println!(
            "Select the Status:
            1: To Do
            2: In Progress
            3: Done"
        );

        io::stdin()
            .read_line(&mut new_status)
            .expect("Invalid value");

        match new_status.trim().parse::<usize>() {
            Ok(1) => Status::ToDo,
            Ok(2) => Status::InProgress,
            Ok(3) => Status::Done,
            _ => {
                println!("Invalid status, defaulting to 'To Do'");
                Status::ToDo
            }
        }
    }

    fn string_input() -> String {
        println!("Choose your new text");

        let mut new_string = String::new();
        io::stdin()
            .read_line(&mut new_string)
            .expect("Invalid value");
        new_string.trim().to_string()
    }

    fn int_intput() -> i32 {
        loop {
            println!("Choose your new number");

            let mut new_int = String::new();
            io::stdin().read_line(&mut new_int).expect("Invalid number");
            let new_int: i32 = match new_int.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            return new_int;
        }
    }
}
