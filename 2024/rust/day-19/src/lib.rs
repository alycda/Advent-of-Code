use std::collections::HashSet;

use itertools::Itertools;
use ornaments::{AocError, Solution};

pub use crate::Day19 as Day;

#[derive(Debug)]
pub struct Day19(String, HashSet<String>);

impl std::ops::Deref for Day19 {
    type Target = HashSet<String>;
    
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

/// Consume
impl IntoIterator for Day19 {
    type Item = String;
    type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.1.into_iter()
    }
}

/// Reference
impl<'a> IntoIterator for &'a Day19 {
    type Item = &'a String;
    type IntoIter = std::collections::hash_set::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.1.iter()
    }
}

impl Day19 {
    fn count(&self, target: &str) -> usize {
        let n = target.len();
        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            for pattern in self {
                let pattern_len = pattern.len();
                if i >= pattern_len && &target[i - pattern_len..i] == pattern {
                    dp[i] += dp[i - pattern_len];
                }
            }
        }

        dp[n]
    }
}

impl<'a> Solution for Day19 {
    type Output = usize;
    type Item = ();

    fn parse(input: &str) -> Self {
        let (patterns, towels) = input.split("\n\n").collect_tuple().unwrap();

        let patterns: HashSet<String> = patterns
            .split(',') 
            .map(|s| s.trim().to_string())
            .collect();

        Self(towels.to_owned(), patterns)
    }

    fn part1(&mut self) -> Result<Self::Output, AocError> {
        let output = self.0
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|line| self.count(&line) > 0)
            .count();

        Ok(output)
    }

    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        let output = self.0
            .lines()
            .map(|s| s.trim().to_string())
            .map(|line| self.count(&line))
            .sum();

        Ok(output)        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!("6", Day::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!("16", Day::parse(input).solve(Part::Two)?);
        Ok(())
    }
}