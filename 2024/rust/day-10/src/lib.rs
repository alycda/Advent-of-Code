//! Day 10: Hoof It

use std::collections::HashMap;

use ornaments::{Position, Solution, UniquePositions, DIRECTIONS};

pub use crate::Day10 as Day;

/// Position, height
pub struct Day10(HashMap<Position, u32>);

impl std::ops::Deref for Day10 {
    type Target = HashMap<Position, u32>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Day10 {
    // Find all positions containing 0
    fn get_trail_heads(&self) -> Vec<Position> {
        self.iter()
            .filter(|&(_, &height)| height == 0)
            .map(|(&pos, _)| pos)
            .collect()
    }

    fn get_rating_part1(&self, pos: Position, visited: &mut UniquePositions) -> usize {
        // If we've found a 9, count it only if we haven't seen it before
        if self.get(&pos) == Some(&9) {
            if visited.insert(pos) {  // returns true if this 9 wasn't in the set
                return 1;
            }
            return 0;  // we've already counted this 9
        }

        let current_height = *self.get(&pos).unwrap();
        let mut total = 0;

        // Check all four directions
        for Position { x: dx, y: dy } in DIRECTIONS {
            let next_pos = Position::new(pos.x + dx, pos.y + dy);
            
            if let Some(&height) = self.get(&next_pos) {
                if height == current_height + 1 {
                    total += self.get_rating_part1(next_pos, visited);
                }
            }
        }

        total
    }

    fn get_rating_part2(&self, pos: Position, path: &mut Vec<Position>) -> usize {
        // Base case: found a 9 (reached a peak)
        if self.get(&pos) == Some(&9) {
            return 1;  // Count this as one valid path
        }
    
        let current_height = *self.get(&pos).unwrap();
        let mut total = 0;
    
        // Try all possible next steps
        
        for Position { x: dx, y: dy } in DIRECTIONS {
            let next_pos = Position::new(pos.x + dx, pos.y + dy);
            
            // Only follow paths that increase by exactly 1
            if let Some(&height) = self.get(&next_pos) {
                if height == current_height + 1 && !path.contains(&next_pos) {
                    total += self.get_rating_part2(next_pos, path);
                }
            }
        }
    
        total
    }
}

impl Solution for Day10 {
    type Output = usize;
    type Item = ();

    fn parse(input: &str) -> Self {
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
        let mut total = 0;

        for start_pos in self.get_trail_heads() {
            let paths = self.get_rating_part1(start_pos, &mut UniquePositions::new());
            total += paths;
        }

        Ok(total)
    }

    fn part2(&mut self) -> miette::Result<Self::Output, ornaments::AocError> {
        let mut total = 0;

        for start_pos in self.get_trail_heads() {
            let paths = self.get_rating_part2(start_pos, &mut Vec::new());
            total += paths;
        }

        Ok(total)
    }

    fn print(input: &str) {
        let grid = Day10::to_grid(input);
        for x in 0..grid.get_height() {
            for y in 0..grid.get_width() {
                print!("{}", grid.get_at_unbounded(Position::new(x as i32, y as i32)));
            }
            println!();
        }
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