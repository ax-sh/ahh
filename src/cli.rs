use clap::{Parser, Subcommand};
use clap_stdin::MaybeStdin;

#[derive(Debug, Subcommand)]
enum Commands {
    /// Lists available options
    #[clap(alias = "ls")]
    List,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// using stdin for positional arg value
    #[clap(default_value = "")]
    pub(crate) value: MaybeStdin<String>,
    /// prompt text for ahh to use
    pub(crate) prompt: Vec<String>,

    /// Run the generated program with debug
    #[clap(short = 'd', long)]
    pub(crate) debug: bool,

    /// Change the ollama model as needed
    #[clap(short = 'm', default_value = "llama3.1:latest")]
    pub(crate) model: String,

    #[command(subcommand)]
    command: Option<Commands>,
}
