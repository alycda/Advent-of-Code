use crate::custom_error::AocError;

use std::collections::HashMap;

fn expand(input: &str) -> HashMap<usize, usize> {
    let mut d: HashMap<usize, usize> = HashMap::new();
    let mut current_position = 0;
    let mut is_file = true;
    let mut b = 0;

    for c in input.chars() {
        if let Some(length) = c.to_digit(10) {
            let length = length as usize;
            if is_file {
                for loc in 0..length {
                    d.insert(current_position + loc, b);
                }
                b += 1;
                is_file = false;
            } else {
                is_file = true;
            }
            current_position += length;
        }
    }

    d
}

fn rearrange(mut rearranged: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut left = 0;
    let mut right = *rearranged.keys().max().unwrap_or(&0);
    
    while left < right {
        if let Some(file_id) = rearranged.remove(&right) {
            while rearranged.contains_key(&left) {
                left += 1;
            }
            rearranged.insert(left, file_id);
        }
        right -= 1;
    }

    rearranged
}

fn checksum(expanded_rearranged: &HashMap<usize, usize>) -> u128 {
    expanded_rearranged.iter()
        .map(|(loc, file_id)| (*loc as u128) * (*file_id as u128))  // Cast to u128 for multiplication
        .sum()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut expanded = expand(input);
    expanded = rearrange(expanded.clone());
    
    Ok(checksum(&expanded).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // use rstest::rstest;

    // #[rstest]
    // #[case("00...111...2...333.44.5555.6666.777.888899", "0099811188827773336446555566")]
    // fn test_rearrange(#[case] input: &str, #[case] expected: &str) {
    //     assert_eq!(rearrange_loop(input.to_string()), expected);
    // }

    // #[rstest]
    // #[case("00...111...2...333.44.5555.6666.777.888899", 1928)]
    // fn test_checksum(#[case] input: &str, #[case] expected: u64) {
    //     assert_eq!(checksum(rearrange(input)), expected);
    // }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
