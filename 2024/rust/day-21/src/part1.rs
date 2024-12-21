use std::collections::HashMap;

use crate::{get_all_paths, Position,  AocError};

/// Up, Right, Down, Left
pub const DIRECTIONS: [Position; 4] = [Position::NEG_Y, Position::X, Position::Y, Position::NEG_X];

/// stores all chars, not recommended for NUMBERS (u8 vs char)
#[derive(Debug)]
pub struct Grid<T>(Vec<Vec<T>>);

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


// type NumericKeypad = Grid<char>;

///
/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
///     | 0 | A |
///     +---+---+
/// 
fn make_numeric_keypad() -> Grid<&'static str> {
    let numeric_keypad = "7,8,9
4,5,6
1,2,3
,0,A";

    Grid(numeric_keypad.lines()
        .map(|line| line.split(',')
            // .map(
            //     |num| num.parse::<u8>().unwrap_or_else(|_| u8::MAX)
            // )
        .collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>())
}


/// 
///     +---+---+
///     | ^ | A |
/// +---+---+---+
/// | < | v | > |
/// +---+---+---+
/// 
fn make_dir_pad() -> Grid<&'static str> {
    let dir_keypad = ",^,A
<,v,>";

    Grid(dir_keypad.lines()
        .map(|line| line.split(',')
        .collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>())
}

fn to_positions(grid: &Grid<&str>) -> HashMap<char, Position> {
    grid.0
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, cell)| {
                if cell.is_empty() {
                    None
                } else {
                    Some((cell.chars().next().unwrap(), Position::new(x as i32, y as i32)))
                    // Some((cell.chars().next().unwrap(), Position::new(y as i32, x as i32)))
                }
            })
        })
        .collect()
}

fn get_neighbors(grid: Grid<&str>) -> HashMap<String, Vec<&str>> {
    let mut neighbors = HashMap::new();

    grid.walk(|pos| {
        let char = grid.get_at_unbounded(pos);

        if !char.is_empty() {
            let n = DIRECTIONS.iter()
                .map(|dir| pos + dir)
                .filter_map(|pos| {
                    let c = grid.get_at(pos);

                    if c.is_some_and(|c| c.is_empty()) {
                        None
                    } else {
                        c
                    }
                }).collect::<Vec<_>>();

            neighbors.insert(char.to_owned(), n);
        }
    });

    neighbors
}

fn number_pad(code: &str, positions: &HashMap<char, Position>, neighbors: &HashMap<String, Vec<&str>>) -> Vec<char> {
    dbg!(&code);

    let mut current = 'A';
    let mut number_path = Vec::new();
    
    for target in code.chars() {
        if let Some(path) = find_shortest_path(current, target, positions, &neighbors) {
            number_path.extend(path);
            number_path.push('A'); // Add button press
            current = target;
        }
    }
    // dbg!(&number_path.iter().collect::<String>());
    // dbg!(&number_path.len());

    number_path
}

fn dir_pad(commands: Vec<char>, positions: &HashMap<char, Position>, neighbors: &HashMap<String, Vec<&str>>) -> Vec<char> {
//     dbg!(&code);

    let mut current = 'A';
    let mut dir_path: Vec<char> = Vec::new();
    
    for command in commands {
        if let Some(path) = find_shortest_path(current, command, positions, &neighbors) {
            dir_path.extend(path);
            dir_path.push('A');
            current = command;
        }
    }
    // dbg!(&dir_path.iter().collect::<String>());
    // dbg!(&dir_path.len());

    dir_path
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let num_grid = make_numeric_keypad();
    // dbg!(&num_grid);
    
    let dir_grid = make_dir_pad();
    // dbg!(&dir_grid);

    let char_positions = to_positions(&num_grid);
    // dbg!(&char_positions);

    let arrow_positions = to_positions(&dir_grid);
    // dbg!(&arrow_positions);

    let char_neighbors = get_neighbors(num_grid);
    let dir_neighbors = get_neighbors(dir_grid);
    // dbg!(&char_neighbors);
    // dbg!(&dir_neighbors);

    let output = input
        .lines()
        .map(|code| number_pad(code, &char_positions, &char_neighbors) )
        .map(|commands| dir_pad(commands, &arrow_positions, &dir_neighbors) )
        .map(|commands| dir_pad(commands, &arrow_positions, &dir_neighbors) )
        .collect::<Vec<_>>();

    // dbg!(&output.len());

    let final_output: usize = input.lines().enumerate().map(|(idx, line)| {
        // dbg!(&line);

        let num: usize = line.chars().filter_map(|c| {
            if c.is_ascii_digit() {
                Some(c)
            } else {
                None
            }
        }).collect::<String>().parse().unwrap();

        // dbg!(num, output[idx].len());

        num * (output[idx].len())
    }).sum();

    Ok(final_output.to_string())
}


fn find_shortest_path(
    from: char,
    to: char,
    char_positions: &HashMap<char, Position>,
    neighbors: &HashMap<String, Vec<&str>>
) -> Option<Vec<char>> {
    // Get positions
    let start_pos = char_positions.get(&from)?;
    let end_pos = char_positions.get(&to)?;
    
    // Use get_all_paths to get all possible paths
    // The skip position (0,3) seems to be the empty spot in the keypad
    let paths = get_all_paths(*start_pos, *end_pos, Position::new(3, 0));
    
    // Convert the shortest path string into a Vec<char>
    paths.into_iter()
        .min_by_key(|path| path.len())?  // Get shortest path
        .chars()
        .filter(|&c| c != 'a')  // Remove the 'a' since number_pad/dir_pad add it later
        .collect::<Vec<_>>()
        .into()
}


// fn find_shortest_path(
//     from: char,
//     to: char,
//     char_positions: &HashMap<char, Position>,
//     neighbors: &HashMap<String, Vec<&str>>
//     // skip: Position
// ) -> Option<Vec<char>> {
//     let mut queue = VecDeque::new();
//     let mut visited = HashSet::new();
    
//     // (current_char, path_so_far)
//     queue.push_back((from, Vec::new()));
//     visited.insert(from);

//     while let Some((current, path)) = queue.pop_front() {
//         // Found A POSSIBLE target, need all of them??
//         if current == to {
//             // dbg!(&path.iter().collect::<String>());
//             // panic!("Found A target");

//             return Some(path);
//         }

//         // Get neighbors of current position
//         if let Some(next_positions) = neighbors.get(&current.to_string()) {
//             for next in next_positions {
//                 let next_char = next.chars().next().expect("Empty string");
//                 if !visited.insert(next_char) {
//                     continue;
//                 }

//                 // Calculate direction to add to path
//                 let curr_pos = char_positions.get(&current).unwrap();
//                 let next_pos = char_positions.get(&next_char).unwrap();
//                 let direction = get_direction(*curr_pos, *next_pos);

//                 let mut new_path = path.clone();
//                 new_path.push(direction);
                
//                 queue.push_back((next_char, new_path));
//             }
//         }
//     }
//     None
// }

// fn get_direction(from: Position, to: Position) -> char {
//     let diff = to - from;
//     match (diff.x, diff.y) {
//         (-1, 0) => '^', // (0, -1) => '^',
//         (1, 0) => 'v', // (0, 1) => 'v'
//         (0, -1) => '<', // (-1, 0)
//         (0, 1) => '>', // (1, 0)
//         _ => panic!("Invalid move")
//     }
// }

// #[deprecated(note = "Use get_all_paths instead")]
// fn get_direction(from: Position, to: Position) -> char {
//     let diff = to - from;
//     match (diff.x, diff.y) {
//         (0, -1) => '^',
//         (0, 1) => 'v',
//         (-1, 0) => '<',
//         (1, 0) => '>',
//         _ => panic!("Invalid move")
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(('A', '0'), 1)]
    #[case(('0', '2'), 1)]
    #[case(('2', '9'), 3)]
    #[case(('9', 'A'), 1)]
    fn test_all_paths(#[case] (from, to): (char, char), #[case] expected: usize) {
        let num_grid = make_numeric_keypad();
        let char_positions = to_positions(&num_grid);

        let all_paths = get_all_paths(char_positions[&from], char_positions[&to], Position::new(0,3));
        assert_eq!(all_paths.len(), expected);
    }

    // #[rstest]
    // #[case("029A", 3)]
    // fn test_all_paths_2(#[case] code: &str, #[case] expected: usize) {
    //     let num_grid = make_numeric_keypad();
    //     let char_positions = to_positions(&num_grid);
    //     let char_neighbors = get_neighbors(num_grid);

    //     let mut current = 'A';
    //     let mut all_paths = 0;
        
    //     // for target in code.chars() {
    //     //     if let Some(paths) = get_all_paths(current, target, &char_positions, &char_neighbors) {
    //     //         all_paths += paths.len();
    //     //         current = target;
    //     //     }
    //     // }

    //     assert_eq!(all_paths, expected);
    // }

    #[rstest]
    #[case("029A", 12)] // ["<A^A>^^AvvvA", "<A^A^>^AvvvA", "<A^A^^>AvvvA"]
    #[case("980A", 12)] // ["^^^A<AvvvA>A", ""]
    #[case("179A", 14)] //?? ["^<<A^^A>>AvvvA", ""]
    #[case("456A", 12)] // ["^^<<A>A>AvvA", ""]
    #[case("379A", 14)] // ??
    fn test_numeric(#[case] input: &str, #[case] expected: usize) {
        let num_grid = make_numeric_keypad();        
        // let dir_grid = make_dir_pad();
        let char_positions = to_positions(&num_grid);
        // let arrow_positions = to_positions(&dir_grid);
        let char_neighbors = get_neighbors(num_grid);
        // let dir_neighbors = get_neighbors(dir_grid);

        assert_eq!(number_pad(input, &char_positions, &char_neighbors).len(), expected);
    }

    // #[rstest]
    // #[case("029A", 28)] // ["<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"]
    // #[case("980A", 26)] // ??
    // // #[case("179A", 28)] // ???
    // // #[case("456A", 26?)] // ??
    // // #[case("379A", 28)] // ???
    // fn test_direction(#[case] input: &str, #[case] expected: usize) {
    //     let num_grid = make_numeric_keypad();        
    //     let dir_grid = make_dir_pad();
    //     let char_positions = to_positions(&num_grid);
    //     let arrow_positions = to_positions(&dir_grid);
    //     let char_neighbors = get_neighbors(num_grid);
    //     let dir_neighbors = get_neighbors(dir_grid);

    //     assert_eq!(dir_pad(number_pad(input, &char_positions, &char_neighbors), &arrow_positions, &dir_neighbors).len(), expected);
    // }

    // #[rstest]
    // #[case("029A", 2030)] // 2030??, 1856??
    // #[case("980A", 60760)] // 60760??, 58800??
    // #[case("179A", 12530)] // 12530??, 12888??
    // #[case("456A", 30096)] // 30096??, 31008??
    // #[case("379A", 26530)] // 26530??, 27288??
    // fn test_lines(#[case] input: &str, #[case] expected: usize) {
    //     assert_eq!(process(input).unwrap().parse::<usize>().unwrap(), expected);
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
