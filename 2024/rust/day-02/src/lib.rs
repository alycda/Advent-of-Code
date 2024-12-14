use ornaments::{Solution, AocError};

#[derive(Debug)]
pub struct Sequence(Vec<i32>);

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

pub struct Day2 {
    sequences: Vec<Sequence>
}

impl Solution for Day2 {
    type Output = usize;
    type Item = Vec<Sequence>;

    fn parse(input: &'static str) -> Self {
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

        Self { sequences }
    }

    fn part1(&mut self) -> Result<Self::Output, AocError> {
        Ok(self.sequences.iter()
            .filter(|seq| seq.is_safe())
            .count())
    }

    fn part2(&mut self) -> Result<Self::Output, AocError> {
        let (safe, notsafe): (Vec<_>, Vec<_>) = self.sequences.iter()
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