use glam::IVec2;

use crate::custom_error::AocError;

/// COL, ROW
#[derive(Debug, PartialEq, Eq)]
struct Grid(usize, usize, String);


#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up(IVec2, char),
    Down(IVec2, char),
    Left(IVec2, char),
    Right(IVec2, char),
}

impl Grid {
    fn new(cols: usize, rows: usize, data: String) -> Self {
        Self(cols, rows, data)
    }

    // chars per row is cols + 1
    fn get_cols(&self) -> usize {
        self.0 + 1
    }

    fn get_rows(&self) -> usize {
        self.1
    }

    /// accounts for newline characters
    fn to_idx(&self, pos: &IVec2) -> usize {
        // let chars_per_row = self.get_cols() + 1;

        self.get_cols() * (pos.y) as usize + (pos.x) as usize
    }

    fn get_char(&self, pos: &IVec2) -> char {
        self.2.as_bytes()[self.to_idx(pos)] as char
    }

    fn get_neighbors(&self, pos: IVec2) -> Vec<Direction> {
        // todo: Hashmap
        let mut neighbors = Vec::new();
        let cols = self.get_cols();
        let rows = self.get_rows();
        let col = pos.x;
        let row = pos.y;

        // Check up (decrease Y)
        if row > 0 {
            let neighbor = IVec2::new(col, row - 1);
            neighbors.push(Direction::Up(neighbor, self.get_char(&neighbor)));
        }

        // Check down (increase Y)
        if row + 1 < rows as i32 {
            let neighbor = IVec2::new(col, row + 1);
            neighbors.push(Direction::Down(neighbor, self.get_char(&neighbor)));
        }

        // Check left (decrease X)
        if col > 0 {
            let neighbor = IVec2::new(col - 1, row);
            neighbors.push(Direction::Left(neighbor, self.get_char(&neighbor)));
        }

        // Check right (increase X)
        if col + 1 < cols as i32 {
            let neighbor = IVec2::new(col + 1, row);
            neighbors.push(Direction::Right(neighbor, self.get_char(&neighbor)));
        }

        neighbors
    }

    fn find_paths(&self, pos: IVec2, current_value: u32, mut path: Vec<IVec2>) -> usize {
        // Base case: we've reached 0
        if current_value == 0 {
            return 1;
        }

        // Add current position to path
        path.push(pos);
        
        // Look for the next number in sequence (current_value - 1) in neighbors
        let target = current_value - 1;
        
        self.get_neighbors(pos)
            .into_iter()
            // .filter(|direction| {
            //     // Extract the char value from the direction and convert to digit
            //     let neighbor_value = match direction {
            //         Direction::Up(_, c) |
            //         Direction::Down(_, c) |
            //         Direction::Left(_, c) |
            //         Direction::Right(_, c) => dbg!(c.to_digit(10)).unwrap()
            //     };
            //     neighbor_value == target
            // })
            .filter(|direction| {
                let neighbor_value = match direction {
                    Direction::Up(_, c) |
                    Direction::Down(_, c) |
                    Direction::Left(_, c) |
                    Direction::Right(_, c) => c.to_digit(10)
                    // {
                    //     let digit = c.to_digit(10);
                    //     if digit.is_none() {
                    //         // Using debug! macro if you have logging set up
                    //         println!("Found non-digit character: {}", c);
                    //     }
                    //     digit
                    // }
                };
                
                // Only proceed if we got a valid digit and it matches our target
                matches!(neighbor_value, Some(value) if value == target)
            })
            .map(|direction| {
                // Extract the position from the direction
                match direction {
                    Direction::Up(pos, _) |
                    Direction::Down(pos, _) |
                    Direction::Left(pos, _) |
                    Direction::Right(pos, _) => pos
                }
            })
            .map(|next_pos| {
                // Recursively find paths from this position
                self.find_paths(next_pos, target, path.clone())
            })
            .sum()
    }
}

pub fn process(input: &str) -> miette::Result<String, AocError> {    
    let mut peekable = input.lines().peekable();
    let cols = peekable.peek().unwrap().chars().count();
    let rows = peekable.count();

    let grid = Grid::new(cols, rows, input.to_string());


    let output: usize = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                // .inspect(|(col, d)| {
                //     let d = d.to_digit(10).unwrap();

                //     if d == 9 {
                //         dbg!((row, col, d));

                //         dbg!(grid.get_neighbors(IVec2::new(*col as i32, *row as i32)));
                //     }

                // })
                // .count();
                .filter(|(_, c)| *c == '9')
                .map(move |(col, _)| IVec2::new(col as i32, row as i32))
        })
        .map(|start_pos| grid.find_paths(start_pos, 9, Vec::new()))
        .sum();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

//     use rstest::rstest;

//     #[rstest]
//     #[case("0123
// 1234
// 8765
// 9876", "1")]
//     #[case("...0...
// ...1...
// ...2...
// 6543456
// 7.....7
// 8.....8
// 9.....9", "2")]
// #[case("..90..9
// ...1.98
// ...2..7
// 6543456
// 765.987
// 876....
// 987....", "4")]
// #[case("10..9..
// 2...8..
// 3...7..
// 4567654
// ...8..3
// ...9..2
// .....01", "3")]
// #[case("89010123
// 78121874
// 87430965
// 96549874
// 45678903
// 32019012
// 01329801
// 10456732", "9")]
//     fn test_cases(#[case] input: &str, #[case] expected: &str) {
//         // assert_eq!(process(input).unwrap(), expected);
//         todo!("haven't built test yet");
//     }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
