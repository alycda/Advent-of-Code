use std::{collections::HashSet, hash::Hash};

use glam::IVec2;

use crate::custom_error::AocError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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

    fn turn_right(&self) -> Self {
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
        let col = idx % cols;
        let row = idx / cols;
        IVec2::new(col as i32, row as i32)
    }

    fn get_cols(&self) -> usize {
        self.0 + 1
    }

    fn get_rows(&self) -> usize {
        self.1
    }

    fn get_bounds(&self) -> ((i32, i32), (i32, i32)) {
        let max_x = self.get_cols() as i32 - 1;
        let max_y = self.get_rows() as i32 - 1;
        ((0, max_x), (0, max_y))
    }

    fn place_obstacles(&self, start_position: IVec2, existing_walls: &HashSet<IVec2>) -> usize {
        let mut valid_positions = HashSet::new();
        let mut pos = start_position;
        let mut dir = Direction::Up;
        let bounds = self.get_bounds();
        
        // First get all valid guard positions
        loop {
            let next_pos = pos + dir.to_position();
            if existing_walls.contains(&next_pos) {
                dir = dir.turn_right();
            } else if (bounds.0.0..=bounds.0.1).contains(&next_pos.x) 
                && (bounds.1.0..=bounds.1.1).contains(&next_pos.y) {
                pos = next_pos;
                valid_positions.insert(pos);
            } else {
                break;
            }
        }
    
        // Then check each valid position for cycles when used as a new wall
        valid_positions.iter()
            .filter(|pos| {
                let mut new_walls = existing_walls.clone();
                new_walls.insert(**pos);
                sentry(start_position, Direction::Up, &new_walls, pos, bounds)
            })
            .count()
    }
}

fn sentry(
    mut position: IVec2,
    mut direction: Direction,
    walls: &HashSet<IVec2>,
    new_wall: &IVec2,
    bounds: ((i32, i32), (i32, i32)),
) -> bool {
    let mut visited: HashSet<(IVec2, Direction)> = HashSet::new();
    let (x_minmax, y_minmax) = bounds;

    loop {
        if !visited.insert((position, direction)) {
            return true;
        }

        let next_position = position + direction.to_position();

        if walls.contains(&next_position) || next_position == *new_wall {
            direction = direction.turn_right();
        } else if (x_minmax.0..=x_minmax.1).contains(&next_position.x)
            && (y_minmax.0..=y_minmax.1).contains(&next_position.y)
        {
            position = next_position;
        } else {
            return false;
        }
    }
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut peekable = input.lines().peekable();
    let cols = peekable.peek().unwrap().chars().count();
    let rows = peekable.count();

    let grid = Grid::new(cols, rows);
    let start = input.find("^").unwrap();
    let walls = input.match_indices("#")
        .map(|(idx, _)| grid.to_position(idx))
        .collect::<HashSet<_>>();

    let start_position = grid.to_position(start);
    let direction = Direction::Up;

    sentry(start_position, direction, &walls, &start_position, grid.get_bounds());

    let output = grid.place_obstacles(start_position, &walls);

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