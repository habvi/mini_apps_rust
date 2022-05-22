use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct Task {
    pub text: String,
    pub created_at: DateTime<Local>
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Local> = Local::now();
        Task { text, created_at }
    }
}