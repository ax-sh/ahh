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

fn piped_input() -> String {
    let piped = io::stdin().lock();
    let mut prefix = String::new();

    if piped.is_terminal() {
        eprintln!("No input provided. Please pipe text or specify a file.");
    } else {
        for line in piped.lines() {
            match line {
                Ok(line) => {
                    // Add each line to the collected_input
                    prefix.push_str(&line);
                    prefix.push('\n'); // Preserve newline character
                }
                Err(err) => {
                    eprintln!("Error reading line: {}", err);
                }
            }
        }
    }
    return prefix;
}

fn main() {
    let cli = Cli::parse();
    // let client = Client::new();
    let prompt = &cli.prompt.join(" ");
    println!("{}", prompt);
    let piped = piped_input();
    print!("{}", piped)
    // let args: Vec<String> = std::env::args().collect();
    // println!("{:?}", args);
}
