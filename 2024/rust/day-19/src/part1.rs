use itertools::Itertools;

use crate::AocError;

use std::collections::HashSet;

fn count_ways(patterns: &HashSet<String>, target: &str) -> usize {
    let n = target.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        for pattern in patterns {
            let pattern_len = pattern.len();
            if i >= pattern_len && &target[i - pattern_len..i] == pattern {
                dp[i] += dp[i - pattern_len];
            }
        }
    }

    dp[n]
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (patterns_str, lines) = input.split("\n\n").collect_tuple().unwrap();

    let patterns: HashSet<String> = patterns_str
        .split(',') 
        .map(|s| s.trim().to_string())
        .collect();

    let output = lines
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|line| count_ways(&patterns, line) > 0)
        .count();

    Ok(output.to_string())
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
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
