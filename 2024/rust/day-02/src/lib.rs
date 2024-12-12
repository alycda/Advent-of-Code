use std::{cmp::Ordering, str::Lines};

use ornaments::{Solution, AocError};

#[derive(Debug, Clone)]
pub struct Day2<'a>(Lines<'a>);

impl<'a> Day2<'a> {
    fn get(&self) -> Lines<'a> {
        self.0.clone()
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
            .filter(|line: &&str| is_safe(line_to_nums(line)))
            .count();

        Ok(output)
    }

    fn part2(&mut self) -> Result<Self::Output, AocError> {
        let (safe, notsafe): (Vec<_>, Vec<_>) = self.get()
            .enumerate()
            .partition(|(_idx, line)| is_safe(line_to_nums(line)));

        let dampened = notsafe
            .iter()
            .filter(|(_idx, line)| {
                let nums = line_to_nums(line);
                // Check if removing any single number makes the sequence safe
                nums.iter().enumerate().any(|(skip_idx, _)| {
                    is_safe(
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

#[deprecated]
fn line_to_nums(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

/// The numbers are either all increasing or all decreasing.
/// Any two adjacent numbers differ by at least one and at most three.
#[deprecated]
fn is_safe(nums: Vec<i32>) -> bool {
    let mut safe = false;

    let nums2 = nums.clone();
    let mut last = 0;
    let mut direction = ' ';

    for n in nums {
        match direction {
            '+' => {
                if n > last {
                    // dbg!((last - n).abs());
                    match (last - n).abs() {
                        1..=3 => {
                            safe = true;
                        }
                        _ => {
                            safe = false;
                            break;
                        }
                    }
                } else {
                    safe = false;
                    break;
                }
            }
            '-' => {
                if n < last {
                    match (n - last).abs() {
                        1..=3 => {
                            safe = true;
                        }
                        _ => {
                            safe = false;
                            break;
                        }
                    }
                } else {
                    safe = false;
                    break;
                }
            }
            // first pass
            _ => match n.cmp(&nums2[1]) {
                Ordering::Greater => {
                    direction = '-';
                }
                Ordering::Less => {
                    direction = '+';
                }
                Ordering::Equal => {
                    safe = false;
                    break;
                }
            },
        }
        last = n;
    }

    safe
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