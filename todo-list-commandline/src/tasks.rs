use chrono::{serde::ts_seconds, DateTime, Local};
use serde::Deserialize;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{Result, Seek, SeekFrom};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Local>
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Local> = Local::now();
        Task { text, created_at }
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // Open the file.
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // Consume the file's contents as a Vec of tasks.
    let mut tasks: Vec<Task> = match serde_json::from_reader(&file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    // Rewind the file after reading from it.
    file.seek(SeekFrom::Start(0))?;

    // Write the modified task list back into the file.
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {

}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {

}