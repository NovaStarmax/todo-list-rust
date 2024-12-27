use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ToDo {
    pub title: String,
    pub information: String,
    pub status: Status,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl ToDo {
    pub fn new_task(title: String, information: String, status: Status) -> Self {
        Self {
            title: title,
            information: information,
            status: status,
        }
    }
}