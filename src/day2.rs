use std::fs;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn main() -> Result<(), std::io::Error> {
    let f = fs::File::open("data/day2.txt")?;
    let reader = io::BufReader::new(f);
    let original_program: Vec<usize> = reader
        .split(b',')
        .map(Result::unwrap)
        .map(|v| String::from_utf8(v))
        .map(Result::unwrap)
        .map(|s| usize::from_str((&s).trim()))
        .map(Result::unwrap)
        .collect();

    // before starting, fiddle the numbers
    let mut program = original_program.clone();
    program[1] = 12;
    program[2] = 2;
    let output = run(program);

    println!("Position 0: {}", output[0]);

    for i in 0..99 {
        for j in 0..99 {
            let mut program = original_program.clone();
            program[1] = i;
            program[2] = j;
            let output = run(program);
            if output[0] == 19690720 {
                println!("The inputs {} and {} produce the desired output.", i, j);
                println!("The answer is: {}", 100 * i + j);
            }
        }
    }
    Ok(())
}

fn run(input: Vec<usize>) -> Vec<usize> {
    let mut memory = input.clone();
    let mut i = 0; // the instruction pointer
    loop {
        match memory[i] {
            1 => {
                let a = memory[i + 1];
                let b = memory[i + 2];
                let c = memory[i + 3];
                memory[c] = memory[a] + memory[b];
                i = i + 4;
            }
            2 => {
                let a = memory[i + 1];
                let b = memory[i + 2];
                let c = memory[i + 3];
                memory[c] = memory[a] * memory[b];
                i = i + 4;
            }
            99 => {
                break;
            }
            _ => {
                println!("Exception!");
                break;
            }
        }
    }
    memory
}
