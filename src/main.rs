use std::process;
use clap::Parser;
use owo_colors::OwoColorize;

use crate::parser::UrmParser;
use crate::prompt::Prompt;

mod prompt;
mod parser;
mod machine;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let mut prompt = Prompt::new();
    println!("Loading program from {}", args.path.display().bright_white().bold());

    let content = std::fs::read_to_string(&args.path).expect("Couldn't open file");

    let parser = UrmParser::new(content);
    let mut machine = parser.parse();

    println!("Loaded {} instructions", machine.get_instruction_count().bright_white().bold());
    println!();

    loop {
        if let Ok(cmd) = prompt.read() {
            if cmd == "exit" {
                process::exit(0);
            } else if cmd == "run" || cmd == "r" {
                println!("Ran {} steps", machine.run().bright_white().bold());
            } else if cmd == "step" || cmd == "s" {
                machine.step(true);
            } else if cmd == "registers" || cmd == "reg" {
                machine.print_registers();
            } else if cmd == "list" || cmd == "l" {
                machine.print_instructions();
            } else if cmd == "help" || cmd == "h" || cmd == "?" {
                print_help();
            }
            println!();
        }
    }
}

fn print_help() {
    println!("{}\t\t\tShow this help screen", "(h)elp".bright_white().bold());
    println!("{}\t\t\tShow program instructions", "(l)ist".bright_white().bold());
    println!("{}\t\tShow registers content", "(reg)isters".bright_white().bold());
    println!("{}\t\t\tRun the whole program", "(r)un".bright_white().bold());
    println!("{}\t\t\tRun a single instruction", "(s)tep".bright_white().bold());
}