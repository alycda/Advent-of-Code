// use std::collections::HashMap;

use nom::{
    character::complete::{self, line_ending, space1}, multi::separated_list1, sequence::separated_pair
};

use ornaments::{Solution, AocError};

// pub use self::Day1;
pub use crate::Day1 as Day;

#[derive(Debug, Clone)]
pub struct Day1(Vec<i32>, Vec<i32>);

impl Solution for Day1 {
    type Output = i32;
    type Item = Vec<(i32, i32)>;

    fn parse(input: &str) -> Self {
        let (mut left, mut right): (Vec<i32>, Vec<i32>) = input.lines()
            .map(|line| {
                let nums = line
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                (nums[0], nums[1])
            })
            .unzip();

        left.sort();
        right.sort();

        Self(left, right)
    }

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