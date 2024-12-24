mod todo;
mod task_manager;
use task_manager::*;
// use todo::run;
// use todo::TaskManager;
use todo::{Status, ToDo};
use std::io;

fn main() {
    let mut tasks = TaskManager::new();
    run(&mut tasks);
}

fn run(tasks: &mut TaskManager){
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
            1 => tasks.add_task(),
            2 => tasks.list_task(),
            3 => tasks.delete_task(),
            4 => println!("Modifying tasks is not implemented yet."),
            5 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}