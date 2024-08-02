use bat::PrettyPrinter;
use clap::{Parser, Subcommand};
use std::io;
use std::io::{BufRead, IsTerminal};

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Lists available options
    #[clap(alias = "ls")]
    List,
    #[clap(alias = "fast")]
    Fast { prompt: Vec<String> },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Description of the command to execute
    pub(crate) prompt: Vec<String>,

    /// Run the generated program with debug
    #[clap(short = 'd', long)]
    pub(crate) debug: bool,

    /// Change the ollama model as needed
    #[clap(short = 'm', default_value = "llama3.1:latest")]
    pub(crate) model: String,

    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}


// pub fn piped_input() -> String {
//     let mut prefix = String::new();
//     io::stdin().read_to_string(&mut prefix).unwrap_or_else(|err| {
//         eprintln!("Error reading input: {}", err);
//     });
//     prefix
// }

pub fn piped_input() -> String {
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

pub fn print_markdown(md: String) {
    PrettyPrinter::new()
        .input_from_bytes(md.as_bytes())
        .language("md")
        .print()
        .unwrap();
}
