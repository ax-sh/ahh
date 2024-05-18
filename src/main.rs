use clap::{Parser, Subcommand};
use colored::Colorize;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use std::io::{self, BufRead, IsTerminal};

use std::{env, fs, process};

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
    let current_dir = env::current_dir().unwrap();

    let paths = fs::read_dir(current_dir.join("./src/prompts")).unwrap();
    for path in paths {
        let file_path = path.unwrap().path();
        println!("Prompt File Path: {}", file_path.display())
    }
}

fn get_default_prompt() -> String {
    // TODO temp hard code as the alternative doesnt work in diff dir
    return String.from(
        "
**Instructions**
- Your answer should be on point without fluff
- Answers should be shorter

Input:
",
    );
    // let current_dir = env::current_dir().unwrap();
    // // note doesnt work if using from diff dir
    // let file_path = current_dir.join("./src/prompts/default_prompt.md");
    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // return contents;
}

async fn execute_prompt(prompt: &str, piped: &str, model: &str) {
    println!("");
    let mut spinner = Spinner::new(spinners::Dots, " Loading... ", Color::Yellow);

    let ollama = Ollama::default();
    let instructions = if piped.is_empty() {
        get_default_prompt()
    } else {
        piped.to_string()
    };

    let prompt_with_instructions = [&instructions, prompt].join("\n");

    let res = ollama
        .generate(GenerationRequest::new(
            model.to_string(),
            prompt_with_instructions,
        ))
        .await;

    if let Ok(res) = res {
        let md = res.response;
        spinner.stop_and_persist(
            " ✔ ".green().to_string().as_str(),
            "Got some Answers!".green().to_string().as_str(),
        );

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
    let model = "llama3:latest";
    if prompt.trim().is_empty() {
        eprintln!("{}", "No Prompt provided. Exiting".red());
        process::exit(0);
    }

    if args.debug {
        // dbg!(ollama);
        println!("---------");
        println!("{}: {}", "[PIPED] ".green(), piped.green());
        println!("{}: {}", "[PROMPT]".green(), prompt.green());

        println!("---------");
        println!("");
    }

    match &args.command {
        Some(Commands::List) => list_prompts(),
        None => execute_prompt(&prompt, &piped, model).await,
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_external_prompts() {
        println!(
            "The current directory is: {}",
            env::var("CARGO_MANIFEST_DIR")
        );
    }
    #[test]
    fn test_default_ahh_execution() {
        let mut cmd = Command::cargo_bin("ahh").unwrap();

        cmd.args(&["Which year did the Titanic sink?", "(Just the number)"])
            .assert()
            .success()
            .stderr("")
            .stdout(predicate::str::contains("1912"));
    }

    //   #[test]
    //   fn test_empty_ahh_execution() {
    //     let mut cmd = Command::cargo_bin("ahh").unwrap();

    //     cmd
    //       .args([" "])
    //       .assert()
    //       .success()
    //       .stderr("")
    //   }
}
