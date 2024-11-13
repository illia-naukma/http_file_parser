use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self};
use std::process::exit;

use http_file_parser::parser::parse;
use http_file_parser::request::make_request;

#[derive(Parser, Debug)]
#[command(
    author = "Your Name <your.email@example.com>",
    version = "1.0",
    about = "HTTP Request File Parser",
    long_about = "This CLI parses .http files and allows you to make requests based on the parsed data.",
    disable_help_flag = true,
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Parse {
        #[arg(short, long)]
        file: String,
    },
    Credits,
    Help,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { file } => {
            let http_content = fs::read_to_string(file).unwrap_or_else(|err| {
                eprintln!("Error reading file '{}': {}", file, err);
                exit(1);
            });

            let http_file = match parse(&http_content) {
                Ok(http_file) => http_file,
                Err(e) => {
                    eprintln!("Failed to parse HTTP file: {}", e);
                    exit(1)
                }
            };

            println!("Parsed HTTP Requests:");
            for (i, request) in http_file.requests.iter().enumerate() {
                println!("{}. {} {}", i + 1, request.method, request.url);
            }

            println!("Enter the number of the request you want to make:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let choice: usize = input.trim().parse().expect("Please enter a valid number");
            if choice < 1 || choice > http_file.requests.len() {
                eprintln!("Invalid request number selected.");
                exit(1);
            }

            match make_request(&http_file.requests[choice - 1]) {
                Ok(response) => {
                    println!("Status: {}", response.status());
                    match response.text() {
                        Ok(body) => println!("Body: {}", body),
                        Err(e) => eprintln!("Failed to read response body: {}", e),
                    }
                }
                Err(e) => eprintln!("Request failed: {}", e),
            }
        }
        Commands::Credits => {
            println!("HTTP Request File Parser CLI");
            println!("Version: 1.0");
            println!("Author: Illia Tsymbal <illia.tsymbal@ukma.edu.ua>");
            println!("This tool was developed for educational purposes to demonstrate parsing and handling of HTTP files using Rust.");
        }
        Commands::Help => {
            println!("HTTP Request File Parser CLI");
            println!();
            println!("USAGE:");
            println!("  http_file_parser <COMMAND>");
            println!();
            println!("COMMANDS:");
            println!("  parse   Parses an HTTP file and makes requests based on parsed data.");
            println!("          Options:");
            println!("            --file <FILE>     Specify the path to the .http file to parse.");
            println!();
            println!("  credits Shows credits and authorship information.");
            println!();
            println!("  help    Displays this help information.");
            println!();
        }
    }
}
