use std::str::FromStr;
use crate::machine::{Machine, Instruction, InstructionType};

pub struct UrmParser {
    program_string: String,
}

impl UrmParser {
    pub fn new(string: String) -> Self {
        return Self { program_string: string };
    }

    pub fn parse(&self) -> Machine {
        let mut registers: Vec<i32> = Vec::new();
        let mut instructions: Vec<Instruction> = Vec::new();

        for (i, line) in self.program_string.lines().enumerate() {
            // Cleanup line and remove comments
            let mut line_copy = line.replace(" ", "").trim().to_uppercase();

            // Remove comments
            let comment_offset = line_copy.find(";");
            if let Some(i) = comment_offset {
                line_copy.replace_range(i..line_copy.len(), "");
            }

            // Don't process empty lines
            if line_copy.len() == 0 {
                continue;
            }

            if i == 0 { // First line contains register values
                for register in line_copy.split(',') {
                    registers.push(register.parse().unwrap());
                }
            } else {
                let instruction = self.parse_instruction(&mut line_copy);

                // print!("Instruction: {}, args [", instruction.instruction_type.to_string());
                // for arg in &instruction.args {
                //     print!("{arg} ");
                // }
                // println!("]");
                
                instructions.push(instruction);
            }
        }

        return Machine::new(registers, instructions);
    }

    fn parse_instruction(&self, line: &mut String) -> Instruction {
        let instruction_type: InstructionType;
        let mut args: Vec<i32> = Vec::new();

        // Parse instruction type
        instruction_type = InstructionType::from_str(&(line.chars().nth(0).unwrap()).to_string()).unwrap();

        // Remove (
        line.replace_range(..2, "");
        // Remove ) and everything after
        line.replace_range(line.find(")").unwrap()..line.len(), "");

        // Parse args
        for arg in line.split(',') {
            args.push(arg.parse().unwrap());
        }

        return Instruction { instruction_type, args }
    }
}