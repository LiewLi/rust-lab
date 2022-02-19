use std::{path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    List,
    Add {
        #[structopt()]
        text: String
    },
    Done {
        #[structopt()]
        index: usize
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "rusty-todo", about = "todo cli")]
pub struct CommandLineOptions {
    #[structopt(subcommand)]
    pub action: Action,
    #[structopt(parse(from_os_str), short = "p", long = "path")]
    pub json_file_path: Option<PathBuf>,
}

