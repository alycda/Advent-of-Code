//! Day 1: Historian Hysteria
//! 
//! --- Part One ---
//! Pair up the smallest number in the left list with the smallest number in the 
//! right list, then the second-smallest left number with the second-smallest 
//! right number, and so on.
//! 
//! Within each pair, figure out how far apart the two numbers are; you'll need 
//! to add up all of those distances.
//! 
//! To find the total distance between the left list and the right list, add up 
//! the distances between all of the pairs you found.
//! 
//! -- Part Two ---
//! 
//! figure out exactly how often each number from the left list appears in the 
//! right list. Calculate a total similarity score by adding up each number in 
//! the left list after multiplying it by the number of times that number 
//! appears in the right list

// use std::collections::HashMap;

use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    character::complete::{self, digit1, line_ending, space1}, multi::separated_list1, sequence::separated_pair
};

use ornaments::{Solution, AocError};

pub use crate::Day1 as Day;

/// Solution for comparing and matching numbers between two lists
/// 
/// This implementation solves a puzzle where:
/// 1. Numbers from two lists need to be paired by their sorted positions
/// 2. The absolute difference between each pair is calculated
/// 3. All differences are summed to produce a final result
/// 
/// The secondary part handles counting matching numbers between lists
#[derive(Debug, Clone)]
pub struct Day1(Vec<i32>, Vec<i32>);

impl Solution for Day1 {
    type Output = i32;
    type Item = Vec<(i32, i32)>;

    /// Parses input string into two sorted vectors of integers
    /// 
    /// # Arguments
    /// * `input` - String containing pairs of numbers separated by whitespace
    /// 
    /// # Returns
    /// * `Self` - Day1 struct containing two sorted vectors
    ///
    /// # Panics
    /// * If any line doesn't contain exactly two numbers
    /// * If any number cannot be parsed as i32
    fn parse(input: &str) -> Self {
        let (mut left, mut right): (Vec<i32>, Vec<i32>) = input.lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<i32>().expect("a valid number"))
                    .collect_tuple()
                    .expect("Each line must have exactly two numbers")
            })
            .unzip();

        left.sort();
        right.sort();

        Self(left, right)
    }

    /// Nom parser implementation for handling input parsing with error handling
    /// 
    /// Parses lines of space-separated integer pairs using nom combinators
    fn nom_parser(input: &str) -> nom::IResult<&str, Self::Item, nom::error::Error<&str>> {
        separated_list1(
            line_ending::<&str, nom::error::Error<&str>>,
            separated_pair(
                complete::i32,
                space1,
                complete::i32,
            ),
        )(input)
    }
    
    /// Calculates sum of absolute differences between paired numbers
    /// 
    /// Pairs are formed by matching indices in the sorted vectors
    /// 
    /// # Returns
    /// * Sum of absolute differences or error
    fn part1(&mut self) -> Result<Self::Output, AocError> {
        // let Day1(ref mut left, ref mut right) = *self;
        let Day1(left, right) = self.get_mut();

        let output = left
            .iter()
            .zip(right.iter())
            .map(|(l, r)| (l - r).abs())
            .sum::<Self::Output>();
    
        Ok(output)
    }
    
    /// Calculates sum of products between numbers and their frequency matches
    /// 
    /// For each number in left vector, multiply it by how many times it appears
    /// in the right vector
    /// 
    /// # Returns
    /// * Sum of products or error
    fn part2(&mut self) -> Result<Self::Output, AocError> {
        // let (left, right) = (&mut self.0, &mut self.1);
        let Day1(left, right) = self.get_mut();
        
        let output = left
            .iter()
            .map(|n| n * right.iter().filter(|&x| x == n).count() as Self::Output)
            .sum::<Self::Output>();
    
        Ok(output)
        
    }
}

#[derive(Debug, Clone)]
pub struct Day1Hashmap(Vec<usize>, HashMap<usize, usize>);

impl Solution for Day1Hashmap {
    type Output = usize;
    type Item = HashMap<usize, usize>;

    fn parse(input: &str) -> Self {
        let mut left = vec![];
        let mut right: HashMap<usize, usize> = HashMap::new();

        for line in input.lines() {
            let mut items = line.split_whitespace();
            left.push(
                items.next().unwrap().parse::<usize>().unwrap(),
            );
            right
                .entry(
                    items
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                )
                .and_modify(|v| {
                    *v += 1;
                })
                .or_insert(1);
        };

        Self(left, right)
    }

    fn nom_parser(input: &str) -> nom::IResult<&str, Self::Item, nom::error::Error<&str>> {
        let mut map = HashMap::new();

        let (input, pairs) = separated_list1(
            line_ending::<&str, nom::error::Error<&str>>,
            separated_pair(
                digit1,
                space1,
                digit1,
            ),
        )(input)?;

        for (left, right) in pairs {
            map
                .entry(left)
                .and_modify(|v| {
                    *v += 1;
                })
                .or_insert(1);
        }

        todo!();

        // Ok((input, map))
    }

    fn part1(&mut self) -> Result<Self::Output, AocError> {
        unimplemented!("Part 1 not implemented for Day1Hashmap")
        // let Day1Hashmap(left, right) = self.get_mut();

        // let output = left
        //     .iter()
        //     .zip(right.iter())
        //     .map(|(l, r)| (l - r).abs())
        //     .sum::<Self::Output>();

        // Ok(output)
    }

    // O(n) with constant time lookups using HashMap
    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        let Day1Hashmap(left, right) = self.get_mut();

        let result: usize = left
            .iter()
            .map(|number| {
                number * right.get(number).unwrap_or(&0)
            })
            .sum();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ornaments::Part;
    
    #[test]
    fn test_day1_part1() -> miette::Result<()> {
        let input = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";
        assert_eq!("11", Day1::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn day1_nom_parser() {
        let input = "3   4";
        let result = Day1::nom_parser(input);
        assert_eq!(Ok(("", vec![(3, 4)])), result);
    }
    
    // #[test]
    // fn test_day1_part1_nom() -> miette::Result<()> {
    //     let input = "3   4
    // 4   3
    // 2   5
    // 1   3
    // 3   9
    // 3   3";
    //     let (left, right) = input.lines()
    //         .map(|line| Day1::nom_parser(line).unwrap().1)
    //         .inspect(|a|{
    //             dbg!(a);
    //         })
    //         .collect();

    //     // let thing = Day1(left, right);
            
    //     // left.sort();
    //     // right.sort();

    //     // let output = left
    //     //     .iter()
    //     //     .zip(right.iter())
    //     //     .map(|(l, r)| (l - r).abs())
    //     //     .sum::<Day1::Output>();


    //     // let (_, pairs) = Day1::nom_parser(input).unwrap();

    //     // assert_eq!("11", Day1::nom_parser(input).solve(Part::One)?);
    //     assert_eq!("11", Day1(left, right).solve(Part::One)?);
    //     Ok(())
    // }
    
    #[test]
    fn test_day1_part2() -> miette::Result<()> {
        let input = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";
        assert_eq!("31", Day1::parse(input).solve(Part::Two)?);
        Ok(())
    }
    
    // #[test]
    // fn test_day1_part1_hashmap() -> miette::Result<()> {
    //     let input = "3   4
    // 4   3
    // 2   5
    // 1   3
    // 3   9
    // 3   3";
    //     assert_eq!("11", Day1Hashmap::parse(input).solve(Part::One)?);
    //     Ok(())
    // }
    
    #[test]
    fn test_day1_part2_hashmap() -> miette::Result<()> {
        let input = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";
        assert_eq!("31", Day1Hashmap::parse(input).solve(Part::Two)?);
        Ok(())
    }
}