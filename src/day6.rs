use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::rc::Rc;

fn main() -> io::Result<()> {
    let f = fs::File::open("data/day6.txt")?;
    let reader = io::BufReader::new(f);
    let orbits = Rc::new(read_orbits(reader));

    let checksum = count_orbits(orbits.clone());
    println!("Orbits: {}", checksum);
    let mut you_chain = chain_for(orbits.clone(), "YOU".to_string());
    you_chain.reverse();
    let mut san_chain = chain_for(orbits.clone(), "SAN".to_string());
    san_chain.reverse();

    println!("y: {:?}, s: {:?}", you_chain.last(), san_chain.last());
    let mut counter = 0;
    for (y, s) in you_chain.iter().zip(san_chain.iter()) {
        counter = counter + 1;
        if y != s {
            println!(
                "hops: {}",
                (you_chain.len() - 1 - counter) + (san_chain.len() - 1 - counter)
            );
            break;
        }
    }

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

fn count_orbits(orbits: Rc<HashMap<String, String>>) -> i32 {
    let mut counter = 0;
    for body in orbits.keys() {
        let mut secondary = body;
        loop {
            match orbits.get(secondary) {
                Some(primary) => {
                    secondary = primary;
                    counter = counter + 1;
                }
                None => break,
            }
        }
    }
    counter
}

fn chain_for(orbits: Rc<HashMap<String, String>>, body: String) -> Vec<String> {
    let mut chain: Vec<String> = vec![body.clone()];
    let mut secondary = &body.clone();
    loop {
        match orbits.get(secondary) {
            Some(primary) => {
                chain.push(secondary.clone());
                secondary = primary;
            }
            None => break,
        }
    }
    chain
}

#[test]
fn test_count_orbits() {
    let f = fs::File::open("data/day6-example.txt").unwrap();
    let reader = io::BufReader::new(f);
    let orbits = Rc::new(read_orbits(reader));

    let checksum = count_orbits(orbits);
    assert_eq!(checksum, 42);
}
