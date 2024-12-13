use std::{collections::HashSet, ops::DerefMut};

use glam::IVec2;

use ornaments::{AocError, Direction, Grid, Solution};

pub mod custom_error;

pub mod part2;

#[derive(Debug)]
pub struct Day6(Grid, IVec2);

impl Solution for Day6 {
    type Output = usize;
    type Item = IVec2;

    fn parse(input: &'static str) -> Self {
        let start_index = dbg!(input.find("^").unwrap());
        let grid = Day6::to_grid(input);
        let start_position = dbg!(grid.to_position(start_index));

        Self(grid, start_position)
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        let mut walls = HashSet::new();

        self.0.walk(|pos| {
            if '#' == self.0.get_at_unbounded(pos) {
                walls.insert(pos);
            }
        });

        assert!(!walls.is_empty(), "Walls should not be empty");

        let mut direction = Direction::Up;
        let mut visited = HashSet::from([self.1]);

        while (IVec2::ZERO.x..=self.0.get_width() as i32).contains(&self.1.x)
            && (IVec2::ZERO.y..=self.0.get_height() as i32).contains(&self.1.y) 
        {
            let next_pos = self.1 + direction.to_offset();
            
            if walls.contains(&next_pos) {
                direction = direction.turn_right();
            } else {
                self.1 = next_pos;
                visited.insert(self.1);
            }
        }

        Ok(visited.len() - 2)
    }
}

impl std::ops::Deref for Day6 {
    type Target = Vec<Vec<char>>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl std::ops::DerefMut for Day6 {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", Day6::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", Day6::parse(input).solve(Part::Two)?);
        Ok(())
    }
}