use clap::Parser;
use colored::Colorize;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use rust_embed::Embed;

use std::process;
// use std::{env, fs, process};

use crate::cli::print_markdown;
// use crate::config::Config;
use cli::{Cli, Commands};
use spinoff::{spinners, Color, Spinner};

mod cli;
mod config;

// fn list_prompts_in_project() {
// todo only works when on the project folder fix this for getting the resources from binary
//     let current_dir = env::current_dir().unwrap();
//
//     let paths = fs::read_dir(current_dir.join("./src/prompts")).unwrap();
//     for path in paths {
//         let file_path = path.unwrap().path();
//         // todo make prompts
//         println!("Prompt File Path: {}", file_path.display())
//     }
// }

#[derive(Embed)]
#[folder = "src/prompts/"]
struct Asset;

fn load_prompt(prompt_file_path: &str) -> String {
    let md = Asset::get(prompt_file_path).unwrap();
    let default_prompt = std::str::from_utf8(md.data.as_ref()).unwrap();
    return default_prompt.to_string();
}
async fn list_prompts() {
    // let ollama = Ollama::default();
    // let models = ollama.list_local_models().await.unwrap();
    // for variant in models.as_slice() {
    //     println!("{}", variant.name.green());
    // }

    for file in Asset::iter() {
        println!("{}", file);
    }
    // list_prompts_in_project()
}

fn get_default_prompt() -> String {
    return load_prompt("default_prompt.md");
}

fn get_hustle_prompt() -> String {
    return load_prompt("hustle.md");
}

async fn execute_prompt(prompt: &str, piped: &str, model: &str) {
    if prompt.trim().is_empty() {
        eprintln!("{}", "No Prompt provided. Exiting".red());
        process::exit(0);
    }
    println!();
    let mut spinner = Spinner::new(spinners::Monkey, "Thinking...", Color::Yellow);

    let ollama = Ollama::default();
    let instructions = if piped.is_empty() {
        get_default_prompt()
    } else {
        piped.to_string()
    };

    println!("[[inst]] {}", instructions);
    println!();

    let prompt_with_instructions = [&instructions, prompt].join("\n");

    let res = ollama
        .generate(GenerationRequest::new(
            model.to_string(),
            prompt_with_instructions,
        ))
        .await;

    match res {
        Ok(res) => {
            let md = res.response;
            spinner.success("Got some Answers!".green().to_string().as_str());

            print_markdown(md);
            println!()
        }
        Err(err) => {
            spinner.fail(err.to_string().as_str());
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Cli = Cli::parse();
    // let _config = Config::new();
    let prompt = &args.prompt.join(" ");
    let piped = cli::piped_input();
    let mut model = args.model;
    if model.is_empty() {
        model = String::from("llama3.1:latest")
    }

    if model.is_empty() {
        eprintln!("model flag is empty");
        process::exit(0);
    }

    if args.debug {
        // dbg!(ollama);
        println!("---------");
        println!("{}: {}", "[PIPED] ".green(), piped.green());
        println!("{}: {}", "[PROMPT]".green(), prompt.green());

        println!("---------");
        println!();
    }

    match &args.command {
        Some(Commands::List) => list_prompts().await,
        Some(Commands::Fast { prompt }) => {
            execute_prompt(
                &prompt.join(" "),
                "Instructions\n AI, I'm in a hurry. Give me a bite-sized summary on\n\nInput",
                &model,
            )
            .await
        }
        Some(Commands::Icon { prompt }) => {
            execute_prompt(&prompt.join(" "), &load_prompt("generate_icon.md"), &model).await
        }
        Some(Commands::Hustle { prompt }) => {
            execute_prompt(&prompt.join(" "), &get_hustle_prompt(), &model).await
        }
        None => execute_prompt(&prompt, &piped, &model).await,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use assert_cmd::Command;
    use config::Config;
    use directories::ProjectDirs;
    use predicates::prelude::*;

    use crate::get_default_prompt;

    #[test]
    fn test_check_dir() {
        if let Some(proj_dirs) = ProjectDirs::from("com", "ax-sh", "ahh") {
            let dir = proj_dirs.config_dir();
            eprint!("DIR-----> {}", dir.display())
            // Lin: /home/alice/.config/barapp
            // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
            // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
        }
    }

    #[test]
    fn test_external_prompts() {
        let default_prompt = get_default_prompt();
        println!("{}", default_prompt)
    }

    #[test]
    fn test_config() {
        let settings = Config::builder()
            // Add in `./Settings.toml`
            .add_source(config::File::with_name("./src/Settings"))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();
        let pref = settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap();
        let priority: &String = pref.get("priority").unwrap();
        print!("[[PRIORITY]] {}", priority);
        // Print out our settings (as a HashMap)
        eprintln!("{:?}", pref);
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
