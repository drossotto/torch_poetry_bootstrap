use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

use clap::Parser;
use regex::Regex;

mod resolver;
use resolver::{load_sources, resolve_best_source};

mod logs;
use logs::*;

mod tomlgen;
use tomlgen::generate_poetry_source_toml;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    dry_run: bool,

    #[arg(long)]
    print_toml: bool,

    #[arg(long)]
    log: Option<String>,
}

/// Logs a message to both stdout and optionally to a file
fn make_logger(log_path: Option<String>) -> impl Fn(&str) {
    move |msg: &str| {
        println!("{}", msg);
        if let Some(ref path) = log_path {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .expect("Cannot open log file");
            writeln!(file, "{}", msg).unwrap();
        }
    }
}

/// Runs `nvidia-smi` and extracts the CUDA version (e.g., "12.3")
fn detect_cuda_version() -> Option<String> {
    let output = Command::new("nvidia-smi")
        .output()
        .expect(FAILED_NVIDIA_SMI_RUN);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"CUDA Version: (\d+\.\d+)").ok()?;
    let captures = re.captures(&stdout)?;
    Some(captures[1].to_string())
}

fn main() {
    let args = Args::parse();
    let log = make_logger(args.log.clone());

    log(START_DETECTION);

    match detect_cuda_version() {
        Some(ver) => {
            log(&format!("✅ Detected CUDA version: {}", ver));

            log(LOADING_SOURCE_JSON);
            let sources = load_sources("data/cuda_torch_sources.json");

            let selected = resolve_best_source(&ver, &sources);
            log(SOURCE_SELECTED);
            log(&format!("{SELECTED_SOURCE_NAME} {}", selected.source));
            log(&format!("{SELECTED_SOURCE_URL} {}", selected.url));
            
            if args.print_toml {
                let toml = generate_poetry_source_toml(&selected);
                log(&format!("{PRINTED_TOML}\n{}", toml));
            }
        }
        None => {
            log(CUDA_NOT_FOUND);
        }
    } 
}