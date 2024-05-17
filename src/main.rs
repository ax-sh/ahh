use clap::Parser;
use reqwest::blocking::Client;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Description of the command to execute
    prompt: Vec<String>,

    /// Run the generated program without asking for confirmation
    #[clap(short = 'y', long)]
    force: bool,
}

fn main() {
    let cli = Cli::parse();
    let client = Client::new();
    let prompt = &cli.prompt.join(" ");
    println!("{}", &prompt)
}