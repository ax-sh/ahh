use clap::{Parser, Subcommand};
use colored::Colorize;
use cli::Cli;

mod cli;

fn log_model(model: String) {
    //  format!("{} {} {}", "[USING".green(), model.blue(), "MODEL]".green())

    println!("{} {} {}", "[USING".green(), model.blue(), "MODEL]".green())
}

#[tokio::main]
async fn main() {
    let args: Cli = Cli::parse();
    let model = args.model;
    let prompt = &args.prompt.join(" ");
    let piped_value = args.value;
    log_model(model);
    println!("piped_value string is null or empty: {}", piped_value.is_empty());
    println!("piped_value={}", piped_value);

    if args.debug {
        // dbg!(ollama);
        println!("---------");
        // println!("{}: {}", "[PIPED] ".green(), piped.green());
        println!("{}: {}", "[PROMPT]".green(), prompt.green());

        println!("---------");
    }
}
