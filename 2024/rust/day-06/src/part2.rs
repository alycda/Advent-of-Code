use std::{collections::{HashMap, HashSet}, hash::Hash};

// use rayon::prelude::*;

use glam::IVec2;

use crate::custom_error::AocError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up, // A
    Down, // X
    Left, // #
    Right, // O
}

impl Direction {
    fn to_position(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::NEG_Y,
            Direction::Down => IVec2::Y,
            Direction::Right => IVec2::X,
            Direction::Left => IVec2::NEG_X,
        }
    }

    fn right_turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

/// COL, ROW
#[derive(Debug, PartialEq, Eq)]
struct Grid(usize, usize, Vec<(IVec2, Direction)>);

impl Grid {
    fn new(cols: usize, rows: usize) -> Self {
        Self(cols, rows, Vec::new())
    }

    fn to_position(&self, idx: usize) -> IVec2 {

        let cols = self.get_cols();
        // let chars_per_row = cols + 1;
        let col = idx % cols;
        let row = idx / cols;
        IVec2::new(col as i32, row as i32)
    }

    // chars per row is cols + 1
    fn get_cols(&self) -> usize {
        self.0 + 1
    }

    fn get_rows(&self) -> usize {
        self.1
    }

    // fn place_obstacles(&self, start_position: IVec2, existing_walls: &HashSet<IVec2>) -> usize {
    //     self.2.iter()
    //         .take(1)
    //         .filter(|(new_obstacle, _)| {

    //             let mut new_walls = existing_walls.clone();
    //             new_walls.insert(new_obstacle.clone());
    //             let ref mut new_grid = Grid::new(self.get_cols(), self.get_rows());

    //             // start position
    //             sentry(start_position, Direction::Up, new_grid, &new_walls);

    //             dbg!(&self.2, &new_grid.2);

    //             false
    //         }).count()
    // }

    fn place_obstacles(&self, start_position: IVec2, existing_walls: &HashSet<IVec2>) -> usize {
        self.2.iter()
            .filter(|(new_obstacle, _)| {
                let mut new_walls = existing_walls.clone();
                new_walls.insert(new_obstacle.clone());
                let mut new_grid = Grid::new(self.get_cols(), self.get_rows());

                // Start position
                if let Some(cycle_detected) = sentry_2(start_position, Direction::Up, &mut new_grid, &new_walls, &self.2) {
                    cycle_detected
                } else {
                    false
                }
            })
            .count()
    }

    fn insert(&mut self, pos: IVec2, cardinal: Direction) {
        // self.2.insert(pos);
        self.2.push((pos, cardinal));
    }

    fn exit(&self, pos: IVec2, dir: Direction) -> bool {
        let cols = self.get_cols();
        let rows = self.get_rows();
        let col = pos.x as usize;
        let row = pos.y as usize;

        match dir {
            Direction::Up => {
                if row == 0 {
                    return true;
                }
            },
            Direction::Down => {
                if row + 1 == rows {
                    return true;
                }
            },
            Direction::Left => {
                if col == 0 {
                    return true;
                }
            },
            Direction::Right => {
                if col + 1 == cols {
                    return true;
                }
            }
        }

        false
    }
    
}

fn sentry(mut position: IVec2, mut direction: Direction, grid: &mut Grid, walls: &HashSet<IVec2>) {
    loop {
        if grid.exit(position, direction) {
            break;
        }

        let next_position = position + direction.to_position();

        if let Some(_) = walls.get(&next_position) {
            // direction = Ivec2::rotate
            direction = direction.right_turn();            
        } else {
            position = next_position;
            grid.insert(position, direction);
        }
    }
}

fn sentry_2(mut position: IVec2, mut direction: Direction, grid: &mut Grid, walls: &HashSet<IVec2>, original_path: &[(IVec2, Direction)]) -> Option<bool> {
    let mut visited = HashSet::new();

    loop {
        if grid.exit(position, direction) {
            return None;
        }

        if !visited.insert((position, direction)) {
            return Some(true); // Found a cycle
        }

        // Check if the current position and direction match any position and direction in the original path
        if original_path.contains(&(position, direction)) {
            return Some(true); // Found a cycle
        }

        let next_position = position + direction.to_position();

        if let Some(_) = walls.get(&next_position) {
            direction = direction.right_turn();            
        } else {
            position = next_position;
            grid.insert(position, direction);
        }
    }
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut peekable = input.lines().peekable();
    let cols = peekable.peek().unwrap().chars().count();
    let rows = peekable
        .count();

    let mut grid = Grid::new(cols, rows);
    let start = input.find("^").unwrap();
    let walls = input.match_indices("#")
        // .inspect(|(idx, _)| { dbg!(idx); } )
        .map(|(idx, _)|{ grid.to_position(idx) })
        // .inspect(|pos| { dbg!(pos); })
        .collect::<HashSet<_>>();

    let mut last_position = grid.to_position(start);

//     let mut neighbors: Option<HashMap<&str, (Position, char)>> = Some(grid.get_neighbors(last_position));
    let mut direction = Direction::Up;

    // loop {
    //     if grid.exit(last_position, direction) {
    //         break;
    //     }

    //     let next_position = last_position + direction.to_position();

    //     if let Some(_) = walls.get(&next_position) {
    //         // direction = Ivec2::rotate
    //         direction = direction.right_turn();            
    //     } else {
    //         last_position = next_position;
    //         grid.insert(last_position, direction);
    //     }
    // }
    sentry(last_position, direction, &mut grid, &walls);

    dbg!(grid.2.len());

    let output = grid.place_obstacles(grid.to_position(start), &walls);

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
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
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
