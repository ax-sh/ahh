use clap::Parser;
// use reqwest::blocking::Client;
use std::io::{self, BufRead, IsTerminal};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Description of the command to execute
    prompt: Vec<String>,

    /// Run the generated program without asking for confirmation
    #[clap(short = 'y', long)]
    force: bool,
}

fn main() {
    // let cli = Cli::parse();
    // // let client = Client::new();
    // let prompt = &cli.prompt.join(" ");
    // println!("{}", prompt);

    // let args: Vec<String> = std::env::args().collect();
    // println!("{:?}", args);

    let piped = io::stdin().lock();

    if piped.is_terminal() {
        eprintln!("No input provided. Please pipe text or specify a file.");
    } else {
        for line in piped.lines() {
            let line = line.expect("Failed to read line from stdin");
            // Process the line of text
            println!("Line: {}", line);
        }
    }
}
