use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let f = fs::File::open("data/day6.txt")?;
    let reader = io::BufReader::new(f);
    let orbits = read_orbits(reader);

    let checksum = count_orbits(orbits);
    println!("Orbits: {}", checksum);

    Ok(())
}

fn read_orbits<T: io::Read>(reader: io::BufReader<T>) -> HashMap<String, String> {
    reader
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            let sep = l.find(")").unwrap();
            (String::from(&l[(sep + 1)..]), String::from(&l[0..sep]))
        })
        .collect()
}

fn count_orbits(orbits: HashMap<String, String>) -> i32 {
    let mut counter = 0;
    for body in orbits.keys() {
        let mut secondary = body;
        loop {
            println!("In the loop for {}", secondary);
            match orbits.get(secondary) {
                Some(primary) => {
                    println!("we have a primary: {}", primary);
                    secondary = primary;
                    counter = counter + 1;
                }
                None => break,
            }
        }
        println!("Done for {}", body);
    }
    counter
}

#[test]
fn test_count_orbits() {
    let f = fs::File::open("data/day6-example.txt").unwrap();
    let reader = io::BufReader::new(f);
    let orbits = read_orbits(reader);

    let checksum = count_orbits(orbits);
    assert_eq!(checksum, 42);
}
