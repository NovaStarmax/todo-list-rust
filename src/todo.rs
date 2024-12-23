#[derive(Debug)]
pub struct ToDo {
    pub title: String,
    pub information: String,
    pub status: Status,
}

#[derive(Debug)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
