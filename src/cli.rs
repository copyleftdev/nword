use clap::Parser;

/// CLI tool to collect, condense, and annotate source code for LLM context.
#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Directory to scan (defaults to the current directory)
    #[arg(default_value = ".")]
    pub directory: String,
}
