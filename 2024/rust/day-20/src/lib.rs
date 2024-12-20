use std::collections::HashMap;

use ornaments::{AocError, Position, Solution, Something, DIRECTIONS};

const TARGET_PICOSECONDS:i32 = 100;

pub mod part2;

pub use crate::Day20 as Day;

pub struct Day20(HashMap<Position, i32>);

impl std::ops::Deref for Day20 {
    type Target = HashMap<Position, i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl Iterator for Day20 {
//     type Item = (Position, i32);

//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.iter().next().map(|(k, v)| (*k, *v))
//     }
// }

/// Consume
impl IntoIterator for Day {
    type Item = (Position, i32);
    type IntoIter = std::collections::hash_map::IntoIter<Position, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Reference
impl<'a> IntoIterator for &'a Day {
    type Item = (&'a Position, &'a i32);
    type IntoIter = std::collections::hash_map::Iter<'a, Position, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Day20 {}

impl Solution for Day {
    type Output = i32;
    type Item = ();

    fn parse(input: &str) -> Self {
        let grid = Day::to_grid(input);
        let start = grid.to_position(input.find("S").unwrap());
        let _end = grid.to_position(input.find("E").unwrap());
        let maze = grid.to_maze('#');

        let rows = grid.get_height();
        let cols = grid.get_width();

        // let mut track = Something::<i32>();
        let mut track = std::collections::HashMap::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((start, 0));
        track.insert(start, 0);

        while let Some((pos, steps)) = queue.pop_front() {
            for dir in DIRECTIONS {
                let next = pos + dir;
                if next.x >= 0 && next.x < cols as i32 && 
                   next.y >= 0 && next.y < rows as i32 &&
                   !track.contains_key(&next) && !maze.contains(&next) {
                    track.insert(next, steps + 1);
                    queue.push_back((next, steps + 1));
                }
            }
        }

        Self(track)
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        let mut count = 0;
        for (&pos, &steps) in &self.0 {
            for dir in DIRECTIONS {
                let wall_pos = pos + dir;
                let two_away = pos + dir * 2;
                
                if !self.contains_key(&wall_pos) && 
                self.contains_key(&two_away) && 
                self[&two_away] - steps >= TARGET_PICOSECONDS as i32 + 2 {
                    count += 1;
                }
            }
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!("0", Day::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "";
        assert_eq!("0", Day::parse(input).solve(Part::Two)?);
        Ok(())
    }
}