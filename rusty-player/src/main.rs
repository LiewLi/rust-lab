mod cli;
mod play;
mod playlist;

use structopt::StructOpt;

use anyhow::Result;
use cli::{Command, Opt};

fn main() -> Result<()> {
    let opt = Opt::from_args();
    match opt.cmd {
        Command::Play { url } => play::play_file(&url),
    }
}
