use std::collections::HashMap;

use crate::AocError;

pub type Position = glam::IVec2;

/// Up, Right, Down, Left
pub const DIRECTIONS: [Position; 4] = [Position::NEG_Y, Position::X, Position::Y, Position::NEG_X];

/// stores all chars, not recommended for NUMBERS (u8 vs char)
#[derive(Debug)]
pub struct Grid<T>(Vec<Vec<T>>);

// #[derive(Debug)]
// struct Grid {
//     // HashMap mapping coordinates to heights
//     heights: HashMap<IVec2, u32>,
//     // Store dimensions for convenience
//     rows: usize,
//     cols: usize,
// }

impl<T> std::ops::Deref for Grid<T> {
    type Target = Vec<Vec<T>>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: std::fmt::Debug + Copy + PartialEq> Grid<T> {
    pub fn get_width(&self) -> usize {
        self[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.len()
    }

    /// Walks the grid from top-left to bottom-right
    pub fn walk<F: FnMut(Position) -> O, O>(&self, mut see: F) {
        for row in 0..self.get_height() {
            for col in 0..self.get_width() {
                let pos = Position::new(col as i32, row as i32);

                see(pos);
            }
        }
    }

    pub fn get_at_unbounded(&self, pos: Position) -> T {
        self[pos.y as usize][pos.x as usize]
    }

    /// Bounded by the grid's dimensions
    pub fn get_at(&self, pos: Position) -> Option<T> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.get_width() as i32 || pos.y >= self.get_height() as i32 {
            return None;
        }

        Some(self[pos.y as usize][pos.x as usize])
        // Some(self.get_at_unbounded(pos))
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let numeric_keypad = "7,8,9
4,5,6
1,2,3
,0,A";

    let num_grid = Grid(numeric_keypad.lines()
        .map(|line| line.split(',')
            // .map(
            //     |num| num.parse::<u8>().unwrap_or_else(|_| u8::MAX)
            // )
        .collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>());

    // dbg!(&num_grid);

    let char_positions: HashMap<char, Position> = num_grid.0
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, cell)| {
                if cell.is_empty() {
                    None
                } else {
                    Some((cell.chars().next().unwrap(), Position::new(x as i32, y as i32)))
                }
            })
        })
        .collect();

    dbg!(&char_positions);

    let mut a = HashMap::new();

    num_grid.walk(|pos| {
        let char = num_grid.get_at_unbounded(pos);
        // dbg!(pos, &char);

        let neighbors = DIRECTIONS.iter()
            .map(|dir| pos + dir)
            .filter_map(|pos| {
                num_grid.get_at(pos)

                // if let Some(num) = num_grid.get_at(pos) {
                //     Some((num, pos))
                // } else {
                //     None
                // }
            })
            .collect::<Vec<_>>();

        a.insert(char, neighbors);

        // for neighbor in DIRECTIONS {
        //     let peek = pos + neighbor;
        //     if let Some(num) = num_grid.get_at(peek) {
        //         dbg!(peek, num);
        //     }
        // }
    });

    dbg!(&a);

    let output = input.lines().count();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // use rstest::rstest;

    // #[rstest]
    // #[case("", "")]
    // fn test_cases(#[case] input: &str, #[case] expected: &str) {
    //     assert_eq!(process(input).unwrap(), expected);
    // }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "029A
980A
179A
456A
379A";
        assert_eq!("126384", process(input)?);
        Ok(())
    }
}
