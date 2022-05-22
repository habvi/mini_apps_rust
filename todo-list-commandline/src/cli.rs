use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal_file.
    Add {
        /// The task description text.
        #[structopt()]
        text: String
    },
    /// Remove the journal_file by position.
    Done {
        #[structopt()]
        position: usize
    },
    /// List all tasks in the journal_file.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Todo List",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    // Use a different journal_file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}