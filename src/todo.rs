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
