use std::collections::HashSet;
use glam::IVec2;
use ornaments::{AocError, Direction, Solution};

#[derive(Debug)]
pub struct Day6 {
    walls: HashSet<IVec2>,
    start_pos: IVec2,
    bounds: (IVec2, IVec2), // (min, max)
}

impl Day6 {
    fn calc_bounds(walls: &HashSet<IVec2>) -> (IVec2, IVec2) {
        // Get the grid dimensions from actual content
        let max_x = walls.iter().map(|pos| pos.x).max().unwrap_or(0);
        let max_y = walls.iter().map(|pos| pos.y).max().unwrap_or(0);
        
        (IVec2::ZERO, IVec2::new(max_x, max_y))
    }

    fn is_in_bounds(&self, pos: IVec2) -> bool {
        pos.x >= 0 && pos.y >= 0 
            && pos.x <= self.bounds.1.x 
            && pos.y <= self.bounds.1.y
    }

    fn follow_path(&self, start: IVec2, extra_wall: Option<IVec2>) -> HashSet<IVec2> {
        let mut pos = start;
        let mut dir = Direction::Up;
        let mut visited = HashSet::from([start]);
        
        loop {
            let next_pos = pos + dir.to_offset();
            
            if self.walls.contains(&next_pos) || extra_wall.map_or(false, |w| w == next_pos) {
                dir = dir.turn_right();
            } else if self.is_in_bounds(next_pos) {
                pos = next_pos;
                visited.insert(pos);
            } else {
                break; // Path leads outside
            }
        }
        
        visited
    }

    fn creates_cycle(&self, start: IVec2, wall_pos: IVec2) -> bool {
        let mut pos = start;
        let mut dir = Direction::Up;
        let mut visited = HashSet::new();
        
        while self.is_in_bounds(pos) {
            if !visited.insert((pos, dir)) {
                return true; // Found a cycle
            }
            
            let next_pos = pos + dir.to_offset();
            
            if self.walls.contains(&next_pos) || next_pos == wall_pos {
                dir = dir.turn_right();
            } else if self.is_in_bounds(next_pos) {
                pos = next_pos;
            } else {
                return false;
            }
        }
        
        false
    }
}

impl Solution for Day6 {
    type Output = usize;
    type Item = IVec2;

    fn parse(input: &'static str) -> Self {
        let mut walls = HashSet::new();
        let mut start_pos = None;
        
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = IVec2::new(x as i32, y as i32);
                match c {
                    '#' => { walls.insert(pos); }
                    '^' => { start_pos = Some(pos); }
                    _ => {}
                }
            }
        }
        
        let start_pos = start_pos.expect("Missing start position");
        let bounds = Self::calc_bounds(&walls);
        
        Self { walls, start_pos, bounds }
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        let visited = self.follow_path(self.start_pos, None);
        Ok(visited.len())
    }

    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        // Get all positions the guard visits initially
        let initial_path = self.follow_path(self.start_pos, None);
        
        // Try each position as a new wall
        let count = initial_path.iter()
            .filter(|&pos| {
                *pos != self.start_pos 
                && !self.walls.contains(pos)
                && self.creates_cycle(self.start_pos, *pos)
            })
            .count();
            
        Ok(count)
    }
}

// use std::collections::HashSet;
// use glam::IVec2;
// use ornaments::{AocError, Direction, Solution};

// #[derive(Debug)]
// pub struct Day6 {
//     walls: HashSet<IVec2>,
//     start_pos: IVec2,
//     bounds: (IVec2, IVec2), // (min, max)
// }

// impl Day6 {
//     fn calc_bounds(walls: &HashSet<IVec2>) -> (IVec2, IVec2) {
//         let (min_x, max_x) = walls.iter()
//             .map(|pos| pos.x)
//             .fold((i32::MAX, i32::MIN), |(min, max), x| {
//                 (min.min(x), max.max(x))
//             });
            
//         let (min_y, max_y) = walls.iter()
//             .map(|pos| pos.y)
//             .fold((i32::MAX, i32::MIN), |(min, max), y| {
//                 (min.min(y), max.max(y))
//             });

//         (
//             IVec2::new(min_x.min(0), min_y.min(0)),
//             IVec2::new(max_x, max_y)
//         )
//     }

//     fn is_in_bounds(&self, pos: IVec2) -> bool {
//         (self.bounds.0.x..=self.bounds.1.x).contains(&pos.x)
//             && (self.bounds.0.y..=self.bounds.1.y).contains(&pos.y)
//     }

//     fn simulate_path(&self, start: IVec2, additional_wall: Option<IVec2>) -> Option<HashSet<IVec2>> {
//         let mut pos = start;
//         let mut dir = Direction::Up;
//         let mut visited = HashSet::from([start]);
        
//         loop {
//             let next_pos = pos + dir.to_offset();
            
//             if self.walls.contains(&next_pos) || additional_wall.map_or(false, |w| w == next_pos) {
//                 dir = dir.turn_right();
//             } else if self.is_in_bounds(next_pos) {
//                 pos = next_pos;
//                 if !visited.insert(pos) {
//                     return Some(visited); // Found a cycle
//                 }
//             } else {
//                 return None; // Path leads outside
//             }
//         }
//     }
// }

// impl Solution for Day6 {
//     type Output = usize;
//     type Item = IVec2;

//     fn parse(input: &'static str) -> Self {
//         let mut walls = HashSet::new();
//         let mut start_pos = None;
        
//         for (y, line) in input.lines().enumerate() {
//             for (x, c) in line.chars().enumerate() {
//                 let pos = IVec2::new(x as i32, y as i32);
//                 match c {
//                     '#' => { walls.insert(pos); },
//                     '^' => start_pos = Some(pos),
//                     _ => {}
//                 }
//             }
//         }
        
//         let start_pos = start_pos.expect("Should have a start position");
//         let bounds = Self::calc_bounds(&walls);
        
//         Self { walls, start_pos, bounds }
//     }

//     fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
//         Ok(self.simulate_path(self.start_pos, None)
//             .map_or(0, |visited| visited.len() - 2)) // -2 for start pos and exit
//     }

//     // fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
//     //     // Get initial path to find valid positions for new walls
//     //     let initial_path = self.simulate_path(self.start_pos, None)
//     //         .expect("Should have an initial path");
            
//     //     // Try each position in the initial path as a new wall
//     //     let valid_wall_count = initial_path.iter()
//     //         .filter(|&pos| {
//     //             *pos != self.start_pos && // Don't block start
//     //             !self.walls.contains(pos) && // Not an existing wall
//     //             self.simulate_path(self.start_pos, Some(*pos)).is_some() // Creates a cycle
//     //         })
//     //         .count();
            
//     //     Ok(valid_wall_count)
//     // }
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