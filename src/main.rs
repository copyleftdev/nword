mod cli;
mod file_processor;
mod filters;
mod language_detection;
mod text_processing;

use serde::Serialize;
use file_processor::FileData;
use std::fs::File;
use std::io::Write;
use chrono::Utc;
use clap::Parser;

#[derive(Serialize)]
struct Output {
    files: Vec<FileData>,
}

fn main() {
    let args = cli::Args::parse();
    let files = file_processor::process_directory(&args.directory);
    let output = Output { files };
    let timestamp = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let file_name = format!("project_snapshot_{}.json", timestamp);

    let mut file = match File::create(&file_name) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create file {}: {}", file_name, e);
            return;
        }
    };

    if let Err(e) = write!(file, "{}", serde_json::to_string(&output).unwrap()) {
        eprintln!("Failed to write to file {}: {}", file_name, e);
    }

    println!("Snapshot saved to: {}", file_name);
}
