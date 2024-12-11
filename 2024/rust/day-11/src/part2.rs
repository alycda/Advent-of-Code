use crate::custom_error::AocError;

use std::collections::HashMap;

fn run(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones: HashMap<usize, usize> = HashMap::new();
    
    for (stone, count) in stones {
        let length = stone.to_string().len();
        
        if stone == 0 {
            *new_stones.entry(1).or_default() += count;
        } else if length % 2 == 0 {
            let divisor = 10_usize.pow((length / 2) as u32);
            *new_stones.entry(stone / divisor).or_default() += count;
            *new_stones.entry(stone % divisor).or_default() += count;
        } else {
            *new_stones.entry(stone * 2024).or_default() += count;
        }
    }
    
    new_stones
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut stones: HashMap<usize, usize> = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .fold(HashMap::new(), |mut acc, stone| {
            *acc.entry(stone).or_default() += 1;
            acc
        });

    for _ in 0..75 {
        stones = run(stones);
    }
    
    Ok(stones.values().sum::<usize>().to_string())
}