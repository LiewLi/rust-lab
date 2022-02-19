use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf};
use chrono::{DateTime, Utc, serde::ts_seconds};
use std::io::{Seek, SeekFrom};
use std::fs::OpenOptions;
use anyhow::{Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct  Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Self {
        Self {
            text,
            created_at: Utc::now(),
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<50} [{}]", self.text, self.created_at)
    }
}


pub fn list_tasks(json_file_path: PathBuf) -> Result<()> {
    let mut file = OpenOptions::new().read(true).open(json_file_path)?;
    let tasks = parse_tasks(&mut file)?;
    if tasks.is_empty() {
        println!("All done, non todo left!!")
    } else {
        for (index, task) in tasks.into_iter().enumerate() {
            println!("{}: {}", index, task);
        }
    }

    Ok(())

}

pub fn add_task(task: Task, json_file_path: PathBuf) -> Result<()> {
    let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(json_file_path)?;
    
    let mut tasks = parse_tasks(&mut file)?;
    tasks.push(task);
    file.set_len(0)?;
    serde_json::to_writer(&file, &tasks)?;
    Ok(())

}

pub fn parse_tasks(file: &mut std::fs::File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks: Vec<Task> = match serde_json::from_reader(&*file) {
        Ok(tasks) => tasks,
        Err(err) if err.is_eof() => Vec::new(),
        Err(err) => Err(err)?
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}

pub fn done_task(json_file_path: PathBuf, index: usize) -> Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(json_file_path)?;
    let mut tasks = parse_tasks(&mut file)?;
    if index < tasks.len() {
        tasks.remove(index);
        file.set_len(0)?;
        serde_json::to_writer(&file, &tasks)?
    }
    Ok(())
}
