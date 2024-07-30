use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Debug, Subcommand)]
enum Commands {
    /// Lists available options
    #[clap(alias = "ls")]
    List,
}
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// prompt text for ahh to use
    prompt: Vec<String>,

    /// Run the generated program with debug
    #[clap(short = 'd', long)]
    debug: bool,

    /// Change the ollama model as needed
    #[clap(short = 'm', default_value = "llama3.1:latest")]
    model: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

fn log_model(model: String) {
    //  format!("{} {} {}", "[USING".green(), model.blue(), "MODEL]".green())

    println!("{} {} {}", "[USING".green(), model.blue(), "MODEL]".green())
}

#[tokio::main]
async fn main() {
    let args: Cli = Cli::parse();
    let model = args.model;
    let prompt = &args.prompt.join(" ");
    log_model(model);

    if args.debug {
        // dbg!(ollama);
        println!("---------");
        // println!("{}: {}", "[PIPED] ".green(), piped.green());
        println!("{}: {}", "[PROMPT]".green(), prompt.green());

        println!("---------");
    }
}
