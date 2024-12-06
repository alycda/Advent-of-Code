use std::collections::HashMap;

use tracing::instrument;

use crate::custom_error::AocError;

// Represents an ABSOLUTE position in the grid (COL, ROW)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize, 
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn get_col(&self) -> usize {
        self.x
    }

    fn get_row(&self) -> usize {
        self.y
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up(Position, Option<char>), // A
    Down(Position, Option<char>), // X
    Left(Position, Option<char>), // #
    Right(Position, Option<char>), // O
    // TopLeft(Position, Option<char>), // A#
    // TopRight(Position, Option<char>), // AO
    // BottomLeft(Position, Option<char>), // X#
    // BottomRight(Position, Option<char>), // XO
}

/// COL, ROW
#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid(usize, usize, String);

impl Grid {
    fn new(cols: usize, rows: usize, grid: String) -> Self {
        Self(cols, rows, grid)
    }

    #[instrument]
    fn to_position(&self, idx: usize) -> Position {
        // panic!("{idx}");

        let cols = self.get_cols();
        // let chars_per_row = cols + 1;
        let col = idx % cols;
        let row = idx / cols;
        dbg!(Position::new(col, row))
    }

    // chars per row is cols + 1
    fn get_cols(&self) -> usize {
        self.0 + 1
    }

    fn get_rows(&self) -> usize {
        self.1
    }

    /// accounts for newline characters
    #[instrument]
    fn to_idx(&self, pos: Position) -> usize {
        // let chars_per_row = self.get_cols() + 1;

        self.get_cols() * pos.get_row() + pos.get_col()
    }

    #[instrument]
    fn get_char(&self, pos: Position) -> char {
        self.2.as_bytes()[dbg!(self.to_idx(pos))] as char
    }

    fn insert(&mut self, c: char, pos: Position) {
        self.2.remove(self.to_idx(pos));
        self.2.insert(self.to_idx(pos), c);
    }

//     fn _get_all_neighbors() {
//         todo!()
//     }

    fn get_neighbors(&self, pos: Position) -> HashMap<&str, (Position, char)> {
        let mut neighbors = Vec::new();
        let cols = self.get_cols();
        // let chars_per_row = cols + 1;
        let rows = self.get_rows();
        let col = pos.get_col();
        let row = pos.get_row();
        // let char = dbg!(self.get_char(pos));

        // Check up (decrease Y)
        if row > 0 {
            let neighbor = Position::new(col, row - 1);
            neighbors.push(Direction::Up(neighbor, Some(self.get_char(neighbor))));
        }
        // Check down (increase Y)
        if row + 1 < rows {
            let neighbor = Position::new(col, row + 1);
            neighbors.push(Direction::Down(neighbor, Some(self.get_char(neighbor))));
        }
        // Check left (decrease X)
        if col > 0 {
            let neighbor = Position::new(col - 1, row);
            neighbors.push(Direction::Left(neighbor, Some(self.get_char(neighbor))));
        }
        // Check right (increase X)
        if col + 1 < cols {
            let neighbor = Position::new(col + 1, row);
            neighbors.push(Direction::Right(neighbor, Some(self.get_char(neighbor))));
        }
        neighbors.iter().fold(HashMap::new(), |mut map, dir| {
            match dir {
                Direction::Up(new_pos, char) => { map.insert("Up", (*new_pos, char.unwrap_or('?'))); }
                Direction::Right(new_pos, char) => { map.insert("Right", (*new_pos, char.unwrap_or('?'))); }
                Direction::Down(new_pos, char) => { map.insert("Down", (*new_pos, char.unwrap_or('?'))); }
                Direction::Left(new_pos, char) => { map.insert("Left", (*new_pos, char.unwrap_or('?'))); }
            }
    
            map
        })
    }

    fn print(&self) {
        self.2.lines().for_each(|l| {
            dbg!(l);
        });        
    }
    
}

// fn _move_forward(mut start_position: &Position, pos: Position, cell: char, neighbors: &mut Option<HashMap<&str, (Position, char)>>, grid: &mut Grid) -> Option<HashMap<&str, (Position, char)>> {
//     let current_pos = start_position;
//     let next_pos = pos;
//     let should_mark_x = cell != 'X';
    
//     // Drop the borrow by setting neighbors to None
//     // neighbors = None;
    
//     // Now we can modify grid
//     if should_mark_x {
//         grid.insert('X', *current_pos);
//     }
//     grid.insert('^', next_pos);
    
//     start_position = &next_pos;
//     // Get new neighbors after modification
//     Some(grid.get_neighbors(next_pos))
// }

// // fn turn_right() {

// // }

#[instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut peekable = input.lines().peekable();
    let cols = peekable.peek().unwrap().chars().count();
    let rows = peekable
        // .enumerate()
        // .inspect(|(row, line)| { 
        // dbg!(row, line); 

        // // line.chars().enumerate().inspect(|(col, c)| {
        // //     dbg!(row, col, c);
        // // }).count();
        // })
        .count();

    // dbg!(cols, rows); // (3, 4)

    let start = input.find("^");

    let mut grid = Grid::new(cols, rows, input.to_owned());
    let mut start_position = grid.to_position(start.unwrap());

    // dbg!(&start_position);

    let mut neighbors: Option<HashMap<&str, (Position, char)>> = Some(grid.get_neighbors(start_position));

    while let Some(ref n) = neighbors {
        grid.print();

//         dbg!(start_position, &n);
//         // dbg!(start_position);
//         panic!("halt");

        match grid.get_char(start_position) {
            // we start facing up
            '^' => {
                // forward
                if let Some((pos, cell)) = n.get("Up") {
                    match cell {
                        '.' | 'X' => {
                            // // set the current position to 'X'
                            // // dbg!(grid.to_idx(*pos));
                            // // set this position as '^'
                            // if *cell != 'X' { grid.insert('X', start_position) }
                            // grid.insert('^', *pos);
                            // // start_position = *pos;

                            // neighbors = Some(grid.get_neighbors(*pos));

                            // Store the values we need before modifying grid
                            let current_pos = start_position;
                            let next_pos = *pos;
                            let should_mark_x = *cell != 'X';
                            
                            // Drop the borrow by setting neighbors to None
                            // neighbors = None;
                            
                            // Now we can modify grid
                            if should_mark_x {
                                grid.insert('X', current_pos);
                            }
                            dbg!(grid.insert('^', next_pos));

                            
                            start_position = next_pos;
                            // Get new neighbors after modification
                            neighbors = Some(grid.get_neighbors(next_pos));
                        },
                        // we hit a wall, turn right (relative)
                        '#' => {
                            if let Some((pos, cell)) = n.get("Right") {
                                match cell {
                                    '.' | 'X' => {
                                        // set the current position to 'X'
                                        // dbg!(grid.to_idx(*pos));
                                        // set this position as '>'
                                        // if cell != 'X' { grid.insert('X', *start_position) }
                                        // grid.insert('>', *pos);

                                        // Store the values we need before modifying grid
                                        let current_pos = start_position;
                                        let next_pos = *pos;
                                        let should_mark_x = *cell != 'X';
                                        
                                        // Drop the borrow by setting neighbors to None
                                        // neighbors = None;
                                        
                                        // Now we can modify grid
                                        if should_mark_x {
                                            grid.insert('X', current_pos);
                                        }
                                        grid.insert('>', next_pos);
                                        
                                        start_position = next_pos;
                                        // Get new neighbors after modification
                                        neighbors = Some(grid.get_neighbors(next_pos));
                                    },

    //                                 '#' => {
    //                                     panic!("We should never have a '#' here with a '^' character");

    //                                     // relative right
    //                                     if let Some((pos, cell)) = neighbors.get("Down") {
    //                                         match cell {
    //                                             '.' | 'X' => {
    //                                                 // set the current position to 'X'
    //                                                 dbg!(grid.to_idx(*pos));
    //                                                 // set this position as 'v'
    //                                                 // if cell != 'X' { grid.insert('X', *start_position) }
    //                                                 // grid.insert('v', *pos);
    //                                             },
    //                                             '#' => {

    //                                                 // relative right
    //                                                 if let Some((pos, cell)) = neighbors.get("Left") {
    //                                                     match cell {
    //                                                         '.' | 'X' => {
    //                                                             // set the current position to 'X'
    //                                                             dbg!(grid.to_idx(*pos));
    //                                                             // set this position as '^'
    //                                                             // if cell != 'X' { grid.insert('X', *start_position) }
    //                                                             // grid.insert('<', *pos);
    //                                                         },
    //                                                         '#' => {
    //                                                             todo!()
    //                                                         },
    //                                                         _ => panic!("invalid character")
    //                                                     }
    //                                                 }
    //                                             },
    //                                             _ => todo!()
    //                                         }
    //                                     }
    //                                 },
                                    _ => todo!()
                                }
                            } else {
                                todo!("break??");
                            }
                        },
                        '\n' => {
                            dbg!(pos, cell);
                            panic!("newline");
                        }
                        unknown => {
                            dbg!(unknown);
                            panic!("unknown character");
                        }

                    }
                } else {
                    // todo!("break?");
                    break;
                }
            },
            '>' => {
                // forward
                if let Some((pos, cell)) = n.get("Right") {
                    match cell {
                        '.' | 'X' => {
                            // Store the values we need before modifying grid
                            let current_pos = start_position;
                            let next_pos = *pos;
                            let should_mark_x = *cell != 'X';
                            
                            // Drop the borrow by setting neighbors to None
                            // neighbors = None;
                            
                            // Now we can modify grid
                            if should_mark_x {
                                grid.insert('X', current_pos);
                            }
                            dbg!(grid.insert('>', next_pos));

                            
                            start_position = next_pos;
                            // Get new neighbors after modification
                            neighbors = Some(grid.get_neighbors(next_pos));
                        }
                        // we hit a wall, turn right (relative)
                        '#' => {
                            if let Some((pos, cell)) = n.get("Down") {
                                match cell {
                                    '.' | 'X' => {
                                        // Store the values we need before modifying grid
                                        let current_pos = start_position;
                                        let next_pos = *pos;
                                        let should_mark_x = *cell != 'X';
                                        
                                        // Drop the borrow by setting neighbors to None
                                        // neighbors = None;
                                        
                                        // Now we can modify grid
                                        if should_mark_x {
                                            grid.insert('X', current_pos);
                                        }
                                        grid.insert('v', next_pos);
                                        
                                        start_position = next_pos;
                                        // Get new neighbors after modification
                                        neighbors = Some(grid.get_neighbors(next_pos));
                                    },
                                    _ => todo!()
                                }
                            }
                        }
                        _ => todo!()
                    }
                } else {
                    todo!("break?");
                }
            },
            'v' => {
                if let Some((pos, cell)) = n.get("Down") {
                    match cell {
                        '.' | 'X' => {
                            // Store the values we need before modifying grid
                            let current_pos = start_position;
                            let next_pos = *pos;
                            let should_mark_x = *cell != 'X';
                            
                            // Drop the borrow by setting neighbors to None
                            // neighbors = None;
                            
                            // Now we can modify grid
                            if should_mark_x {
                                grid.insert('X', current_pos);
                            }
                            dbg!(grid.insert('v', next_pos));

                            
                            start_position = next_pos;
                            // Get new neighbors after modification
                            neighbors = Some(grid.get_neighbors(next_pos));
                        }
                        // we hit a wall, turn right (relative)
                        '#' => {
                            if let Some((pos, cell)) = n.get("Left") {
                                match cell {
                                    '.' | 'X' => {
                                        // Store the values we need before modifying grid
                                        let current_pos = start_position;
                                        let next_pos = *pos;
                                        let should_mark_x = *cell != 'X';
                                        
                                        // Drop the borrow by setting neighbors to None
                                        // neighbors = None;
                                        
                                        // Now we can modify grid
                                        if should_mark_x {
                                            grid.insert('X', current_pos);
                                        }
                                        grid.insert('<', next_pos);
                                        
                                        start_position = next_pos;
                                        // Get new neighbors after modification
                                        neighbors = Some(grid.get_neighbors(next_pos));
                                    },
                                    _ => todo!()
                                }
                            }
                        }
                        _ => todo!()
                    }
                } else {
                    todo!("break?");
                }
            },
            '<' => {
                // forward
                if let Some((pos, cell)) = n.get("Left") {
                    match cell {
                        '.' | 'X' => {
                            // Store the values we need before modifying grid
                            let current_pos = start_position;
                            let next_pos = *pos;
                            let should_mark_x = *cell != 'X';
                            
                            // Drop the borrow by setting neighbors to None
                            // neighbors = None;
                            
                            // Now we can modify grid
                            if should_mark_x {
                                grid.insert('X', current_pos);
                            }
                            dbg!(grid.insert('<', next_pos));

                            
                            start_position = next_pos;
                            // Get new neighbors after modification
                            neighbors = Some(grid.get_neighbors(next_pos));
                        }
                        // we hit a wall, turn right (relative)
                        '#' => {
                            if let Some((pos, cell)) = n.get("Up") {
                                match cell {
                                    '.' | 'X' => {
                                        // Store the values we need before modifying grid
                                        let current_pos = start_position;
                                        let next_pos = *pos;
                                        let should_mark_x = *cell != 'X';
                                        
                                        // Drop the borrow by setting neighbors to None
                                        // neighbors = None;
                                        
                                        // Now we can modify grid
                                        if should_mark_x {
                                            grid.insert('X', current_pos);
                                        }
                                        grid.insert('^', next_pos);
                                        
                                        start_position = next_pos;
                                        // Get new neighbors after modification
                                        neighbors = Some(grid.get_neighbors(next_pos));
                                    },
                                    _ => todo!()
                                }
                            }
                        }
                        _ => todo!()
                    }
                } else {
                    todo!("break?");
                }
            },
            unknown => {
                dbg!(unknown);
                panic!("invalid character")
            }
        }
    }

//     // for n in 1..rows {
//     //     grid.2.insert(n * cols, '\n');
//     // }

//     // dbg!(&grid.2);
//     // let _ = &grid.2.lines().inspect(|l| {dbg!(l);}).count();

    let output = grid.2.chars().filter(|c| *c == 'X').count();

    Ok(output.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    // use rstest::rstest;

//     #[rstest]
//     #[case("ABC
// ^..
// DEF
// GHI", "100")]
//     fn test_cases(#[case] input: &str, #[case] expected: &str) {
//         assert_eq!(process(input).unwrap(), expected);
//     }

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
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
