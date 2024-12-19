use itertools::Itertools;

use crate::AocError;

use std::collections::BTreeMap;

fn preprocess_patterns<'a>(patterns: &'a [&str]) -> BTreeMap<char, Vec<&'a str>> {
    patterns.iter()
        .fold(BTreeMap::new(), |mut acc, &pattern| {
            if let Some(c) = pattern.chars().next() {
                acc.entry(c).or_default().push(pattern);
            }
            acc
        })
}

fn can_construct(target: &str, patterns: &[&str]) -> bool {
    let pattern_map = preprocess_patterns(patterns);
    dfs_string_match(target, &pattern_map)
}

fn dfs_string_match(remaining: &str, pattern_map: &BTreeMap<char, Vec<&str>>) -> bool {
    // Base case: successfully matched everything
    if remaining.is_empty() {
        return true;
    }
    
    // Get first char without iterating whole string
    let first_char = remaining.chars().next().unwrap();
    
    // Try patterns that start with our current character
    if let Some(valid_patterns) = pattern_map.get(&first_char) {
        for &pattern in valid_patterns {
            if remaining.starts_with(pattern) {
                // Use slice instead of position tracking
                if dfs_string_match(&remaining[pattern.len()..], pattern_map) {
                    return true;
                }
            }
        }
    }
    
    false
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
