use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;

pub type ProgramData = Vec<i32>;

pub fn load<P: AsRef<Path>>(path: P) -> io::Result<ProgramData> {
    let f = fs::File::open(path)?;
    let reader = io::BufReader::new(f);
    let program: Vec<i32> = reader
        .split(b',')
        .map(Result::unwrap)
        .map(|v| String::from_utf8(v))
        .map(Result::unwrap)
        .map(|s| i32::from_str((&s).trim()))
        .map(Result::unwrap)
        .collect();
    Ok(program)
}

pub trait Machine {
    fn run(&mut self, program: ProgramData) -> ProgramData;

    fn reset(&mut self);
}
