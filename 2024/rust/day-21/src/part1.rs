use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash};

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

    let dir_keypad = ",^,A
<,v,>";

    let num_grid = Grid(numeric_keypad.lines()
        .map(|line| line.split(',')
            // .map(
            //     |num| num.parse::<u8>().unwrap_or_else(|_| u8::MAX)
            // )
        .collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>());

    // dbg!(&num_grid);

    let dir_grid = Grid(dir_keypad.lines()
        .map(|line| line.split(',')
        .collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>());

    // dbg!(&dir_grid);

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

    // dbg!(&char_positions);

    let arrow_positions: HashMap<char, Position> = dir_grid.0
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

    // dbg!(&arrow_positions);

    let mut char_neighbors = HashMap::new();

    let mut dir_neighbors = HashMap::new();

    num_grid.walk(|pos| {
        let char = num_grid.get_at_unbounded(pos);
        // dbg!(pos, &char);

        if !char.is_empty() {
            let n = DIRECTIONS.iter()
                .map(|dir| pos + dir)
                .filter_map(|pos| {
                    let c = num_grid.get_at(pos);

                    // dbg!(&c);

                    if c.is_some_and(|c| c.is_empty()) {
                        None
                    } else {
                        c
                    }

                    // num_grid.get_at(pos)

                    // if let Some(num) = num_grid.get_at(pos) {
                    //     Some((num, pos))
                    // } else {
                    //     None
                    // }
                })
                .collect::<Vec<_>>();

            char_neighbors.insert(char.to_owned(), n);
        }

        // for neighbor in DIRECTIONS {
        //     let peek = pos + neighbor;
        //     if let Some(num) = num_grid.get_at(peek) {
        //         dbg!(peek, num);
        //     }
        // }
    });

    // dbg!(&char_neighbors);

    dir_grid.walk(|pos| {
        let char = dir_grid.get_at_unbounded(pos);
        // dbg!(pos, &char);

        if !char.is_empty() {
            let n = DIRECTIONS.iter()
                .map(|dir| pos + dir)
                .filter_map(|pos| {
                    let c = dir_grid.get_at(pos);

                    // dbg!(&c);

                    if c.is_some_and(|c| c.is_empty()) {
                        None
                    } else {
                        c
                    }
                })
                .collect::<Vec<_>>();

            dir_neighbors.insert(char.to_owned(), n);
        }
    });

    // dbg!(&dir_neighbors);

    let output = input
        .lines()
        .map(|code| {
            let mut current = 'A';
            let mut number_path = Vec::new();
            
            for target in code.chars() {
                if let Some(path) = find_shortest_path(current, target, &char_positions, &char_neighbors) {
                    number_path.extend(path);
                    number_path.push('A'); // Add button press
                    current = target;
                }
            }
            dbg!(number_path)
        })
        .map(|numeric_path| {
            let mut dir_current = 'A';
            let mut dir_path = Vec::new();
            
            for command in numeric_path {
                if let Some(path) = find_shortest_path(dir_current, command, &arrow_positions, &dir_neighbors) {
                    dir_path.extend(path);
                    dir_path.push('A');
                    dir_current = command;
                }
            }
            dbg!(dir_path)
        })
        .collect::<Vec<_>>();

    // dbg!(&output);

    Ok(output.len().to_string())
}


fn find_shortest_path(
    from: char,
    to: char,
    char_positions: &HashMap<char, Position>,
    neighbors: &HashMap<String, Vec<&str>>
) -> Option<Vec<char>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    // (current_char, path_so_far)
    queue.push_back((from, Vec::new()));
    visited.insert(from);

    while let Some((current, path)) = queue.pop_front() {
        // Found target
        if current == to {
            return Some(path);
        }

        // Get neighbors of current position
        if let Some(next_positions) = neighbors.get(&current.to_string()) {
            for next in next_positions {
                let next_char = next.chars().next().expect("Empty string");
                if !visited.insert(next_char) {
                    continue;
                }

                // Calculate direction to add to path
                let curr_pos = char_positions.get(&current).unwrap();
                let next_pos = char_positions.get(&next_char).unwrap();
                let direction = get_direction(*curr_pos, *next_pos);

                let mut new_path = path.clone();
                new_path.push(direction);
                
                queue.push_back((next_char, new_path));
            }
        }
    }
    None
}

fn get_direction(from: Position, to: Position) -> char {
    let diff = to - from;
    match (diff.x, diff.y) {
        (0, -1) => '^',
        (0, 1) => 'v',
        (-1, 0) => '<',
        (1, 0) => '>',
        _ => panic!("Invalid move")
    }
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
