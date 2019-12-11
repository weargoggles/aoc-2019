use std::cell::RefCell;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::io;
use std::rc::Rc;

mod intcode;

fn main() -> io::Result<()> {
    let program = intcode::load("data/day7.txt")?;
    let amplifier = Amplifier {
        program: program.clone(),
    };
    let max = permutations(5)
        .map(|phase_setting| {
            phase_setting
                .iter()
                .fold(0i32, |acc, i| amplifier.amplify(acc, i))
        })
        .max();
    println!("maximum is {:?}", max);

    let part_two_max = permutations(5)
        .map(|phase_setting| {
            let machines: Vec<Rc<RefCell<DaySevenMachine>>> = phase_setting
                .iter()
                .map(|x| x + 5) // range from 5 to 9 now
                .map(|p| {
                    let mut machine = DaySevenMachine::new(program.clone());
                    machine.input_deque.push_back(p.clone().try_into().unwrap());
                    Rc::new(RefCell::new(machine))
                })
                .collect();
            let mut register: i32 = 0;
            for machine in machines.iter().cycle() {
                let mut m = machine.borrow_mut();
                m.input_deque.push_back(register);
                match m.run() {
                    MachineState::Output => {
                        register = m.output_deque.pop_front().unwrap();
                    }
                    MachineState::Finished => {
                        break;
                    }
                    MachineState::Error => panic!("unexpected state"),
                }
            }
            register
        })
        .max();
    println!("Max for part two: {:?}", part_two_max);
    Ok(())
}

struct Amplifier {
    program: intcode::ProgramData,
}

impl Amplifier {
    fn amplify(&self, acc: i32, phase_setting: &usize) -> i32 {
        let mut machine = DaySevenMachine::new(self.program.clone());
        machine
            .input_deque
            .push_back(phase_setting.clone().try_into().unwrap());
        machine.input_deque.push_back(acc);
        machine.run();
        machine.output_deque.pop_front().unwrap()
    }
}

pub fn permutations(size: usize) -> Permutations {
    Permutations {
        idxs: (0..size).collect(),
        swaps: vec![0; size],
        i: 0,
    }
}
pub struct Permutations {
    idxs: Vec<usize>,
    swaps: Vec<usize>,
    i: usize,
}
impl Iterator for Permutations {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() {
                    return None;
                }
                if self.swaps[self.i] < self.i {
                    break;
                }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
}

struct DaySevenMachine {
    input_deque: VecDeque<i32>,
    output_deque: VecDeque<i32>,
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

impl DaySevenMachine {
    fn new(program: intcode::ProgramData) -> DaySevenMachine {
        DaySevenMachine {
            instruction_pointer: 0,
            input_deque: VecDeque::new(),
            output_deque: VecDeque::new(),
            memory: program,
        }
    }
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
        let input = self.input_deque.pop_front().unwrap();
        let destination = self.memory[self.instruction_pointer + 1];
        self.store(destination, input);
        self.instruction_pointer = self.instruction_pointer + 2;
    }

    fn output(&mut self) {
        let a = self.load_parameter(1);
        self.output_deque.push_back(a);
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
    fn run(&mut self) -> MachineState {
        loop {
            match self.memory[self.instruction_pointer] % 100 {
                1 => self.add(self.memory[self.instruction_pointer]),
                2 => self.mul(self.memory[self.instruction_pointer]),
                3 => self.input(),
                4 => {
                    self.output();
                    return MachineState::Output;
                }
                5 => self.jump_if_true(),
                6 => self.jump_if_false(),
                7 => self.less_than(),
                8 => self.equals(),
                99 => {
                    return MachineState::Finished;
                }
                _ => {
                    println!("Exception! {}", self.memory[self.instruction_pointer]);
                    return MachineState::Error;
                }
            }
        }
    }
}

enum MachineState {
    Output,
    Finished,
    Error,
}
