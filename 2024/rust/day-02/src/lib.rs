use nom::{character::complete::{self, line_ending, space1}, multi::separated_list1};
use ornaments::{Solution, AocError};

#[derive(Debug, PartialEq)]
pub struct Sequence(Vec<i32>);

impl std::ops::Deref for Sequence {
    type Target = Vec<i32>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Sequence {
    fn is_safe(&self) -> bool {
        if self.0.len() < 2 { 
            return false; 
        }
        
        let is_ascending = self.0[0] < self.0[1];
        let mut prev = self.0[0];
        
        self.0.iter()
            .skip(1)
            .all(|&n| {
                let valid = if is_ascending {
                    n > prev && (n - prev).abs() <= 3
                } else {
                    n < prev && (n - prev).abs() <= 3
                };
                prev = n;
                valid
            })
    }

    fn can_dampen(&self) -> bool {
        let nums = &self.0;
        nums.iter().enumerate().any(|(skip_idx, _)| {
            let filtered: Vec<_> = nums.iter()
                .enumerate()
                .filter(|(i, _)| *i != skip_idx)
                .map(|(_, &n)| n)
                .collect();
                
            Sequence(filtered).is_safe()
        })
    }
}

pub struct Day2(Vec<Sequence>);

impl std::ops::Deref for Day2 {
    type Target = Vec<Sequence>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Solution for Day2 {
    type Output = usize;
    type Item = Vec<Vec<i32>>;

    fn parse(input: &str) -> Self {
        let sequences = input.lines()
            .filter_map(|line| {
                let nums: Vec<i32> = line.split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect();
                if !nums.is_empty() {
                    Some(Sequence(nums))
                } else {
                    None
                }
            })
            .collect();

        Self(sequences)
    }

    fn nom_parser(input: &str) -> nom::IResult<&str, Self::Item, nom::error::Error<&str>> {
        separated_list1(
            line_ending,
            separated_list1(space1, complete::i32),
        )(input)
    }

    fn part1(&mut self) -> Result<Self::Output, AocError> {
        Ok(self.iter()
            .filter(|seq| seq.is_safe())
            .count())
    }

    fn part2(&mut self) -> Result<Self::Output, AocError> {
        let (safe, notsafe): (Vec<_>, Vec<_>) = self.iter()
            .partition(|seq| seq.is_safe());

        let dampened = notsafe.iter()
            .filter(|seq| seq.can_dampen())
            .count();

        Ok(safe.len() + dampened)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ornaments::Part;
    
    #[test]
    fn test_day2_part1() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", Day2::parse(input).solve(Part::One)?);
        Ok(())
    }

//     #[test]
//     fn day2_nom_parser() {
//         let input = "7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9";

//         let (_, parsed) = Day2::nom_parser(input).unwrap();
//         assert_eq!(vec![
//             Sequence(vec![7, 6, 4, 2, 1]),
//             Sequence(vec![1, 2, 7, 8, 9]),
//             Sequence(vec![9, 7, 6, 2, 1]),
//             Sequence(vec![1, 3, 2, 4, 5]),
//             Sequence(vec![8, 6, 4, 4, 1]),
//             Sequence(vec![1, 3, 6, 7, 9]),
//         ], parsed);
//     }

    #[test]
    fn test_day2_part2() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", Day2::parse(input).solve(Part::Two)?);
        Ok(())
    }
}