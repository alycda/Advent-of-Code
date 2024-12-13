use std::str::Lines;

use ornaments::{Solution, AocError};

#[derive(Debug, Clone)]
pub struct Day2<'a>(Lines<'a>);

impl<'a> Day2<'a> {
    fn get(&self) -> Lines<'a> {
        self.0.clone()
    }

    fn line_to_nums(line: &str) -> Vec<i32> {
        line.split_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    }

    fn is_safe(nums: Vec<i32>) -> bool {
        if nums.len() < 2 { return false; }
        
        let is_ascending = nums[0] < nums[1];
        let mut prev = nums[0];
        
        nums.into_iter()
            .skip(1)
            .all(|n| {
                let valid = if is_ascending {
                    n > prev && (n - prev).abs() <= 3
                } else {
                    n < prev && (n - prev).abs() <= 3
                };
                prev = n;
                valid
            })
    }
}

impl<'a> Solution for Day2<'a> {
    type Output = usize;
    type Item = &'a str;

    fn parse(input: &'static str) -> Self {
        Day2(input.lines())
    }

    fn part1(&mut self) -> Result<Self::Output, AocError> {
        // let output = self.0.clone()
        let output = self.get()
            .filter(|line: &&str| Day2::is_safe(Day2::line_to_nums(line)))
            .count();

        Ok(output)
    }

    fn part2(&mut self) -> Result<Self::Output, AocError> {
        let (safe, notsafe): (Vec<_>, Vec<_>) = self.get()
            .enumerate()
            .partition(|(_idx, line)| Day2::is_safe(Day2::line_to_nums(line)));

        let dampened = notsafe
            .iter()
            .filter(|(_idx, line)| {
                let nums = Day2::line_to_nums(line);
                // Check if removing any single number makes the sequence safe
                nums.iter().enumerate().any(|(skip_idx, _)| {
                    Day2::is_safe(
                        nums.iter()
                            .enumerate()
                            .filter(|(i, _)| *i != skip_idx)
                            .map(|(_, &n)| n)
                            .collect(),
                    )
                })
            })
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