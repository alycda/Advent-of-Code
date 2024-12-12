// use std::collections::HashMap;

// use nom::{
//     character::complete::{self, line_ending, space1}, error::Error, multi::separated_list1, sequence::separated_pair, IResult
// };

use ornaments::{Solution, AocError};

// pub use self::Day1;

#[derive(Debug, Clone)]
pub struct Day1(Vec<i32>, Vec<i32>);

// impl Day1 {
//     fn get_parts_mut(&mut self) -> (&mut Vec<i32>, &mut Vec<i32>) {
//         (&mut self.0, &mut self.1)
//     }
// }

impl Solution for Day1 {
    type Output = i32;

    fn parse(input: &str) -> Self {
        let (left, right) = input.lines()
            .map(|line| {
                let nums = line
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                (nums[0], nums[1])
            })
            .unzip();

        Self(left, right)
    }

    // fn nom_parser(input: &str) -> IResult<&str, Self, Error<&str>> {
    //     separated_list1(
    //         line_ending,
    //         separated_pair(
    //             complete::i32,
    //             space1,
    //             complete::i32,
    //         ),
    //     )(input)
    // }

    // fn parse_nom(input: &str) -> Self {
    //     let (_, pairs) = separated_list1(
    //         line_ending,
    //         separated_pair(
    //             complete::i32,
    //             space1,
    //             complete::i32,
    //         ),
    //     )(input).unwrap();
        
    //     let (left, right) = pairs.into_iter().unzip();
    //     Self(left, right)
    // }
    
    fn part1(&mut self) -> Result<Self::Output, AocError> {
        // let Day1(ref mut left, ref mut right) = *self;
        let Day1(left, right) = self.get_mut();
        
        left.sort();
        right.sort();

        let output = left
            .iter()
            .zip(right.iter())
            .map(|(l, r)| (l - r).abs())
            .sum::<Self::Output>();
    
        Ok(output)
    }
    
    fn part2(&mut self) -> Result<Self::Output, AocError> {
        // let (left, right) = (&mut self.0, &mut self.1);
        let Day1(left, right) = self.get_mut();
        
        left.sort();
        right.sort();

        let output = left
            .iter()
            .map(|n| n * right.iter().filter(|&x| x == n).count() as Self::Output)
            .sum::<Self::Output>();
    
        Ok(output)
        
    }
}

// #[derive(Debug, Clone)]
// struct Day1Hashmap(Vec<usize>, HashMap<usize, usize>);

// impl Solution for Day1Hashmap {
//     type Output = i32;

//     fn parse(input: &str) -> Self {
//         let mut left = vec![];
//         let mut right: HashMap<usize, usize> = HashMap::new();

//         for line in input.lines() {
//             let mut items = line.split_whitespace();
//             left.push(
//                 items.next().unwrap().parse::<usize>().unwrap(),
//             );
//             right
//                 .entry(
//                     items
//                         .next()
//                         .unwrap()
//                         .parse::<usize>()
//                         .unwrap(),
//                 )
//                 .and_modify(|v| {
//                     *v += 1;
//                 })
//                 .or_insert(1);
//         };

//         Self(left, right)
//     }

//     // O(n) with constant time lookups using HashMap
//     // pub fn part2(input: &str) -> miette::Result<String> {
//     //     let mut left = vec![];
//     //     let mut right: HashMap<usize, usize> = HashMap::new();

//     //     for line in input.lines() {
//     //         let mut items = line.split_whitespace();
//     //         left.push(
//     //             items.next().unwrap().parse::<usize>().unwrap(),
//     //         );
//     //         right
//     //             .entry(
//     //                 items
//     //                     .next()
//     //                     .unwrap()
//     //                     .parse::<usize>()
//     //                     .unwrap(),
//     //             )
//     //             .and_modify(|v| {
//     //                 *v += 1;
//     //             })
//     //             .or_insert(1);
//     //     }

//     //     let result: usize = left
//     //         .iter()
//     //         .map(|number| {
//     //             number * right.get(number).unwrap_or(&0)
//     //         })
//     //         .sum();

//     //     Ok(result)
//     // }
// }

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
    
    // #[test]
    // fn test_day1_part1_nom() -> miette::Result<()> {
    //     let input = "3   4
    // 4   3
    // 2   5
    // 1   3
    // 3   9
    // 3   3";
    //     assert_eq!("11", Day1::parse_nom(input).solve(Part::One).unwrap());
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
    
//     #[test]
//     fn test_day1_part2_hashmap() -> miette::Result<()> {
//         let input = "3   4
//     4   3
//     2   5
//     1   3
//     3   9
//     3   3";
//         assert_eq!("31", Day1Hashmap::parse(input).solve(Part::One).unwrap());
//     }
}