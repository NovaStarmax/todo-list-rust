use crate::ToDo;
use crate::Status;
use serde::{Deserialize, Serialize};
use std::fs;
// use crate::todo::new

pub fn load_from_file(path: &str) -> Vec<ToDo> {
    let data = fs::read_to_string(path).expect("Unable to read file");
    serde_json::from_str(&data).expect("Failed to deserialize tasks")
}

pub fn get_todo(tasks: &Vec<ToDo>) {
    for task in tasks.iter() {
        
        println!("{:?}", task);
    }
}
#[cfg(test)]
mod tests {
    use super::*; // Importe les fonctions et structures du fichier.
    // #[test]
    // fn test_todo() {
    //     let antoine = ToDo{
    //         title: String::from("Antoine"),
    //         information: String::from("Gobbe"),
    //         status: Status::Done,
    //     };
    //     let antoine_bis = ToDo{
    //         title: String::from("Antoine"),
    //         information: String::from("Gobbe"),
    //         status: Status::Done,
    //     };
        
    //     assert_eq!(antoine, antoine_bis);
    // }

    // #[test]
    // fn is_err() {
    //     let err = 1;
    //     assert_eq!(err, 2);
    // }
    #[test]
    fn is_valid() {
        let path = "task_folder/tasks.json";
        let tasks = load_from_file(path);
        println!("{:?}", tasks);
        assert_eq!(tasks.len(), 4);
        assert_eq!(tasks[0].title, "Acheter du pain");
        // assert_eq!(tasks[0].status, Status::ToDo);
        // assert_eq!(tasks[1].title, "Task 2");
        // assert_eq!(tasks[1].status, Status::InProgress);
    }
}