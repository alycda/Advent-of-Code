use itertools::Itertools;

use crate::AocError;

fn can_construct(target: &str, patterns: &[&str]) -> bool {
    dfs_string_match(target, patterns, 0)
}

fn dfs_string_match(target: &str, patterns: &[&str], pos: usize) -> bool {
    // Base case - successfully reached end of string
    if pos == target.len() {
        return true;
    }
    
    // Base case - went past end
    if pos > target.len() {
        return false;
    }

    let current_char = target.chars().nth(pos).unwrap();

    // Try each pattern that could match at current position
    for pattern in patterns {
        if pattern.starts_with(current_char) {
            if let Some(next_pos) = try_move(target, pos, pattern) {
                if dfs_string_match(target, patterns, next_pos) {
                    return true;
                }
                // Backtracking happens automatically here when false
            }
        }
    }

    false
}

fn try_move(target: &str, pos: usize, pattern: &str) -> Option<usize> {
    // Check if pattern would fit in remaining string
    if pos + pattern.len() > target.len() {
        return None;
    }

    // Check if pattern fully matches at this position
    if target[pos..].starts_with(pattern) {
        Some(pos + pattern.len())
    } else {
        None
    }
}

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (a, lines) = input.split("\n\n").collect_tuple().unwrap();

    // dbg!(a, lines);

    let mut parts = a.split(", ").collect::<Vec<_>>();

    parts.sort();

    // dbg!(&parts);

    let output = lines.lines().filter(|line| {
        // dbg!(line);
        can_construct(&line, &parts)
    }).count();

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
