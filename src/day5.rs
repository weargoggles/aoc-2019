use std::convert::TryInto;
use std::io;
use std::str::FromStr;

mod intcode;

fn main() -> std::io::Result<()> {
    let program = intcode::load("data/day5.txt")?;
    let mut machine: Box<dyn intcode::Machine> = Box::new(DayFiveMachine {
        memory: Vec::new(),
        instruction_pointer: 0,
    });
    machine.run(program);
    Ok(())
}

struct DayFiveMachine {
    instruction_pointer: usize,
    memory: intcode::ProgramData,
}

enum ParameterMode {
    Position,
    Immediate,
}

fn i32tousize(x: i32) -> usize {
    x.try_into().unwrap()
}

impl DayFiveMachine {
    fn get_parameter_mode(opcode: i32, parameter_number: i32) -> ParameterMode {
        let p: u32 = parameter_number.try_into().unwrap();
        match (opcode / 10i32.pow(p + 1)) % 10i32.pow(p) {
            1 => ParameterMode::Immediate,
            _ => ParameterMode::Position,
        }
    }

    fn load(&self, mode: ParameterMode, parameter: i32) -> i32 {
        match mode {
            ParameterMode::Immediate => parameter,
            ParameterMode::Position => self.memory[i32tousize(parameter)],
        }
    }

    fn store(&mut self, location: i32, value: i32) {
        self.memory[i32tousize(location)] = value;
    }

    fn add(&mut self, opcode: i32) {
        let a = self.memory[self.instruction_pointer + 1];
        let b = self.memory[self.instruction_pointer + 2];
        let c = self.memory[self.instruction_pointer + 3];
        let a_value = self.load(Self::get_parameter_mode(opcode, 1), a);
        let b_value = self.load(Self::get_parameter_mode(opcode, 2), b);
        self.store(c, a_value + b_value);
        self.instruction_pointer = self.instruction_pointer + 4;
    }

    fn mul(&mut self, opcode: i32) {
        let a = self.memory[self.instruction_pointer + 1];
        let b = self.memory[self.instruction_pointer + 2];
        let c = self.memory[self.instruction_pointer + 3];
        let a_value = self.load(Self::get_parameter_mode(opcode, 1), a);
        let b_value = self.load(Self::get_parameter_mode(opcode, 2), b);
        self.store(c, a_value * b_value);
        self.instruction_pointer = self.instruction_pointer + 4;
    }

    fn input(&mut self) {
        let mut input = String::new();
        let destination = self.memory[self.instruction_pointer + 1];
        println!("Input: ");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let value = i32::from_str(&input.trim()).unwrap();
                self.store(destination, value);
            }
            Err(error) => panic!("error: {}", error),
        }
        self.instruction_pointer = self.instruction_pointer + 2;
    }

    fn output(&mut self) {
        let a = self.load_parameter(1);
        println!("Output: {}", a);
        self.instruction_pointer = self.instruction_pointer + 2;
    }

    fn jump_if_true(&mut self) {
        let a = self.load_parameter(1);
        let b = self.load_parameter(2);
        if a != 0 {
            self.instruction_pointer = i32tousize(b);
        } else {
            self.instruction_pointer = self.instruction_pointer + 3;
        }
    }

    fn jump_if_false(&mut self) {
        let a = self.load_parameter(1);
        let b = self.load_parameter(2);
        if a == 0 {
            self.instruction_pointer = i32tousize(b);
        } else {
            self.instruction_pointer = self.instruction_pointer + 3;
        }
    }

    fn instruction(&self) -> i32 {
        self.memory[self.instruction_pointer]
    }

    fn load_parameter(&self, parameter: i32) -> i32 {
        let mode = Self::get_parameter_mode(self.instruction(), parameter);
        self.load(
            mode,
            self.memory[self.instruction_pointer + i32tousize(parameter)],
        )
    }

    fn less_than(&mut self) {
        let a = self.load_parameter(1);
        let b = self.load_parameter(2);
        self.store(
            self.memory[self.instruction_pointer + 3],
            if a < b { 1 } else { 0 },
        );
        self.instruction_pointer = self.instruction_pointer + 4
    }

    fn equals(&mut self) {
        let a = self.load_parameter(1);
        let b = self.load_parameter(2);
        self.store(
            self.memory[self.instruction_pointer + 3],
            if a == b { 1 } else { 0 },
        );
        self.instruction_pointer = self.instruction_pointer + 4
    }
}

impl intcode::Machine for DayFiveMachine {
    fn reset(&mut self) {
        self.instruction_pointer = 0;
    }
    fn run(&mut self, program: intcode::ProgramData) -> intcode::ProgramData {
        self.memory = program;
        loop {
            match self.memory[self.instruction_pointer] % 100 {
                1 => self.add(self.memory[self.instruction_pointer]),
                2 => self.mul(self.memory[self.instruction_pointer]),
                3 => self.input(),
                4 => self.output(),
                5 => self.jump_if_true(),
                6 => self.jump_if_false(),
                7 => self.less_than(),
                8 => self.equals(),
                99 => {
                    break;
                }
                _ => {
                    println!("Exception! {}", self.memory[self.instruction_pointer]);
                    break;
                }
            }
        }
        self.memory.clone()
    }
}
