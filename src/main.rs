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
            "Opening ToDo List
            1 : Add task
            2 : List task
            3 : Delete task
            4 : Modify task
            5 : Quit"
        );
        let init = int_input();
        match init {
            1 => tasks.add_task(),
            2 => tasks.list_task(),
            3 => tasks.delete_task(),
            4 => tasks.modify_task(),
            // 6 => tasks.download(), // Écrire les taches dans le fichier json 
            // 7 => tasks.upload(), // Importer les taches du fichier json dans l’instance de tasks
            5 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}
