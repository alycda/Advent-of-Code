// pub struct Day7(usize, Vec<usize>);

// impl Solution for Day7 {
//     type Output = usize;
//     type Item = Vec<usize>;

//     fn parse(input: &'static str) -> Self {
//         let _ = input.lines();

//         todo!()
//     }
// }

use ornaments::{AocError, Solution};

// Core types and traits
#[derive(Debug)]
pub struct Expression {
    target: usize,
    numbers: Vec<usize>,
}

pub trait Evaluator {
    fn evaluate(&self, current: usize, nums: &[usize], target: usize) -> Option<usize>;
}

// Part 1 implementation
pub struct BasicEvaluator;

impl Evaluator for BasicEvaluator {
    fn evaluate(&self, current: usize, nums: &[usize], target: usize) -> Option<usize> {
        if current > target {
            return None;
        }
        
        if nums.is_empty() {
            return if current == target { Some(current) } else { None };
        }

        let n = nums[0];
        let rest = &nums[1..];

        self.evaluate(current * n, rest, target)
            .or_else(|| self.evaluate(current + n, rest, target))
    }
}

// Part 2 implementation
pub struct AdvancedEvaluator;

impl Evaluator for AdvancedEvaluator {
    fn evaluate(&self, current: usize, nums: &[usize], target: usize) -> Option<usize> {
        if current > target {
            return None;
        }
        
        if nums.is_empty() {
            return if current == target { Some(current) } else { None };
        }

        let n = nums[0];
        let rest = &nums[1..];
        let concatenation = format!("{}{}", current, n).parse::<usize>().unwrap();

        self.evaluate(concatenation, rest, target)
            .or_else(|| self.evaluate(current * n, rest, target))
            .or_else(|| self.evaluate(current + n, rest, target))
    }
}

// Main solution struct
pub struct Day7<E: Evaluator> {
    expressions: Vec<Expression>,
    evaluator: E,
}

impl<E: Evaluator + Default> Solution for Day7<E> {
    type Output = usize;
    type Item = Vec<Expression>;

    fn parse(input: &'static str) -> Self 
    where 
        Self: Sized 
    {
        let expressions = input
            .lines()
            .filter_map(|line| {
                let (target, nums) = line.split_once(": ")?;
                let target = target.parse().ok()?;
                let numbers = nums
                    .split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect();
                
                Some(Expression { target, numbers })
            })
            .collect();

        Self {
            expressions,
            evaluator: E::default(),
        }
    }

    fn part1(&mut self) -> Result<Self::Output, AocError> {
        let sum = self.expressions
            .iter()
            .filter_map(|expr| {
                self.evaluator.evaluate(
                    expr.numbers[0], 
                    &expr.numbers[1..], 
                    expr.target
                )
            })
            .sum();

        Ok(sum)
    }

    fn part2(&mut self) -> Result<Self::Output, AocError> {
        // For Day7<BasicEvaluator> this would never be called
        // For Day7<AdvancedEvaluator> this uses the same logic as part1
        self.part1()
    }
}

// Make evaluators implement Default
impl Default for BasicEvaluator {
    fn default() -> Self {
        Self
    }
}

impl Default for AdvancedEvaluator {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("3749", Day7::<BasicEvaluator>::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("11387", Day7::<AdvancedEvaluator>::parse(input).solve(Part::Two)?);
        Ok(())
    }
}