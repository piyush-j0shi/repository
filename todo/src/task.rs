#[derive(Debug)]
pub enum Status {
    Pending,
    Done,
}

#[derive(Debug)]
pub enum Priority {
    Low,
    High,
    Medium,
}

#[derive(Debug)]
pub struct Task {
    pub description: String,
    pub status: Status,
    pub priority: Priority,
}
