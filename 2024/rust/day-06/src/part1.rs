use std::collections::HashMap;

use tracing::instrument;

use crate::custom_error::AocError;

// Represents an ABSOLUTE position in the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position(usize, usize);

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

    fn insert(&mut self, c: char, pos: Position) {
        self.2.insert(self.to_idx(pos), c);
    }

    fn get_all_neighbors() {
        todo!()
    }

    fn get_neighbors(&self, pos: Position) -> HashMap<&str, (Position, char)> {
        let mut neighbors = Vec::new();
        let rows = self.0;
        let cols = self.1;
        let row = pos.0;
        let col = pos.1;
        // let char = grid.get_char(pos);
        // Check up
        if row > 0 {
            let neighbor = Position(row - 1, col);
            neighbors.push(Direction::Up(neighbor, Some(self.get_char(neighbor))));
        }
        // Check down
        if row + 1 < rows {
            let neighbor = Position(row + 1, col);
            neighbors.push(Direction::Down(neighbor, Some(self.get_char(neighbor))));
        }
        // Check left
        if col > 0 {
            let neighbor = Position(row, col - 1);
            neighbors.push(Direction::Left(neighbor, Some(self.get_char(neighbor))));
        }
        // Check right
        if col + 1 < cols {
            let neighbor = Position(row, col + 1);
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
    
}

fn move_forward() {
    // let current_pos = start_position;
    // let next_pos = *pos;
    // let should_mark_x = *cell != 'X';
    
    // // Drop the borrow by setting neighbors to None
    // // neighbors = None;
    
    // // Now we can modify grid
    // if should_mark_x {
    //     grid.insert('X', current_pos);
    // }
    // grid.insert('^', next_pos);
    
    // start_position = next_pos;
    // // Get new neighbors after modification
    // neighbors = Some(grid.get_neighbors(next_pos));
}

fn turn_right() {

}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut peekable = input.lines().peekable();
    let cols = peekable.peek().unwrap().chars().count();
    let rows = peekable.inspect(|line| { dbg!(line); } ).count();

    let input_without_pesky_newlines = input.replace('\n', "");

    // dbg!(input.find("^"));
    let start = dbg!(input_without_pesky_newlines.find("^"));

    let mut grid = Grid(cols, rows, input_without_pesky_newlines);
    let mut start_position = grid.to_position(start.unwrap());

    dbg!(&start_position);

    let c = '^'; // '>' 'v' '<'
    let mut neighbors = Some(grid.get_neighbors(start_position));

    while let Some(ref n) = neighbors {
        // dbg!(start_position, &n);
        // panic!("halt");

        // let neighbors = neighbors.unwrap();

        match c {
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
                            grid.insert('^', next_pos);
                            
                            start_position = next_pos;
                            // Get new neighbors after modification
                            neighbors = Some(grid.get_neighbors(next_pos));
                        },
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
                        _ => todo!()

                    }
                } else {
                    // todo!("break?");
                    break;
                }
            },
    //         '>' => todo!(),
    //         'v' => todo!(),
    //         '<' => todo!(),
            _ => panic!("invalid character")
        }
    }

    for n in 1..rows {
        grid.2.insert(n * cols, '\n');
    }

    // dbg!(&grid.2);
    let _ = &grid.2.lines().inspect(|l| {dbg!(l);}).count();

    let output = grid.2.chars().filter(|c| *c == 'X').count();

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
