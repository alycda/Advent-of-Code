use std::collections::HashMap;
use glam::IVec2;

use ornaments::Solution;

pub mod part1;
pub mod part2;

pub struct Day10(HashMap<IVec2, u32>);

impl std::ops::Deref for Day10 {
    type Target = HashMap<IVec2, u32>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Solution for Day10 {
    type Output = usize;
    type Item = ();

    fn parse(input: &'static str) -> Self {
        let mut heights = HashMap::new();
        let grid = Day10::to_grid(input);
        // let grid = Day10::to_grid(input, Some(|c: char| c.to_digit(10).map(|d| d as usize)));

        grid.walk(|pos| {
            let height = grid.get_at_unbounded(pos).to_digit(10).unwrap_or(0);
            heights.insert(pos, height as u32);
        });

        Self(heights)
    }

    fn part1(&mut self) -> miette::Result<Self::Output, ornaments::AocError> {
        Ok(0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("36", Day10::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    assert_eq!("81", Day10::parse(input).solve(Part::Two)?);
        Ok(())
    }
}