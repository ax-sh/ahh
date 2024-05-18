use clap::{Parser, Subcommand};
use colored::Colorize;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use std::fs;
use std::io::{self, BufRead, IsTerminal};

use bat::PrettyPrinter;
use spinoff::{spinners, Color, Spinner};

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

    /// Run the generated program with debug
    #[clap(short = 'd', long)]
    debug: bool,

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
async fn execute_prompt(prompt: &str, piped: &str) {
    let mut spinner = Spinner::new(spinners::Dots, "Loading...", Color::Blue);

    let ollama = Ollama::default();
    let model = "llama3:latest".to_string();
    // dbg!(ollama);

    let prompt_with_instructions = [piped, prompt].join("\n");

    let res = ollama
        .generate(GenerationRequest::new(model, prompt_with_instructions))
        .await;

    if let Ok(res) = res {
        let md = res.response;
        spinner.stop_and_persist("✔".green().to_string().as_str(), "Got some Answers!");

        PrettyPrinter::new()
            .input_from_bytes(md.as_bytes())
            .language("md")
            .print()
            .unwrap();
    }
}

#[tokio::main]
async fn main() {
    let args: Cli = Cli::parse();
    let prompt = &args.prompt.join(" ");
    let piped = piped_input();

    if args.debug {
        println!("---------");
        println!("{}: {}", "[PIPED] ".green(), piped.green());
        println!("{}: {}", "[PROMPT]".green(), prompt.green());

        println!("---------");
        println!("");
    }

    match &args.command {
        Some(Commands::List) => {
            list_prompts();
        }
        None => execute_prompt(&prompt, &piped).await,
    }
}
