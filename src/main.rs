mod input;
mod task_manager;
mod todo;
mod json_manager;
use crate::input::int_input;
use task_manager::*;
use todo::{Status, ToDo};

fn main() {
    let mut tasks = TaskManager::new();
    tasks.insert(
        10,
        ToDo {
            title: "Ma tache".to_string(),
            information: "Mon information".to_string(),
            status: Status::ToDo,
        },
    );
    run(&mut tasks);
}

fn run(tasks: &mut TaskManager) {
    loop {
        println!(
            "Menu:
            1. Add task
            2. List tasks
            3. Delete task
            4. Modify task
            5. Save tasks to JSON
            6. Load tasks from JSON
            7. Quit"
        );
        let choice = int_input();
        match choice {
            1 => tasks.add_task(),
            2 => tasks.list_task(),
            3 => tasks.delete_task(),
            4 => tasks.modify_task(),
            // 5 => json_manager::save_to_file(&tasks.task, "task_folder/tasks.json"), // Implémenter save
            // 6 => tasks.load_from_file("task_folder/tasks.json"), // Implémenter load
            7 => break,
            _ => println!("Invalid option."),
        }
    }
}
