mod input;
mod task_manager;
mod todo;
mod utility;
use crate::input::int_input;
use task_manager::*;
use todo::{Status, ToDo};
use utility::get_json;

fn main() {
    let mut tasks = TaskManager::new();
    run(&mut tasks);
}

fn run(tasks: &mut TaskManager) {
    let path = "tasks.json";
    get_json(path, tasks);
    loop {
        println!(
            "Menu:
            1. Add task
            2. List tasks
            3. Delete task
            4. Modify task
            5. Quit and Save"
        );
        let action = int_input();
        match action {
            1 => tasks.add_task(),
            2 => tasks.list_task(),
            3 => tasks.delete_task(),
            4 => tasks.modify_task(),
            5 => {
                tasks.save_to_file(&path);
                break;
            }
            _ => println!("Invalid option."),
        }
    }
}
