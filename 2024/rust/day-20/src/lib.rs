use std::collections::HashMap;

use ornaments::{manhattan_distance, AocError, Position, Solution, DIRECTIONS};

const TARGET_PICOSECONDS:i32 = 100;

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
        let end = grid.to_position(input.find("E").unwrap());
        let _maze = grid.to_maze('#');
        let mut path = grid.to_maze('.');

        // there is only one path and this is the length (including E, but not S)
        // dbg!(path.0.len() + 1);

        // we must insert start and end position or we will be off by 2^y
        path.insert(start);
        path.insert(end);

        // // Create distance map using BFS
        // let distances: HashMap<Position, i32> = {
        //     let mut distances = HashMap::new();
        //     let mut queue = std::collections::VecDeque::new();
            
        //     // Start BFS from start position
        //     queue.push_back((start, 0));
        //     distances.insert(start, 0);
            
        //     while let Some((current, dist)) = queue.pop_front() {
        //         for dir in DIRECTIONS {
        //             let next = current + dir;
        //             // Only explore positions that are in our valid path set
        //             if path.contains(&next) && !distances.contains_key(&next) {
        //                 distances.insert(next, dist + 1);
        //                 queue.push_back((next, dist + 1));
        //             }
        //         }
        //     }
            
        //     distances
        // };

        // let path_positions: std::collections::HashSet<Position> = grid.to_maze('.').0;

        // Successors function now only considers positions in our known path
        let successors = |pos: &Position| {
            DIRECTIONS.iter()
                .map(|&dir| pos + dir)
                .filter(|next| path.contains(next))
                .collect::<Vec<_>>()
        };

        let distances: HashMap<Position, i32> = pathfinding::prelude::bfs_reach(start, successors)
            .enumerate()
            .map(|(steps, pos)| (pos, steps as i32))
            .collect();

        Self(distances)
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        let mut count = 0;
        for (pos, steps) in &self.0 {
            for dir in DIRECTIONS {
                let wall_pos = pos + dir;
                let two_away = pos + dir * 2;
                
                if !self.contains_key(&wall_pos) && 
                self.contains_key(&two_away) && 
                self[&two_away] - steps >= TARGET_PICOSECONDS + 2 {
                    count += 1;
                }
            }
        }

        // add start and end position
        Ok(count)
    }

    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        let mut count = 0;

        for (&p, &p_dist) in &self.0 {
            for (&q, &q_dist) in &self.0 {
                let d = manhattan_distance(p, q);
                // For part 2: d < 21 instead of d == 2
                if d < 21 && p_dist - q_dist - d >= TARGET_PICOSECONDS {
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
        assert_eq!("0", Day::parse(input).solve(Part::Two)?);
        Ok(())
    }
}