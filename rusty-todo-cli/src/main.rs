
mod cli;
mod task;

use structopt::StructOpt;
use cli::{Action::*, CommandLineOptions};
use task::*;

use anyhow::{anyhow, Result};

use std::path::PathBuf;

fn main() -> Result<()> {
    let CommandLineOptions { 
        action ,
        json_file_path 
    } = CommandLineOptions::from_args();

    let json_file_path = json_file_path
    .or_else(default_json_file_path)
    .ok_or(anyhow!("err file path"))?;

    println!("JSON file path: {:?}", json_file_path);

    match action {
        List => list_tasks(json_file_path)?,
        Add {text} => add_task(Task::new(text), json_file_path)?,
        Done {index} => done_task(json_file_path, index)?,
    }
    Ok(())
}

fn default_json_file_path() -> Option<PathBuf> {
    home::home_dir().map(|mut path|{
        path.push(".rusty-todo.json");
        path
    }) 
}
