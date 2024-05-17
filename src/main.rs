use clap::{Parser, Subcommand};
use colored::Colorize;
use ollama_rs::Ollama
// use reqwest::blocking::Client;
use std::fs;
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

fn list_prompts() {
    let paths = fs::read_dir("./src/prompts").unwrap();
    for path in paths {
        let file_path = path.unwrap().path();
        println!("Prompt File Path: {}", file_path.display())
    }
}
fn execute_prompt(prompt:&str, piped:&str){
    // let ollama = Ollama::default();
    println!("---------");
    println!("{}: {}", "[PIPED] ".green(), piped.green());
    println!("{}: {}", "[PROMPT]".green(), prompt.green());
    println!("---------");

    // let model = "llama3:latest".to_string();
    // let prompt = "Why is the sky blue?".to_string();

    // let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    // if let Ok(res) = res {
    //     println!("{}", res.response);
    // }
}

fn main() {
    let args: Cli = Cli::parse();
    // let client = Client::new();
    let prompt = &args.prompt.join(" ");
    let piped = piped_input();

    match &args.command {
        Some(Commands::List) => {
            list_prompts();
        }
        None => {
          execute_prompt(&prompt, &piped)
        }
    }
    // dbg!(args);
    // let args: Vec<String> = std::env::args().collect();
    // println!("{:?}", args);
}
