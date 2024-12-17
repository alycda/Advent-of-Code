use std::collections::HashMap;

use ornaments::{AocError, Solution};

#[derive(Debug, Clone)]
pub struct Day11(HashMap<usize, usize>);

impl std::ops::Deref for Day11 {
    type Target = HashMap<usize, usize>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl Iterator for Day11 {
//     type Item = (usize, usize);

//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.iter().next().map(|(k, v)| (*k, *v))
//     }
// }

impl Day11 {
    /// recursion
    pub fn blink_again(input: &str, times: usize) -> String {
        // Base case - if times is 0, return the input
        if times == 0 {
            return input.to_string();
        }

        let numbers: Vec<usize> = input
            .split_whitespace()
            .flat_map(|stone| {
                if stone == "0" {
                    vec![1]
                } else if stone.len() % 2 == 0 {
                    let (left, right) = stone.split_at(stone.len() / 2);
                    vec![
                        left.parse().unwrap(),
                        right.parse().unwrap()
                    ]
                } else {
                    vec![stone.parse::<usize>().unwrap() * 2024]
                }
            })
            .collect();

        // Convert back to string and recurse
        let new_input = numbers.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        Day11::blink_again(&new_input, times - 1)
    }

    /// DO NOT make this a method, it will cause exponential runtime
    fn blink(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
        let mut new_stones: HashMap<usize, usize> = HashMap::new();
    
        for (stone, count) in stones {
            let length = stone.to_string().len();
            
            if stone == 0 {
                *new_stones.entry(1).or_default() += count;
            } else if length % 2 == 0 {
                let divisor = 10_usize.pow((length / 2) as u32);
                *new_stones.entry(stone / divisor).or_default() += count;
                *new_stones.entry(stone % divisor).or_default() += count;
            } else {
                *new_stones.entry(stone * 2024).or_default() += count;
            }
        }
        
        new_stones
    }
}

impl Solution for Day11 {
    type Output = usize;
    type Item = String;

    fn parse(input: &str) -> Self {
        let stones: HashMap<usize, usize> = input
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .fold(HashMap::new(), |mut acc, stone| {
                *acc.entry(stone).or_default() += 1;
                acc
            });

        Self(stones)
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        // Ok(Day11::blink_again(self, 25).split_whitespace().count())

        for _ in 0..25 {
            self.0 = Day11::blink(self.0.clone());
        }
        // let mut stones = self.0.clone();

        // for _ in 0..25 {
        //     stones = Day11::blink(stones);
        // }

        Ok(self.values().sum::<usize>())
    }

    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        let mut stones = self.0.clone();

        for _ in 0..75 {
            stones = Day11::blink(stones);
        }

        *self = Day11(stones);
        
        Ok(self.values().sum::<usize>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ornaments::Part;

    use rstest::rstest;

    #[rstest]
    #[case(("125 17", 1), "253000 1 7")]
    #[case(("125 17", 2), "253 0 2024 14168")]
    #[case(("125 17", 3), "512072 1 20 24 28676032")]
    #[case(("125 17", 4), "512 72 2024 2 0 2 4 2867 6032")]
    #[case(("125 17", 5), "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32")]
    #[case(("125 17", 6), "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2")]
    // #[case(("125 17", 75), "...")]
    fn test_cases(#[case] input: (&str, usize), #[case] expected: &str) {
        assert_eq!(expected, Day11::blink_again(input.0, input.1));
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", Day11::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part1_recursion() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", Day11::blink_again(input, 25).split_whitespace().count().to_string());
        Ok(())
    }

    #[test]
    fn test_part2a() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("65601038650482", Day11::parse(input).solve(Part::Two)?);
        Ok(())
    }

    #[test]
    fn test_part2b() -> miette::Result<()> {
        let input = "3935565 31753 437818 7697 5 38 0 123";
        assert_eq!("244782991106220", Day11::parse(input).solve(Part::Two)?);
        Ok(())
    }
}