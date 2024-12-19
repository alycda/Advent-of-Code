use itertools::Itertools;

use crate::AocError;

use std::collections::HashSet;

fn can_construct(target: &str, patterns: &HashSet<String>) -> bool {
    let n = target.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;  // Empty string can be constructed in 1 way

    for i in 1..=n {
        for pattern in patterns {
            let pattern_len = pattern.len();
            if i >= pattern_len && &target[i - pattern_len..i] == pattern {
                dp[i] += dp[i - pattern_len];
            }
        }
    }

    dp[n] > 0  // If there's at least one way to construct the string
}

// fn can_construct(target: &str, patterns: &[&str]) -> bool {
//     let pattern_set: HashSet<String> = patterns.iter().map(|s| s.to_string()).collect();
//     let n = target.len();
//     let mut dp = vec![0; n + 1];
//     dp[0] = 1;  // Empty string can be constructed in 1 way

//     for i in 1..=n {
//         for pattern in &pattern_set {
//             let pattern_len = pattern.len();
//             if i >= pattern_len && &target[i - pattern_len..i] == pattern {
//                 dp[i] += dp[i - pattern_len];
//             }
//         }
//     }

//     dp[n] > 0  // If there's at least one way to construct the string
// }

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (patterns_str, lines) = input.split("\n\n").collect_tuple().unwrap();

    let patterns: HashSet<String> = patterns_str
        .split(',') 
        .map(|s| s.trim().to_string())
        .collect();

    let output = lines
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|line| can_construct(&line, &patterns))
        .count();

    // let output = lines.lines().filter(|line| {
    //     // dbg!(line);
    //     can_construct(&line, &parts)
    // }).count();

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
