use std::char;

use tracing::{debug, instrument, trace};

use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Grid(usize, usize, &'static str);
// {
//     rows: usize,
//     cols: usize,
//     cells: &str,
// }

impl Grid {
    #[instrument]
    fn to_position(&self, idx: usize) -> Position {
        let cols = self.0;

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

    #[instrument]
    fn walk(&self) -> Option<bool> {
        Some(false)
    }

    // fn get(&self, Position|usize) -> char {

    // }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// struct Walk(Direction, u8);

#[derive(Debug, PartialEq)]
enum WalkDirection {
    Forward(usize, Position, Vec<char>),
    Backward(usize, Position, Vec<char>),
    Both(Position, Vec<char>), // represents one step each way
    None, // no valid walk pattern
}

impl WalkDirection {
    #[instrument]
    fn expect_to_find(&self, chars: Vec<char>) {
        todo!()
    }
}

// Represents an ABSOLUTE position in the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position(usize, usize);
// {
//     row: usize,
//     col: usize,
// }

impl Position {

    // fn neighbor(&self, at: Direction) -> Self {
    //     match at {
    //         Direction::TopLeft(pos, _) => pos,
    //         _ => Self(0, 0)
    //     }
    // }

    // fn new(row: usize, col: usize) -> Self {
    //     Self { row, col }
    // }

    // fn row(&self) {
    //     &self.0
    // }

    // fn col(&self) {
    //     &self.1
    // }
}

/// if we have a some, we can explore in that direction,
/// if we have a none it was explored or never valid to start with? 
///     or should we use a replacement value like `.`??
/// do we need to know what the previous CHAR was?
#[derive(Debug, PartialEq)]
enum Direction {
    Up(Position, Option<char>), // A
    Down(Position, Option<char>), // X
    Left(Position, Option<char>), // #
    Right(Position, Option<char>), // O
    TopLeft(Position, Option<char>), // A#
    TopRight(Position, Option<char>), // AO
    BottomLeft(Position, Option<char>), // X#
    BottomRight(Position, Option<char>), // XO
}

impl Direction {
    #[instrument]
    fn get_char(&self) -> Option<char> {
        match self {
            Direction::Up(_, c) |
            Direction::Down(_, c) |
            Direction::Left(_, c) |
            Direction::Right(_, c) |
            Direction::TopLeft(_, c) |
            Direction::TopRight(_, c) |
            Direction::BottomLeft(_, c) |
            Direction::BottomRight(_, c) => *c
            // _ => None
        }
    }

    // #[instrument]
    // fn get_offset(&self) -> (isize, isize) {
    //     match self {
    //         Direction::TopLeft(_, _) => (-1, -1),
    //         _ => (0, 0)
    //     }
    // }

    #[instrument]
    fn _backwards(&self) -> Direction {
        match self {
            Direction::Up(pos, _) => Direction::Down(*pos, None),
            _ => todo!(),
        }
    }

    // fn get_coord(&self) -> &Position {
    //     use Direction::*;

    //     // match self {
    //     //     // Direction::_(pos, _) => pos
    //     //     _ => &self.0
    //     // }
    //     let (pos, _) = match self {
    //         Up(p, c) => (p, c),
    //         _ => unreachable!(), // Only if you're ABSOLUTELY sure
    //     };
    //     // &pos.as_mut()
    //     &pos
    // }

    // fn update(&mut self, c: char) {

    // }
}

/// Gets all valid orthogonal neighbors (up, down, left, right)
#[instrument]
fn get_orthogonal_neighbors(pos: Position, grid: Grid) -> Vec<Direction> {
    let mut neighbors = Vec::new();
    let rows = grid.0;
    let cols = grid.1;
    let row = pos.0;
    let col = pos.1;
    // let char = grid.get_char(pos);

    // Check up
    if row > 0 {
        let neighbor = Position(row - 1, col);
        neighbors.push(Direction::Up(neighbor, Some(grid.get_char(neighbor))));
    }
    // Check down
    if row + 1 < rows {
        let neighbor = Position(row + 1, col);
        neighbors.push(Direction::Down(neighbor, Some(grid.get_char(neighbor))));
    }
    // Check left
    if col > 0 {
        let neighbor = Position(row, col - 1);
        neighbors.push(Direction::Left(neighbor, Some(grid.get_char(neighbor))));
    }
    // Check right
    if col + 1 < cols {
        let neighbor = Position(row, col + 1);
        neighbors.push(Direction::Right(neighbor, Some(grid.get_char(neighbor))));
    }

    neighbors
}

/// Gets all valid diagonal neighbors
#[instrument]
fn get_diagonal_neighbors(pos: Position, grid: Grid) -> Vec<Direction> {
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

/// Gets all valid neighbors (orthogonal + diagonal)
#[instrument]
fn get_all_neighbors(pos: Position, grid: Grid) -> Vec<Direction> {
    let mut neighbors = get_orthogonal_neighbors(pos, grid);
    neighbors.extend(get_diagonal_neighbors(pos, grid));
    neighbors
}

#[instrument]
fn do_direction(direction: &Direction, from_position: Position) -> WalkDirection {
    match direction {
        Direction::Up(_pos, Some(n)) => {
        //     // match n {
        //     //     'S' => WalkDirection::Forward(2, from_position, vec!('X', 'M')),
        //     //     'A' => WalkDirection::Backward(2, from_position, vec!('X', 'M')),
        //     //     _ => WalkDirection::None
        //     // }
        }
        // _ => WalkDirection::None
        _ => {
            dbg!(direction, from_position);
            panic!("halt")
        }
    }

    // match (c, n) {
    //     // ('A', 'S') | ('M', 'X') => {
    //     //     // dbg!(c, n, "MATCH!, walk backwards 2 steps");
    //     //     // Walk(Direction::Down, 2)
    //     //     WalkDirection::Backward(2, from_position)
    //     //         // .expect_to_find(vec!('M', 'X'));
    //     //         // .expect_to_find(vec!('A', 'S'));
    //     // }
    //     ('A', 'S') => {
    //         // dbg!(c, n, "MATCH!, walk backwards 2 steps");
    //         // Walk(Direction::Down, 2)
    //         WalkDirection::Backward(2, from_position, vec!('M', 'X'))
    //             // .expect_to_find(vec!('M', 'X'));
    //             // .expect_to_find(vec!('A', 'S'));
    //     }
    //     ('M', 'X') => {
    //         // dbg!(c, n, "MATCH!, walk backwards 2 steps");
    //         // Walk(Direction::Down, 2)
    //         WalkDirection::Backward(2, from_position, vec!('A', 'S'))
    //             // .expect_to_find(vec!('M', 'X'));
    //             // .expect_to_find(vec!('A', 'S'));
    //     }
    //     // ('S', 'A') | ('X', 'M') => {
    //     //     // dbg!(c, n, "MATCH!, walk forwards 2 steps");
    //     //     // Walk(Direction::Up, 2)
    //     //     WalkDirection::Forward(2, from_position)
    //     //         // .expect_to_find(vec!('X', 'M'));
    //     //         // .expect_to_find(vec!('S', 'A'));
    //     // }
    //     ('S', 'A') => {
    //         // dbg!(c, n, "MATCH!, walk forwards 2 steps");
    //         // Walk(Direction::Up, 2)
    //         WalkDirection::Forward(2, from_position, vec!('X', 'M'))
    //             // .expect_to_find(vec!('X', 'M'));
    //             // .expect_to_find(vec!('S', 'A'));
    //     }
    //     ('X', 'M') => {
    //         // dbg!(c, n, "MATCH!, walk forwards 2 steps");
    //         // Walk(Direction::Up, 2)
    //         WalkDirection::Forward(2, from_position, vec!('S', 'A'))
    //             // .expect_to_find(vec!('X', 'M'));
    //             // .expect_to_find(vec!('S', 'A'));
    //     }
    //     // ('A', 'M') | ('M', 'A') => {
    //     //     // dbg!(c, n, "MATCH!, walk backwards AND FORWARDS 1 step each");
    //     //     // Walk(Direction::Up, 1)
    //     //     // Walk(Direction::Down, 2)
    //     //     WalkDirection::Both(from_position)
    //     //         // .expect_to_find(vec!('S', 'X'));
    //     //         // .expect_to_find(vec!('X', 'S'));
    //     // }
    //     ('A', 'M') => {
    //         // dbg!(c, n, "MATCH!, walk backwards AND FORWARDS 1 step each");
    //         // Walk(Direction::Up, 1)
    //         // Walk(Direction::Down, 2)
    //         WalkDirection::Both(from_position, vec!('S', 'X'))
    //             // .expect_to_find(vec!('S', 'X'));
    //             // .expect_to_find(vec!('X', 'S'));
    //     }
    //     ('M', 'A') => {
    //         // dbg!(c, n, "MATCH!, walk backwards AND FORWARDS 1 step each");
    //         // Walk(Direction::Up, 1)
    //         // Walk(Direction::Down, 2)
    //         WalkDirection::Both(from_position, vec!('X', 'S'))
    //             // .expect_to_find(vec!('S', 'X'));
    //             // .expect_to_find(vec!('X', 'S'));
    //     }
    //     // ('X', 'M') => {
    //     //     dbg!("MATCH!, walk forwards 2 steps");
    //     // }
    //     // ('M', 'X') => {
    //     //     dbg!("MATCH!, walk backwards 2 steps");
    //     // }
    //     _ => {
    //         // dbg!("give up in this direction");
    //         WalkDirection::None
    //     }
    // }
}

#[instrument]
pub fn process(input: &'static str) -> miette::Result<String, AocError> {
    // let mut matches = 0_u64;
    let mut peekable = input.lines().peekable();

    let cols = peekable.peek().unwrap().chars().count();
    let rows = peekable.count();
    let grid = Grid(cols, rows, input);

    // find all X, look at neighbors to determine if any direction matches?
    // or find 'A' and 'M' first as we are closer to the middle / binary search and potentially more successful?

    // starting with 'A' because it should be easier to branch outward than try to walk forwards?

    let output = input.match_indices('A')
    // dbg!(input.match_indices('X').collect::<Vec<_>>().len());
    // dbg!(input.match_indices('M').collect::<Vec<_>>().len());
    // let a = dbg!(input.match_indices('A').collect::<Vec<_>>());
    // dbg!(input.match_indices('S').collect::<Vec<_>>().len());

    // .inspect(|(idx, c)| {
    //     dbg!(idx, c)
    // })
    // flat_map or filter_map?
    .map(|(idx, c)| {
        let c:char = c.as_bytes()[0] as char;
        // dbg!(idx, c);
        debug!("CHAR {c} at: {idx}");

        // let row = idx / cols;
        // let col = idx % cols;

        // let start_position = dbg!(Position(row, col));
        let start_position = grid.to_position(idx);

        let neighbors = get_all_neighbors(start_position, grid);

        let _ = neighbors.iter().map(|neighbor|{
        //     Direction::Up(Position(0, 0), None)

            // match neighbor {
            //     Direction::Up(_pos, Some(n)) => {
            //         do_direction(c, *n)
            //     }
            //     Direction::Down(_pos, Some(n)) => {
            //         do_direction(c, *n)
            //     }
            //     Direction::Left(_pos, Some(n)) => {
            //         do_direction(c, *n)
            //     }
            //     Direction::Right(_pos, Some(n)) => {
            //         do_direction(c, *n)
            //     }
            //     _ => todo!()
            // }

            if let Some(_) = neighbor.get_char() {
            //     do_direction(neighbor, (start_position, c))
                ((start_position, c), neighbor)
            } else {
                todo!()
            }
        })
        // .filter(|walk| {
        //     match walk {
        //         WalkDirection::None => false,
        //         _ => true
        //     }
        // })
        .inspect(|walk| {
            dbg!(c, walk);
            // dbg!(grid.get_char())
        })
        .collect::<Vec<_>>();

        0
    })
    .sum::<usize>();

    // let _output = input.lines()
    // // let rows = dbg!(output.size_hint());
    // // let output = output
    // .inspect(|line| {
    //     dbg!(line);
    // }).map(|line| {
    //    0 
    // }).sum::<usize>();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("..X...
.SAMX.
.A..A.
XMAS.S
.X....", "3")]
// #[case("MMMSXXMASM
// MSAMXMSMSA
// AMXSXMAAMM
// MSAMASMSMX
// XMASAMXAMM
// XXAMMXXAMA
// SMSMSASXSS
// SAXAMASAAA
// MAMMMXMMMM
// MXMXAXMASX", "18")]
// #[case("....XXMAS.
// .SAMXMS...
// ...S..A...
// ..A.A.MS.X
// XMASAMX.MM
// X.....XA.A
// S.S.S.S.SS
// .A.A.A.A.A
// ..M.M.M.MM
// .X.X.XMASX", "18")]
    fn test_cases(#[case] input: &'static str, #[case] expected: &str) {
        assert_eq!(process(input).unwrap(), expected);
    }

    // #[test]
    // fn test_process() -> miette::Result<()> {
    //     let input = "";
    //     assert_eq!("", process(input)?);
    //     Ok(())
    // }
}
