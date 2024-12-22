use std::collections::HashSet;

use crate::AocError;

/// To mix a value into the secret number, calculate the bitwise XOR of 
/// the given value and the secret number. Then, the secret number becomes 
/// the result of that operation.
/// 
/// luckily, XOR is commutative, so the order of the operands doesn't matter.
fn mix(secret: usize, result: usize) -> usize {
    result ^ secret
}

/// To prune the secret number, calculate the value of the secret number modulo 16777216. 
/// Then, the secret number becomes the result of that operation.
fn prune(secret: usize) -> usize {
    secret % 16777216
}

fn calculate_next_secret(mut secret: usize) -> (usize, usize) {
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

    // dbg!(secret, secret % 10);

    (secret, secret % 10)
}


fn repeat(input: usize, count: usize) -> Vec<(usize, usize)> {
    fn inner(input: usize, count: usize, mut acc: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        if count == 0 {
            return acc;
        }
        
        let next = calculate_next_secret(input);
        acc.push(next);
        inner(next.0, count - 1, acc)
    }

    inner(input, count, Vec::with_capacity(count))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let sequences = input.lines()
        // .flat_map(|line| {
        //     let number: usize = line.parse().unwrap();

        //     repeat(number, 10).iter().unzip().1
        // })
        .map(|line| {
            let number: usize = line.parse().unwrap();

            repeat(number, 10).iter()
                .map(|v| v.1)
                .collect::<Vec<usize>>()
                // differences
                .windows(2)
                .map(|window| {
                    // window[1] is current, window[0] is previous
                    (window[1], window[1] as i32 - window[0] as i32)
                }).collect::<Vec<_>>()
        })
        // .inspect(|v| {
        //     dbg!(v);
        // })
        .map(|differences| {
            differences.windows(4)
                .map(|w| {
                    let pattern = (w[0].1, w[1].1, w[2].1, w[3].1);
                    let price = w[3].0;  // The price after the pattern
                    (pattern, price)
                })
                .collect::<Vec<_>>()
        })
        .inspect(|v| {
            dbg!(v);
        })
        .collect::<Vec<_>>();

    let max_bananas = sequences.iter()
        .flat_map(|buyer_sequences| {
            // Get all patterns from this buyer
            buyer_sequences.iter().map(|(pattern, _)| *pattern)
        })
        .collect::<HashSet<_>>() // Get unique patterns
        .into_iter()
        .map(|pattern| {
            sequences.iter()
                .map(|buyer_sequences| {
                    buyer_sequences.iter()
                        .find(|(p, _)| *p == pattern)
                        .map_or(0, |(_, price)| *price)
                })
                .sum::<usize>()
        })
        .max()
        .unwrap_or(0);

// panic!("at the disco");
    Ok(max_bananas.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // use rstest::rstest;

    // #[rstest]
    // #[case("", "")]
    // fn test_cases(#[case] input: &str, #[case] expected: &str) {
    //     assert_eq!(process(input).unwrap(), expected);
    // }

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
