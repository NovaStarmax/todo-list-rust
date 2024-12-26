use crate::Status;
use std::io;

pub fn string_input() -> String {
    let mut new_string = String::new();
    io::stdin()
        .read_line(&mut new_string)
        .expect("Invalid value");
    new_string.trim().to_string()
}

pub fn int_input() -> i32 {
    loop {
        let mut new_int = String::new();
        io::stdin().read_line(&mut new_int).expect("Invalid number");
        let new_int: i32 = match new_int.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return new_int;
    }
}

pub fn status_input() -> Status {
    let input = int_input();
    match input {
        1 => Status::ToDo,
        2 => Status::InProgress,
        3 => Status::Done,
        _ => {
            println!("Invalid status, defaulting to 'To Do'");
            Status::ToDo
        }
    }
}
