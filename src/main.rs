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
        if let Ok(cmd) = prompt.read(machine.get_program_counter()) {
            if cmd == "exit" {
                process::exit(0);
            } else if cmd == "run" || cmd == "r" {
                // TODO: Check if registers are empty and display a warning?
                println!("Ran {} steps", machine.run().bright_white().bold());
            } else if cmd == "step" || cmd == "s" {
                machine.step(true);
            } else if cmd.starts_with("pc") {
                let mut args = cmd.split(" ");
                let arg = args.nth(1);

                if arg.is_some() {
                    let pc = match arg.unwrap().parse::<usize>() {
                        Ok(v) => v,
                        Err(_) => break,
                    };

                    match machine.set_program_counter(pc) {
                        Ok(_) => (),
                        Err(e) => println!("Couldn't set program counter: {}", e),
                    }
                }
                // TODO: :)
            } else if cmd.starts_with("registers") || cmd.starts_with("reg") {
                let mut args = cmd.split(" ");
                let mut arg = args.nth(1);
                
                if arg.is_some() && arg.unwrap().contains("=") {
                    // Set single register to a value
                    let mut args = arg.unwrap().split("=");
                    let reg = match args.next().unwrap().parse::<usize>() {
                        Ok(v) => v,
                        Err(_) => break,
                    };

                    let value = match args.next().unwrap().parse::<i32>() {
                        Ok(v) => v,
                        Err(_) => break,
                    };

                    if reg > 0 {
                        machine.set_register(reg, value);
                    }
                } else if arg.is_some() {
                    // Set all registers
                    let mut i = 1;

                    loop {
                        let value = match arg {
                            Some(v) => v.parse::<i32>(),
                            None => break,
                        };

                        let value: i32 = match value {
                            Ok(v) => v,
                            Err(_) => break,
                        };

                        machine.set_register(i, value);

                        i = i + 1;
                        arg = args.next();
                    }
                }

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
    println!("{}\t\t\tShow program counter value", "pc".bright_white().bold());
    println!("{} {}\t\tSet program counter value", "pc".bright_white().bold(), "value");
    println!("{}\t\tShow registers content", "(reg)isters".bright_white().bold());
    println!("{} {}\t\tSet registers content (R1 = 0, R2 = y...)", "(reg)isters".bright_white().bold(), "x y");
    println!("{} {}\t\tSet registers content (Rx = y)", "(reg)isters".bright_white().bold(), "x=y");
    println!("{}\t\t\tRun the whole program", "(r)un".bright_white().bold());
    println!("{}\t\t\tRun a single instruction", "(s)tep".bright_white().bold());
}