use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn deal_into_new<T: Clone>(mut input: Vec<T>) -> Vec<T> {
    input.reverse();
    input
}

fn cut<T: Clone>(mut input: Vec<T>, n: i32) -> Vec<T> {
    if n == 0 {
        input
    } else if n > 0 {
        let mut tail = input.split_off(n as usize);
        tail.append(&mut input);
        tail
    } else {
        let offset: i32 = (input.len() as i32) + (n as i32);
        let mut tail = input.split_off(offset as usize);
        tail.append(&mut input);
        tail
    }
}

fn deal_with_increment<T: Clone + Debug>(input: Vec<T>, increment: i32) -> Vec<T> {
    let width = input.len();
    let mut output: Vec<Option<T>> = {
        let mut output = Vec::new();
        for _ in 0..width {
            output.push(None);
        }
        output
    };
    let mut offset: usize = 0;
    for i in input.iter() {
        output.get_mut(offset).unwrap().replace(i.clone());
        println!("output: {:?}", output);
        offset = offset + (increment as usize);
        offset = offset % (width as usize);
    }
    let real_output: Option<Vec<T>> = output.into_iter().collect();
    real_output.unwrap()
}

#[derive(PartialEq, Eq, Debug)]
struct Deck(Vec<i32>);

impl Deck {
    fn new(n: i32) -> Deck {
        Deck((0..n).collect())
    }

    fn cut(self, n: i32) -> Deck {
        Deck(cut(self.0, n))
    }

    fn deal_into_new(self) -> Deck {
        Deck(deal_into_new(self.0))
    }

    fn deal_with_increment(self, n: i32) -> Deck {
        Deck(deal_with_increment(self.0, n))
    }

    fn vec(self) -> Vec<i32> {
        self.0
    }
}

impl From<Vec<i32>> for Deck {
    fn from(v: Vec<i32>) -> Deck {
        Deck(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cut_positive() {
        assert_eq!(cut(vec![0, 1, 2, 3, 4, 5], 3), vec![3, 4, 5, 0, 1, 2]);
    }

    #[test]
    fn test_cut_negative() {
        assert_eq!(cut(vec![0, 1, 2, 3, 4, 5], -2), vec![4, 5, 0, 1, 2, 3]);
    }

    #[test]
    fn test_deal_with_increment() {
        assert_eq!(
            deal_with_increment(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3),
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]
        );
    }

    #[test]
    fn test_1() {
        assert_eq!(
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
            Deck::new(10)
                .deal_with_increment(7)
                .deal_into_new()
                .deal_into_new()
                .vec()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Deck::new(10)
                .cut(6)
                .deal_with_increment(7)
                .deal_into_new()
                .vec(),
            vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Deck::new(10)
                .deal_into_new()
                .cut(-2)
                .deal_with_increment(7)
                .cut(8)
                .cut(-4)
                .deal_with_increment(7)
                .cut(3)
                .deal_with_increment(9)
                .deal_with_increment(3)
                .cut(-1)
                .vec(),
            vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]
        )
    }
}

enum Instruction {
    Cut(i32),
    DealIntoNewStack,
    DealWithIncrement(i32),
}

impl Instruction {
    fn load(path: Path) -> std::io::Result<Vec<Instruction>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        reader.lines().map(|l| {
            
        }).collect()
    }
}

fn main() ->  std::io::Result<()> {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    let x = deal_into_new(v);
    println!("x: {:?}", x);
}
