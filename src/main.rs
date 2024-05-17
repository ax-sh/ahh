use clap::{Parser, Subcommand};
// use reqwest::blocking::Client;
use std::io::{self, BufRead, IsTerminal};

#[derive(Debug, Subcommand)]
enum Commands {
    /// Lists available options
    #[clap(alias = "ls")]
    List,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Description of the command to execute
    prompt: Vec<String>,

    /// Run the generated program without asking for confirmation
    #[clap(short = 'y', long)]
    force: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

fn piped_input() -> String {
    let piped = io::stdin().lock();
    let mut prefix = String::new();

    if piped.is_terminal() {
        // eprintln!("No input provided. Please pipe text or specify a file.");
    } else {
        for line in piped.lines() {
            match line {
                Ok(line) => {
                    prefix.push_str(&line); // Add each line to the collected_input
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
    let args: Cli = Cli::parse();
    // let client = Client::new();
    let prompt = &args.prompt.join(" ");
    let piped = piped_input();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &args.command {
        Some(Commands::List) => {
            println!("'List option")
        }
        None => {
            print!("piped: {}", piped);
            println!("prompt: {}", prompt);
        }
    }
    dbg!(args);
    // let args: Vec<String> = std::env::args().collect();
    // println!("{:?}", args);
}
