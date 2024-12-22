use std::collections::{HashMap, HashSet};

use crate::{mix, prune};

/// Part 2
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

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, crate::AocError> {
    let output: usize = input.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .fold(HashMap::new(), |mut acc: HashMap<[i32; 4], usize>, num| {
            let (sequence, differences) = process_sequence(num);
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
    
    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1
2
3
2024";
        assert_eq!("23", process(input)?);
        Ok(())
    }
}