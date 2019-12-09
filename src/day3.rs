use std::collections::hash_set::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan(&self) -> i32 {
        return self.x.abs() + self.y.abs();
    }
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn step(&self, p: Point) -> Point {
        match self {
            Self::Up => Point { y: p.y + 1, ..p },
            Self::Down => Point { y: p.y - 1, ..p },
            Self::Left => Point { x: p.x - 1, ..p },
            Self::Right => Point { x: p.x + 1, ..p },
        }
    }
}

#[derive(Clone, Debug)]
struct Instruction {
    direction: Direction,
    distance: i32,
}

impl Instruction {
    fn step(&self, start: Point) -> Vec<Point> {
        let mut steps: Vec<Point> = Vec::new();
        let mut next = start.clone();
        for _ in 0..self.distance {
            next = self.direction.step(next);
            steps.push(next.clone());
        }
        steps
    }
}

#[derive(Debug)]
enum InstructionParseError {
    IntError(ParseIntError),
    DirectionError,
}

impl From<ParseIntError> for InstructionParseError {
    fn from(e: ParseIntError) -> Self {
        InstructionParseError::IntError(e)
    }
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s.as_bytes()[0] {
            b'U' => Ok(Direction::Up),
            b'D' => Ok(Direction::Down),
            b'L' => Ok(Direction::Left),
            b'R' => Ok(Direction::Right),
            _ => Err(InstructionParseError::DirectionError),
        }?;
        let distance = i32::from_str(&s[1..])?;
        Ok(Instruction {
            direction,
            distance,
        })
    }
}

fn main() -> Result<(), std::io::Error> {
    let f = fs::File::open("data/day3.txt")?;
    let reader = io::BufReader::new(f);
    let instructions: Vec<Vec<Instruction>> = reader
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            let instructions = l
                .split(',')
                .map(Instruction::from_str)
                .map(Result::unwrap)
                .collect();
            instructions
        })
        .collect();

    // make hash sets
    let first = visit(instructions.get(0).unwrap().clone());
    let first_set: HashSet<Point> = first.iter().cloned().collect();
    let second = visit(instructions.get(1).unwrap().clone());
    let second_set: HashSet<Point> = second.iter().cloned().collect();
    println!("path lengths: {}, {}", first.len(), second.len());

    let mut shared: Vec<Point> = first_set.intersection(&second_set).cloned().collect();
    {
        // if it loops back around to the origin (0, 0) might appear again
        let origin = shared.iter().position(|i| i == &Point { x: 0, y: 0 });
        if origin.is_some() {
            shared.remove(origin.unwrap());
        }
    };
    shared.sort_by(|p, q| (p.manhattan()).cmp(&q.manhattan()));
    println!(
        "First and last: {:?} (distance {}), {:?} (distance {})",
        shared.get(1),
        shared.get(1).unwrap().manhattan(),
        shared.last(),
        shared.last().unwrap().manhattan()
    );

    // work out
    let mut closest_connection = shared
        .iter()
        .map(|p| {
            (
                p,
                first.iter().position(|i| i == p).unwrap()
                    + second.iter().position(|i| i == p).unwrap(),
            )
        })
        .collect::<Vec<(&Point, usize)>>();
    closest_connection.sort_by(|p, q| p.1.cmp(&q.1));
    println!(
        "closest connection: {:?}, furthest: {:?}, all: {:?}",
        closest_connection.first(),
        closest_connection.last(),
        closest_connection
    );

    Ok(())
}

fn visit(instructions: Vec<Instruction>) -> Vec<Point> {
    let mut points_visited: Vec<Point> = Vec::new();
    points_visited.push(Point { x: 0, y: 0 });
    let all_points =
        instructions
            .iter()
            .fold(points_visited, |mut a: Vec<Point>, i: &Instruction| {
                let new_points = i.step(a.last().unwrap().clone());
                a.extend(new_points);
                a
            });
    all_points
}
