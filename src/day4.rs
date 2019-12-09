use std::convert::From;
use std::ops::Deref;

#[derive(Clone, Eq, PartialEq)]
struct Password(Vec<i32>);

impl From<i32> for Password {
    fn from(item: i32) -> Self {
        let mut digits: Vec<i32> = Vec::new();
        let mut n = item;
        while n > 9 {
            digits.push(n % 10);
            n = n / 10;
        }
        digits.push(n);
        digits.reverse();
        Password(digits)
    }
}

#[test]
fn test_password_from() {
    assert_eq!(Password::from(1234).0, vec![1i32, 2, 3, 4]);
}

impl Deref for Password {
    type Target = Vec<i32>;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

trait PasswordRule {
    fn validate(&self, input: &Password) -> bool;
}

struct NumDigits(usize);

impl PasswordRule for NumDigits {
    fn validate(&self, input: &Password) -> bool {
        input.0.len() == self.0.into()
    }
}

#[test]
fn test_sixdigits() {
    let six_digits = NumDigits(6);
    assert!(six_digits.validate(&123456.into()));
    assert!(!six_digits.validate(&12345.into()));
    assert!(!six_digits.validate(&1234567891.into()));
    assert!(NumDigits(10).validate(&1234567891.into()));
}

struct TwoAdjacentIdenticalDigits;

impl PasswordRule for TwoAdjacentIdenticalDigits {
    fn validate(&self, input: &Password) -> bool {
        let iter_a = input.iter();
        let mut iter_b = input.iter();
        iter_b.next();
        for (a, b) in iter_a.zip(iter_b) {
            if a == b {
                return true;
            }
        }
        false
    }
}

#[test]
fn test_two_adjacent() {
    assert!(TwoAdjacentIdenticalDigits {}.validate(&1223.into()));
    assert!(!TwoAdjacentIdenticalDigits {}.validate(&1234.into()));
}

struct AlwaysIncreasing;

impl PasswordRule for AlwaysIncreasing {
    fn validate(&self, input: &Password) -> bool {
        let iter_a = input.iter();
        let mut iter_b = input.iter();
        iter_b.next();
        for (a, b) in iter_a.zip(iter_b) {
            if b < a {
                return false;
            }
        }
        true
    }
}

#[test]
fn test_always_increasing() {
    let rule = AlwaysIncreasing {};
    assert!(rule.validate(&1234.into()));
    assert!(!rule.validate(&4321.into()));
    assert!(rule.validate(&1233.into()));
    assert!(!rule.validate(&1232.into()));
}

fn main() {
    let rules: Vec<Box<dyn PasswordRule>> = vec![
        Box::new(TwoAdjacentIdenticalDigits {}),
        Box::new(NumDigits(6)),
        Box::new(AlwaysIncreasing {}),
    ];
    println!(
        "matching passwords: {}",
        (357253..892942)
            .map(Password::from)
            .filter(|v| rules.iter().all(|r| r.validate(v)))
            .count()
    );
}
