use std::{collections::HashMap, io};
#[derive(Debug)]
pub struct ToDo {
    pub id: i32,
    pub title: String,
    pub information: String,
    pub status: Status,
}

pub struct TaskManager {
    pub task: HashMap<i32, ToDo>,
    pub next_id: i32,
}

#[derive(Debug)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

pub enum Action {
    List,
    Add,
    Modify,
    Delete,
}

impl ToDo {
    pub fn add_task() -> Result<ToDo, &'static str> {
        let id = 1;

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

        Ok(ToDo {
            id,
            title: title.trim().to_string(),
            information: information.trim().to_string(),
            status,
        })
    }
}

impl TaskManager {
    pub fn new() -> Result<TaskManager, &'static str> {
        let hashmap: HashMap<i32, ToDo> = HashMap::new();
        let id = 1;
        Ok(
            
        )
    }
}

pub fn run() {
    loop {
        let mut init = String::new();
        println!(
            "Opening ToDo List
            1 : Add task
            2 : List task
            3 : Delete task
            4 : Modify task
            5 : Quit"
        );

        io::stdin()
            .read_line(&mut init)
            .expect("Error reading input");

        let init: usize = match init.trim().parse() {
            Ok(num) => num,
            Err(_) => 5,
        };

        match init {
            1 => match ToDo::add_task() {
                Ok(task) => println!("Task added: {:#?}", task),
                Err(err) => println!("Failed to add task: {}", err),
            },
            2 => println!("Listing tasks is not implemented yet."),
            3 => println!("Deleting tasks is not implemented yet."),
            4 => println!("Modifying tasks is not implemented yet."),
            5 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}
