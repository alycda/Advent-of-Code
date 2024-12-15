use std::collections::HashSet;
use glam::IVec2;
use ornaments::{AocError, Direction, PhantomGrid, Solution};

#[derive(Debug)]
// pub struct Day6(PhantomGrid, IVec2);
pub struct Day6 {
    walls: PhantomGrid,
    start_pos: IVec2,
}

impl Day6 {
    fn follow_path(&self, start: IVec2, extra_wall: Option<IVec2>) -> HashSet<IVec2> {
        let mut pos = start;
        let mut dir = Direction::Up;
        let mut visited = HashSet::from([start]);
        
        loop {
            let next_pos = pos + dir.to_offset();
            
            if self.walls.contains(&next_pos) || extra_wall.map_or(false, |w| w == next_pos) {
                dir = dir.turn_right();
            } else if self.walls.in_bounds(next_pos) {
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
        
        while self.walls.in_bounds(pos) {
            if !visited.insert((pos, dir)) {
                return true; // Found a cycle
            }
            
            let next_pos = pos + dir.to_offset();
            
            if self.walls.contains(&next_pos) || next_pos == wall_pos {
                dir = dir.turn_right();
            } else if self.walls.in_bounds(next_pos) {
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

    fn parse(input: &str) -> Self {
        let mut walls = HashSet::new();
        let mut start_pos = None;
        let mut max_x = 0;
        let mut max_y = 0;
        
        for (y, line) in input.lines().enumerate() {
            max_y = y;

            for (x, c) in line.chars().enumerate() {
                max_x = x;

                let pos = IVec2::new(x as i32, y as i32);
                match c {
                    '#' => { walls.insert(pos); }
                    '^' => { start_pos = Some(pos); }
                    _ => {}
                }
            }
        }

        let bounds = IVec2::new(max_x as i32, max_y as i32);
        let bounds = (IVec2::ZERO, bounds);
        let walls = PhantomGrid(walls, bounds);
        
        let start_pos = start_pos.expect("Missing start position");
        Self { walls, start_pos }
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