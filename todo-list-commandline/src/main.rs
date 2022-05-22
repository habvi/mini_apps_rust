mod cli;
use structopt::StructOpt;

fn main() {
    // cargo run -> help message!
    cli::CommandLineArgs::from_args();

    // cargo run -- add "task"
    // cargo run -- done 4
    // cargo run -- -j xxx.txt list
    println!("{:#?}", cli::CommandLineArgs::from_args());
}
