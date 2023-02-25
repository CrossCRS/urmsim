use std::{str::FromStr, fmt::Display};
use comfy_table::{Table, presets::UTF8_NO_BORDERS};

pub enum InstructionType {
    JUMP,
    SUCCESSOR,
    TRANSFER,
    ZERO,
}

impl FromStr for InstructionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "J" => Ok(Self::JUMP),
            "S" => Ok(Self::SUCCESSOR),
            "T" => Ok(Self::TRANSFER),
            "Z" => Ok(Self::ZERO),
            _   => Err(()),
        }
    }
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JUMP => write!(f, "J"),
            Self::SUCCESSOR => write!(f, "S"),
            Self::TRANSFER => write!(f, "T"),
            Self::ZERO => write!(f, "Z"),
        }
    }
}

pub struct Instruction {
    pub(crate) instruction_type: InstructionType,
    pub(crate) args: Vec<i32>,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args_str = self.args.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        return write!(f, "{}({})", self.instruction_type, args_str);
    }
}

pub struct Machine {
    registers: Vec<i32>,
    instructions: Vec<Instruction>,
    program_counter: usize,
}

impl Machine {
    pub fn new(registers: Vec<i32>, instructions: Vec<Instruction>) -> Self {
        return Self { registers, instructions, program_counter: 0 };
    }

    /// Prints all the used register and their values in the form of a table
    pub fn print_registers(&self) {
        if self.registers.is_empty() {
            println!("All registers are empty.");
            return;
        }

        let mut headers: Vec<String> = Vec::new();
        for (i, _reg) in self.registers.iter().enumerate() {
            headers.push(format!("R{}", i + 1));
        }
        let mut table = Table::new();

        table.load_preset(UTF8_NO_BORDERS)
            .set_header(&headers)
            .add_row(&self.registers);

        println!();
        println!("{table}");
    }

    /// Prints out a listing of all the loaded instructions
    pub fn print_instructions(&self) {
        for (i, instr) in self.instructions.iter().enumerate() {
            println!("{}: {}", i + 1, instr);
        }
    }

    /// Returns total count of loaded instructions
    pub fn get_instruction_count(&self) -> usize {
        return self.instructions.len();
    }

    /// Returns the value of register `r`
    /// 
    /// # Arguments
    /// 
    /// * `r` - Register index (starting at 1)
    pub fn get_register(&self, r: usize) -> i32 {
        if self.registers.len() < r {
            return 0; // Default register value is 0
        }

        return self.registers[r - 1];
    }

    /// Sets the value of register `r` to `value`
    /// 
    /// # Arguments
    /// 
    /// * `r` - Register index (starting at 1)
    /// * `value` - New value
    pub fn set_register(&mut self, r: usize, value: i32) {
        if self.registers.len() < r {
            self.registers.resize(r, 0);
        }
        
        self.registers[r - 1] = value;
    }

    /// Runs a single instruction pointed at by the program counter
    /// 
    /// # Arguments
    /// 
    /// * `print_instruction` - If true, also prints the executed instruction in the format of INSTR_CODE(arguments)
    pub fn step(&mut self, print_instruction: bool) {
        if print_instruction {
            println!("{}", &self.instructions[self.program_counter]);
        }

        let instruction = &self.instructions[self.program_counter];
        match instruction.instruction_type {
            InstructionType::JUMP => {
                // J(m, n, i) = jump to instruction i if reg[m] == reg[n]
                if self.get_register(instruction.args[0] as usize) == self.get_register(instruction.args[1] as usize) {
                    self.program_counter = (instruction.args[2] as usize) - 1;
                } else {
                    self.program_counter += 1;
                }
            },
            InstructionType::SUCCESSOR => {
                // S(m) = increment reg[m]
                self.set_register(instruction.args[0] as usize, self.get_register(instruction.args[0] as usize) + 1);
                self.program_counter += 1;
            },
            InstructionType::TRANSFER => {
                // T(m, n) = copy from reg[m] to reg[n]
                self.set_register(instruction.args[1] as usize, self.get_register(instruction.args[0] as usize));
                self.program_counter += 1;
            },
            InstructionType::ZERO => {
                // Z(m) = zero reg[m]
                self.set_register(instruction.args[0] as usize, 0);
                self.program_counter += 1;
            },
        }
    }

    /// Runs all the remaining instructions
    pub fn run(&mut self) -> i32 {
        let mut counter = 0;
        while self.program_counter < self.instructions.len() {
            self.step(false);
            counter += 1;
        }
        return counter;
    }
}