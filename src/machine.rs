use std::{str::FromStr, fmt::Display};
use comfy_table::{Table, presets::UTF8_NO_BORDERS};

pub enum InstructionType {
    Jump,
    Successor,
    Transfer,
    Zero,
}

impl FromStr for InstructionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "J" => Ok(Self::Jump),
            "S" => Ok(Self::Successor),
            "T" => Ok(Self::Transfer),
            "Z" => Ok(Self::Zero),
            _   => Err(()),
        }
    }
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Jump => write!(f, "J"),
            Self::Successor => write!(f, "S"),
            Self::Transfer => write!(f, "T"),
            Self::Zero => write!(f, "Z"),
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
        write!(f, "{}({})", self.instruction_type, args_str)
    }
}

pub struct Machine {
    registers: Vec<i32>,
    instructions: Vec<Instruction>,
    program_counter: usize,
}

impl Machine {
    pub fn new(registers: Vec<i32>, instructions: Vec<Instruction>) -> Self {
        Self { registers, instructions, program_counter: 0 }
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

    /// Resets program counter and registers
    pub fn reset(&mut self) {
        self.registers.clear();
        self.program_counter = 0;
    }

    /// Returns total count of loaded instructions
    pub fn get_instruction_count(&self) -> usize {
        self.instructions.len()
    }

    /// Returns current program counter value (starting at 1)
    pub fn get_program_counter(&self) -> usize {
        self.program_counter + 1
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

        self.registers[r - 1]
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

    /// Sets the program counter value to `pc`
    /// 
    /// # Arguments
    /// 
    /// * `pc` - New program counter value (starting at 1)
    pub fn set_program_counter(&mut self, pc: usize) -> Result<usize, &'static str> {
        if pc > self.instructions.len() || pc < 1 {
            return Err("Invalid program counter value");
        }

        self.program_counter = pc - 1;
        
        Ok(self.get_program_counter())
    }

    /// Runs a single instruction pointed at by the program counter
    /// 
    /// # Arguments
    /// 
    /// * `print_instruction` - If true, also prints the executed instruction in the format of INSTR_CODE(arguments)
    pub fn step(&mut self, print_instruction: bool) {
        if self.program_counter >= self.instructions.len() {
            println!("No instructions left");
            return;
        }

        if print_instruction {
            println!("{}", &self.instructions[self.program_counter]);
        }

        let instruction = &self.instructions[self.program_counter];
        match instruction.instruction_type {
            InstructionType::Jump => {
                // J(m, n, i) = jump to instruction i if reg[m] == reg[n]
                if self.get_register(instruction.args[0] as usize) == self.get_register(instruction.args[1] as usize) {
                    self.program_counter = (instruction.args[2] as usize) - 1;
                } else {
                    self.program_counter += 1;
                }
            },
            InstructionType::Successor => {
                // S(m) = increment reg[m]
                self.set_register(instruction.args[0] as usize, self.get_register(instruction.args[0] as usize) + 1);
                self.program_counter += 1;
            },
            InstructionType::Transfer => {
                // T(m, n) = copy from reg[m] to reg[n]
                self.set_register(instruction.args[1] as usize, self.get_register(instruction.args[0] as usize));
                self.program_counter += 1;
            },
            InstructionType::Zero => {
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

        counter
    }
}