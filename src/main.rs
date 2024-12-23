mod todo;

use std::env;
use std::process;
use todo::run;
use todo::{Status, ToDo};

fn main() {
    run();
    // let task = ToDo::new(env::args()).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    // println!("Task added : {:#?}", task);
}
