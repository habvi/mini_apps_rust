use std::path::PathBuf;

pub enum Action {
    Add { text: String },
    Done { position: usize },
    List,
}

pub struct CommandLineArgs {
    pub action: Action,
    pub journal_file: Option<PathBuf>,
}