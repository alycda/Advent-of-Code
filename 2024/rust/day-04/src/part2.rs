use std::{collections::HashMap, fmt};

use tracing::instrument;

use crate::custom_error::AocError;
#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid(usize, usize, String);
// {
//     rows: usize,
//     cols: usize,
//     cells: &str,
// }

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
// {
//     row: usize,
//     col: usize,
// }

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    // Up(Position, Option<char>), // A
    // Down(Position, Option<char>), // X
    // Left(Position, Option<char>), // #
    // Right(Position, Option<char>), // O
    TopLeft(Position, Option<char>), // A#
    TopRight(Position, Option<char>), // AO
    BottomLeft(Position, Option<char>), // X#
    BottomRight(Position, Option<char>), // XO
}


impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Direction::Up(pos, c) => write!(f, "Up({},{},{:?})", pos.x, pos.y, c),
            // Direction::Down(pos, c) => write!(f, "Down({},{},{:?})", pos.x, pos.y, c),
            // Direction::Left(pos, c) => write!(f, "Left({},{},{:?})", pos.x, pos.y, c),
            // Direction::Right(pos, c) => write!(f, "Right({},{},{:?})", pos.x, pos.y, c),
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
            // Direction::Up(_, c) |
            // Direction::Down(_, c) |
            // Direction::Left(_, c) |
            // Direction::Right(_, c) |
            Direction::TopLeft(_, c) |
            Direction::TopRight(_, c) |
            Direction::BottomLeft(_, c) |
            Direction::BottomRight(_, c) => *c
            // _ => None
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
    // let static_borrow : &'static str = &input_without_pesky_newlines;

    let grid = dbg!(Grid(cols, rows, input_without_pesky_newlines));

    let output = input.replace('\n', "")
        .match_indices('A')
    .filter(|(idx, _)| {
        // dbg!(idx);

        let start_position = grid.to_position(*idx);
        // dbg!('A', idx, start_position);

        let diagonals = get_diagonal_neighbors(start_position, &grid);
        // dbg!(&diagonals);

        let d = diagonals.iter()
        // .inspect(|neighbor| {
        //     dbg!(neighbor);
        // })
        // .collect::<Vec<_>>();
        // .fold((HashMap::new() |mut neighbors, direction| {
        //     *neighbors.entry(direction.to_string()).or_insert()
        // }));
        .fold(HashMap::new(), |mut map, dir| {
            match dir {
                // d @ Direction::Up(..) => { map.insert("Up", d); }
                // d @ Direction::Down(..) => { map.insert("Down", d); }
                // d @ Direction::Left(..) => { map.insert("Left", d); }
                // d @ Direction::Right(..) => { map.insert("Right", d); }
                d @ Direction::TopLeft(_, char) => { map.insert("TopLeft", char.unwrap_or('.')); }
                d @ Direction::TopRight(_, char) => { map.insert("TopRight", char.unwrap_or('.')); }
                d @ Direction::BottomLeft(_, char) => { map.insert("BottomLeft", char.unwrap_or('.')); }
                d @ Direction::BottomRight(_, char) => { map.insert("BottomRight", char.unwrap_or('.')); }
            }
            map
        });

        // .fold(HashMap::new(), |mut neighbors, direction| {
        //     *neighbors.entry(std::mem::discriminant(direction)).or_insert(0) += 1;
        //     neighbors
        // });

        // dbg!(&d);
        // // dbg!(diagonals.get(Direction::TopLeft));
        // dbg!(&d.get("TopLeft"), &d.get("TopRight"), &d.get("BottomLeft"), &d.get("BottomRight"));

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
            // All four directions exist
            

            match (top_left, top_right, bottom_left, bottom_right) {
                ('M', 'S', 'M', 'S') | 
                ('S', 'M', 'S', 'M') | 
                ('M', 'M', 'S', 'S') |
                ('S', 'S', 'M', 'M') => true,        
                _ => {
                    dbg!(top_left, top_right, bottom_left, bottom_right);
                    false 
                }
            }
        } else {
            false
        }

        // let top_left = std::mem::discriminant(&Direction::TopLeft((), ()));
        // dbg!(diagonals.get(&top_left));

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
// #[case("..XXMAS", "1")]
// #[case("SAMXMS.", "1")]
// #[case("MS.X", "0")]
// #[case("XMASAMX.MM", "2")]
// #[case("XMAS", "1")]
// #[case("SAMX.MM", "1")]
// #[case("...XA.A", "0")]
// #[case(".X.X.XMASX", "1")]
// #[case(".X.XMAS", "1")]
// // #[case("..XXMAS
// // AMXMS..
// // .S..A..
// // A.A.MS.", "1")]
// #[case("...XXMA
// SAMXMS.
// ..S..A.
// .A.A.MS", "2")]
// #[case("X
// M
// A
// S
// A
// M
// X", "2")]
    fn test_cases(#[case] input: &'static str, #[case] expected: &str) {
        assert_eq!(process(input).unwrap(), expected);
    }


// //     #[rstest]
// //     #[case("..X...
// // .SAMX.
// // .A..A.
// // XMAS.S
// // .X....", 2)]
// // #[case("MMMSXXMASM
// // MSAMXMSMSA
// // AMXSXMAAMM
// // MSAMASMSMX
// // XMASAMXAMM
// // XXAMMXXAMA
// // SMSMSASXSS
// // SAXAMASAAA
// // MAMMMXMMMM
// // MXMXAXMASX", 5)]
// // #[case("....XXMAS.
// // .SAMXMS...
// // ...S..A...
// // ..A.A.MS.X
// // XMASAMX.MM
// // X.....XA.A
// // S.S.S.S.SS
// // .A.A.A.A.A
// // ..M.M.M.MM
// // .X.X.XMASX", 5)]
// //     fn test_horizontal(#[case] input: &'static str, #[case] expected: usize) {
// //         assert_eq!(count_horizontal(input), expected);
// //     }

// //     #[rstest]
// //     #[case("..X...
// // .SAMX.
// // .A..A.
// // XMAS.S
// // .X....", 1)]
// // #[case("MMMSXXMASM
// // MSAMXMSMSA
// // AMXSXMAAMM
// // MSAMASMSMX
// // XMASAMXAMM
// // XXAMMXXAMA
// // SMSMSASXSS
// // SAXAMASAAA
// // MAMMMXMMMM
// // MXMXAXMASX", 3)]
// // #[case("....XXMAS.
// // .SAMXMS...
// // ...S..A...
// // ..A.A.MS.X
// // XMASAMX.MM
// // X.....XA.A
// // S.S.S.S.SS
// // .A.A.A.A.A
// // ..M.M.M.MM
// // .X.X.XMASX", 3)]
// //     fn test_vertical(#[case] input: &'static str, #[case] expected: usize) {
// //         assert_eq!(count_vertical(input), expected);
// //     }

// //     #[rstest]
// //     #[case("..X...
// // .SAMX.
// // .A..A.
// // XMAS.S
// // .X....", 1)]
// // #[case("MMMSXXMASM
// // MSAMXMSMSA
// // AMXSXMAAMM
// // MSAMASMSMX
// // XMASAMXAMM
// // XXAMMXXAMA
// // SMSMSASXSS
// // SAXAMASAAA
// // MAMMMXMMMM
// // MXMXAXMASX", 5)]
// // #[case("....XXMAS.
// // .SAMXMS...
// // ...S..A...
// // ..A.A.MS.X
// // XMASAMX.MM
// // X.....XA.A
// // S.S.S.S.SS
// // .A.A.A.A.A
// // ..M.M.M.MM
// // .X.X.XMASX", 5)]
// //     fn test_diagonal_ltr(#[case] input: &'static str, #[case] expected: usize) {
// //         assert_eq!(count_diagonal_ltr(input), expected);
// //     }

// //     #[test]
// //     fn test_diagonals() {
// //         let input = "ABCDEFG
// // XABCDEF
// // YXABCDE
// // ZYXABCD";
// //         dbg!(count_diagonal_ltr(input));
// //         panic!("halt");
// //     }


}
