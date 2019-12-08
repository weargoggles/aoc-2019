use std::fs;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn main() -> Result<(), std::io::Error> {
    let f = fs::File::open("data/day1.txt")?;
    let modules: Vec<i64> = io::BufReader::new(f)
        .lines()
        .map(Result::unwrap)
        .map(|s| i64::from_str(&s))
        .map(Result::unwrap)
        .collect();
    let sum: i64 = modules.iter().cloned().map(fuel_for).sum();
    let sum_with_tyranny: i64 = modules.iter().cloned().map(fuel_for_fuel).sum();
    println!("My fuel requirement is {}", sum);
    println!(
        "With the tyranny of the rocket equation, that's {}",
        sum_with_tyranny
    );
    Ok(())
}

fn fuel_for(mass: i64) -> i64 {
    let fuel = (mass / 3) - 2;
    fuel
}

fn fuel_for_fuel(mass: i64) -> i64 {
    let fuel = fuel_for(mass);
    if fuel > 0 {
        fuel + fuel_for_fuel(fuel)
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fuel_examples() {
        assert_eq!(fuel_for(12), 2);
        assert_eq!(fuel_for(14), 2);
        assert_eq!(fuel_for(1969), 654);
        assert_eq!(fuel_for(100756), 33583);
    }

    #[test]
    fn test_fuel_for_fuel_examples() {
        assert_eq!(fuel_for_fuel(14), 2);
        assert_eq!(fuel_for_fuel(1969), 966);
        assert_eq!(fuel_for_fuel(100756), 50346);
    }
}
