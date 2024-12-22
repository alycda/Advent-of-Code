//! Day 22: Monkey Market

use std::collections::{HashMap, HashSet};

use ornaments::{AocError, Solution};

pub use crate::Day22 as Day;

pub struct Day22(Vec<usize>);

impl std::ops::Deref for Day22 {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Day {}

/// Specialized for Part 2, but can be used for part1
fn calculate_next_secret(mut secret: usize) -> (usize, usize) {
    // First operation
    let result = secret * 64;
    secret = mix(secret, result);
    secret = crate::prune(secret);

    // Second operation
    let result = secret / 32;
    secret = mix(secret, result);
    secret = prune(secret);

    // Third operation
    let result = secret * 2048;
    secret = mix(secret, result);
    secret = prune(secret);

    (secret, secret % 10)
}

fn repeat(input: usize, count: usize) -> usize {
    if count == 0 {
        return input;
    }

    repeat(calculate_next_secret(input).0, count - 1)
}

fn process_sequence(input: usize) -> (Vec<usize>, Vec<i32>) {
    let mut x = input;
    let mut sequence = Vec::with_capacity(2000);
    let mut differences = Vec::with_capacity(1999);
    
    let mut prev = x % 10;
    for _ in 0..2000 {
        let (next_x, digit) = calculate_next_secret(x);
        x = next_x;
        sequence.push(digit);
        if sequence.len() > 1 {
            differences.push(digit as i32 - prev as i32);
        }
        prev = digit;
    }
    
    (sequence, differences)
}

impl Solution for Day {
    type Output = usize;
    type Item = ();

    fn parse(input: &str) -> Self {
        Self(input.lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect::<Vec<_>>())
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        let output = self.iter()
            .map(|number| {
                repeat(*number, 2000)
            }).sum();

        Ok(output)
    }

    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        let output = self.iter()
            .fold(HashMap::new(), |mut acc: HashMap<[i32; 4], usize>, num| {
                let (sequence, differences) = process_sequence(*num);
                let mut seen_patterns = HashSet::new();
                
                differences.windows(4)
                    .zip(&sequence[4..])
                    .filter_map(|(window, &price)| {
                        let pattern = [window[0], window[1], window[2], window[3]];
                        seen_patterns.insert(pattern).then_some((pattern, price))
                    })
                    .for_each(|(pattern, price)| {
                        *acc.entry(pattern).or_default() += price;
                    });
                
                acc
            }).values().max().unwrap().to_owned(); // temporary value dropped while borrowed because .values returns a reference and it doesn't make sense to .clone() a Copy type (usize), so we take ownership of it instead

        Ok(output)
    }
}

/// To mix a value into the secret number, calculate the bitwise XOR of 
/// the given value and the secret number. Then, the secret number becomes 
/// the result of that operation.
/// 
/// luckily, XOR is commutative, so the order of the operands doesn't matter.
pub fn mix(secret: usize, result: usize) -> usize {
    result ^ secret
}

/// To prune the secret number, calculate the value of the secret number modulo 16777216. 
/// Then, the secret number becomes the result of that operation.
pub fn prune(secret: usize) -> usize {
    secret % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    use ornaments::Part;

    use rstest::rstest;

    #[rstest]
    #[case((42, 15), 37)]
    #[case((15, 42), 37)]
    fn test_mix(#[case] input: (usize, usize), #[case] expected: usize) {
        assert_eq!(mix(input.0, input.1), expected);
    }

    #[rstest]
    #[case(100000000, 16113920)]
    fn test_prune(#[case] input: usize, #[case] expected: usize) {
        assert_eq!(prune(input), expected);
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "1
10
100
2024";
        assert_eq!("37327623", Day::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "1
2
3
2024";
        assert_eq!("23", Day::parse(input).solve(Part::Two)?);
        Ok(())
    }

    #[rstest]
    #[case((123, 1), 15887950)]
    #[case((123, 2), 16495136)]
    #[case((123, 3), 527345)]
    #[case((123, 4), 704524)]
    #[case((123, 5), 1553684)]
    #[case((123, 6), 12683156)]
    #[case((123, 7), 11100544)]
    #[case((123, 8), 12249484)]
    #[case((123, 9), 7753432)]
    #[case((123, 10), 5908254)]
    fn test_cases(#[case] input: (usize, usize), #[case] expected: usize) {
        assert_eq!(repeat(input.0, input.1), expected);
    }
}