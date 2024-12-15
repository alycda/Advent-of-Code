use ornaments::{AocError, Solution};

pub mod custom_error;

pub mod part2;

pub struct Day11(String);

impl std::ops::Deref for Day11 {
    type Target = String;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Day11 {
    /// recursion
    pub fn blink(input: &str, times: usize) -> String {
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

        Day11::blink(&new_input, times - 1)
    }
}

impl Solution for Day11 {
    type Output = usize;
    type Item = String;

    fn parse(input: &str) -> Self {
        Self(input.to_string())
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        Ok(Day11::blink(self, 25).split_whitespace().count())
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
    fn test_cases(#[case] input: (&str, usize), #[case] expected: &str) {
        assert_eq!(expected, Day11::blink(input.0, input.1));
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", Day11::parse(input).solve(Part::One)?);
        Ok(())
    }
}