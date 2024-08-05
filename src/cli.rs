use bat::PrettyPrinter;
use clap::{Parser, Subcommand};
use std::io;
use std::io::{BufRead, IsTerminal};

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Lists available options
    #[clap(alias = "ls")]
    List,
    Fast {
        prompt: Vec<String>,
    },
    Hustle {
        prompt: Vec<String>,
    },
    Icon {
        prompt: Vec<String>,
    },
    Brand {
        prompt: Vec<String>,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// consider all other args as prompt
    pub(crate) prompt: Vec<String>,

    /// Run the generated program with debug
    #[clap(short = 'd', long)]
    pub(crate) debug: bool,

    /// Change the ollama model as needed
    #[clap(short = 'm', default_value = "")]
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
    println!();
    PrettyPrinter::new()
        .input_from_bytes(md.as_bytes())
        .language("md")
        .print()
        .unwrap();
    println!()
}
