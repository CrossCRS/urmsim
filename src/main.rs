use std::process;
use std::collections::HashMap;
use clap::Parser;
use owo_colors::OwoColorize;

use crate::machine::Machine;
use crate::parser::UrmParser;
use crate::prompt::Prompt;

mod prompt;
mod parser;
mod machine;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn setup_commands(commands: &mut HashMap<&str, fn(&mut Machine, Vec<&str>)>) {
    commands.insert("exit", |_, _| process::exit(0));

    commands.insert("help", cmd_help);
    commands.insert("h", cmd_help);

    commands.insert("list", cmd_list);
    commands.insert("l", cmd_list);

    commands.insert("pc", cmd_pc);

    commands.insert("registers", cmd_registers);
    commands.insert("reg", cmd_registers);

    commands.insert("run", cmd_run);
    commands.insert("r", cmd_run);

    commands.insert("step", cmd_step);
    commands.insert("s", cmd_step);
}

fn main() {
    // Setup command HashMap
    let mut commands: HashMap<&str, fn(&mut Machine, Vec<&str>)> = HashMap::new();
    setup_commands(&mut commands);

    let args = Cli::parse();
    let mut prompt = Prompt::new();

    // Setup program
    println!("Loading program from {}", args.path.display().bright_white().bold());

    let content = match std::fs::read_to_string(&args.path) {
        Ok(value) => value,
        Err(_) => {
            println!("Couldn't open file {}", args.path.to_str().unwrap_or("").bright_white().bold());
            process::exit(-1);
        },
    };

    let parser = UrmParser::new(content);
    let mut machine = parser.parse();

    println!("Loaded {} instructions", machine.get_instruction_count().bright_white().bold());
    println!();

    // Command loop
    loop {
        if let Ok(cmd) = prompt.read(machine.get_program_counter()) {
            let mut args = cmd.split(' ');
            let cmd = args.next().unwrap_or("");
            let args: Vec<&str> = args.collect();

            if commands.contains_key(cmd) {
                let cmd = commands.get(cmd).unwrap();
                cmd(&mut machine, args);
            } else {
                println!("Unknown command '{}'", cmd.bright_white().bold());
            }

            println!();
        }
    }
}

/// Print all available commands
fn cmd_help(_machine: &mut Machine, _args: Vec<&str>) {
    println!("{}\t\t\tShow this help screen", "(h)elp".bright_white().bold());
    println!("{}\t\t\tShow program instructions", "(l)ist".bright_white().bold());
    println!("{} value\t\tSet program counter value", "pc".bright_white().bold());
    println!("{}\t\tShow registers content", "(reg)isters".bright_white().bold());
    println!("{} x y\t\tSet registers content (R1 = 0, R2 = y...)", "(reg)isters".bright_white().bold());
    println!("{} x=y\t\tSet registers content (Rx = y)", "(reg)isters".bright_white().bold());
    println!("{}\t\t\tRun the whole program", "(r)un".bright_white().bold());
    println!("{}\t\t\tRun a single instruction", "(s)tep".bright_white().bold());
}

/// List currently loaded instructions
fn cmd_list(machine: &mut Machine, _args: Vec<&str>) {
    machine.print_instructions();
}

/// Set program counter
fn cmd_pc(machine: &mut Machine, args: Vec<&str>) {
    let arg = args.first();

    if let Some(pc) = arg {
        let pc = match pc.parse::<usize>() {
            Ok(v) => v,
            Err(_) => return,
        };

        match machine.set_program_counter(pc) {
            Ok(_) => (),
            Err(e) => println!("Couldn't set program counter: {}", e),
        }
    }
}

/// Display or set registers
fn cmd_registers(machine: &mut Machine, args: Vec<&str>) {
    let mut i = 1;

    for arg in &args {
        if arg.contains('=') {
            // Set a specified register
            let mut reg_args = arg.split('=');
            let reg = match reg_args.next().unwrap().parse::<usize>() {
                Ok(v) => v,
                Err(_) => return,
            };

            let value = match reg_args.next().unwrap().parse::<i32>() {
                Ok(v) => v,
                Err(_) => return,
            };

            if reg > 0 {
                machine.set_register(reg, value);
            }
        } else {
            // Set a register based on argument index
            let value: i32 = match arg.parse::<i32>() {
                Ok(v) => v,
                Err(_) => break,
            };

            machine.set_register(i, value);
        }
        i += 1;
    }

    machine.print_registers();
}

/// Run all remaining instructions
fn cmd_run(machine: &mut Machine, _args: Vec<&str>) {
    // TODO: Check if registers are empty and display a warning?
    println!("Ran {} steps", machine.run().bright_white().bold());
}

/// Run a single instruction
fn cmd_step(machine: &mut Machine, _args: Vec<&str>) {
    machine.step(true);
}