use ornaments::Solution;
use miette::Context;

use day_11::Day11 as Day;

// use crate::custom_error::AocError;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
}

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

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

// #[tracing::instrument]
fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input.txt");
    // let result = process(file).context("process part 2")?;
    let result = Day::parse(file).solve(ornaments::Part::Two).context("process part 2")?;

    println!("{}", result);
    Ok(())
}