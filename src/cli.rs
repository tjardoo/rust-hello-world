use std::path::PathBuf;
use structopt::StructOpt;

/// cargo run -- -j test-journal.json list
/// cargo run -- -j test-journal.json add "water the plants"
/// cargo run -- -j test-journal.json done 2
#[derive(Debug, StructOpt)]
pub enum Action {
    Add {
        #[structopt()]
        text: String,
    },
    Done {
        #[structopt()]
        position: usize,
    },
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Hello Cargo",
    about = "A command line todo app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}