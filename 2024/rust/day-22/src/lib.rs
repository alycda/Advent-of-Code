use miette::Diagnostic;
use thiserror::Error;

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

/// Part 1
pub fn calculate_next_secret(mut secret: usize) -> usize {
    // First operation
    let result = secret * 64;
    secret = mix(secret, result);
    secret = prune(secret);

    // Second operation
    let result = secret / 32;
    secret = mix(secret, result);
    secret = prune(secret);

    // Third operation
    let result = secret * 2048;
    secret = mix(secret, result);
    secret = prune(secret);

    secret
}

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
}

pub mod part1;
pub mod part2;