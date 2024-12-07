use std::{collections::HashMap, fmt};

use tracing::instrument;

use crate::custom_error::AocError;

/// COLS, ROWS, GRID
#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid(usize, usize, String);

impl Grid {
    #[instrument]
    fn to_position(&self, idx: usize) -> Position {
        let cols = self.0;
        // let chars_per_row = cols + 1;
        let row = idx / cols;
        let col = idx % cols;
        Position(row, col)
    }

    #[instrument]
    fn to_idx(&self, pos: Position) -> usize {
        let cols = self.0;
        cols * pos.0 + pos.1
    }
    #[instrument]
    fn get_char(&self, pos: Position) -> char {
        self.2.as_bytes()[self.to_idx(pos)] as char
    }
}

// Represents an ABSOLUTE position in the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position(usize, usize);

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    TopLeft(Position, Option<char>), // A#
    TopRight(Position, Option<char>), // AO
    BottomLeft(Position, Option<char>), // X#
    BottomRight(Position, Option<char>), // XO
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::TopLeft(pos, c) => write!(f, "TopLeft({},{},{:?})", pos.0, pos.1, c),
            Direction::TopRight(pos, c) => write!(f, "TopRight({},{},{:?})", pos.0, pos.1, c),
            Direction::BottomLeft(pos, c) => write!(f, "BottomLeft({},{},{:?})", pos.0, pos.1, c),
            Direction::BottomRight(pos, c) => write!(f, "BottomRight({},{},{:?})", pos.0, pos.1, c),
        }
    }
}

impl Direction {
    #[instrument]
    fn get_char(&self) -> Option<char> {
        match self {
            Direction::TopLeft(_, c) |
            Direction::TopRight(_, c) |
            Direction::BottomLeft(_, c) |
            Direction::BottomRight(_, c) => *c
        }
    }
}

/// Gets all valid diagonal neighbors
#[instrument]
fn get_diagonal_neighbors(pos: Position, grid: &Grid) -> Vec<Direction> {
    let mut neighbors = Vec::new();
    let rows = grid.0;
    let cols = grid.1;
    let row = pos.0;
    let col = pos.1;
    // let char = grid.get_char(pos);
    // Up-left
    if row > 0 && col > 0 {
        let neighbor = Position(row - 1, col - 1);
        neighbors.push(Direction::TopLeft(neighbor, Some(grid.get_char(neighbor))));
    }
    // Up-right
    if row > 0 && col + 1 < cols {
        let neighbor = Position(row - 1, col + 1);
        neighbors.push(Direction::TopRight(neighbor, Some(grid.get_char(neighbor))));
    }
    // Down-left
    if row + 1 < rows && col > 0 {
        let neighbor = Position(row + 1, col - 1);
        neighbors.push(Direction::BottomLeft(neighbor, Some(grid.get_char(neighbor))));
    }
    // Down-right
    if row + 1 < rows && col + 1 < cols {
        let neighbor = Position(row + 1, col + 1);
        neighbors.push(Direction::BottomRight(neighbor, Some(grid.get_char(neighbor))));
    }
    neighbors
}

#[instrument]
pub fn process(input: &'static str) -> miette::Result<String, AocError> {
    let mut peekable = input.lines().peekable();
    let cols = peekable.peek().unwrap().chars().count();
    let rows = peekable.count();

    let input_without_pesky_newlines = input.replace('\n', "");

    let grid = Grid(cols, rows, input_without_pesky_newlines);

    let output = input.replace('\n', "")
        .match_indices('A')
    .filter(|(idx, _)| {
        let start_position = grid.to_position(*idx);

        let diagonals = get_diagonal_neighbors(start_position, &grid);

        let d = diagonals.iter()
        .fold(HashMap::new(), |mut map, dir| {
            match dir {
                Direction::TopLeft(_, char) => { map.insert("TopLeft", char.unwrap_or('.')); }
                Direction::TopRight(_, char) => { map.insert("TopRight", char.unwrap_or('.')); }
                Direction::BottomLeft(_, char) => { map.insert("BottomLeft", char.unwrap_or('.')); }
                Direction::BottomRight(_, char) => { map.insert("BottomRight", char.unwrap_or('.')); }
            }
            map
        });

        if let (
            Some(top_left), 
            Some(top_right), 
            Some(bottom_left), 
            Some(bottom_right)
        ) = (
            d.get("TopLeft"),
            d.get("TopRight"),
            d.get("BottomLeft"),
            d.get("BottomRight")
        ) {

            match (top_left, top_right, bottom_left, bottom_right) {
                ('M', 'S', 'M', 'S') | 
                ('S', 'M', 'S', 'M') | 
                ('M', 'M', 'S', 'S') |
                ('S', 'S', 'M', 'M') => true,        
                _ => {
                    false 
                }
            }
        } else {
            false
        }
    }).count();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("M.S
.A.
M.S", "1")]
    #[case(".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........", "9")]
#[case(".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M...
..........", "8")]
    fn test_cases(#[case] input: &'static str, #[case] expected: &str) {
        assert_eq!(process(input).unwrap(), expected);
    }
}
