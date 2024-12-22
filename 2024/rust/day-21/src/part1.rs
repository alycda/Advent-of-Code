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

fn _get_neighbors(grid: Grid<&str>) -> HashMap<String, Vec<&str>> {
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

fn number_pad(code: &str, positions: &HashMap<char, Position>) -> Vec<char> {
    dbg!(&code);

    let mut current = 'A';
    let mut number_path = Vec::new();
    
    for target in code.chars() {
        if let Some(path) = find_shortest_path(current, target, positions) {
            number_path.extend(path);
            number_path.push('A'); // Add button press
            current = target;
        }
    }
    // dbg!(&number_path.iter().collect::<String>());
    // dbg!(&number_path.len());

    number_path
}

fn dir_pad(commands: Vec<char>, positions: &HashMap<char, Position>) -> Vec<char> {
//     dbg!(&code);

    let mut current = 'A';
    let mut dir_path: Vec<char> = Vec::new();
    
    for command in commands {
        if let Some(path) = find_shortest_path(current, command, positions) {
            dir_path.extend(path);
            dir_path.push('A');
            current = command;
        }
    }
    // dbg!(&dir_path.iter().collect::<String>());
    // dbg!(&dir_path.len());

    dir_path
}

fn new_shortest_length(code: &str, cache: &mut HashMap<Combo, usize>, robots: usize, depth: usize) -> usize {
    let num_grid = make_numeric_keypad();
    // dbg!(&num_grid);
    
    let dir_grid = make_dir_pad();
    // dbg!(&dir_grid);

    let char_positions = to_positions(&num_grid);
    // dbg!(&char_positions);

    let arrow_positions = to_positions(&dir_grid);
    // dbg!(&arrow_positions);

    let skip_button = if depth == 0 { Position::new(3, 0) } else { Position::ZERO };

    // let mut pos = start;
    let mut start = if depth == 0 {
        char_positions.get(&'A').unwrap()
    } else {
        arrow_positions.get(&'A').unwrap()
    };

    // dbg!(&code);

    code.chars().fold(0, |length, c| {
        let mut count = length;

        // dbg!(c);

        let next = if depth == 0 {
            char_positions.get(&c).unwrap()
        } else {
            arrow_positions.get(&c).unwrap()
        };

        let saved = Combo {
            from: *start,
            to: *next,
            depth,
            robots,
        };

        if let Some(&cached) = cache.get(&saved) {
            count += cached;
        } else {
            let combos = get_all_paths(*start, *next, skip_button);
            let min_len = if depth == robots {
                combos[0].len()
            } else {
                combos.iter()
                    .map(|m| new_shortest_length(m, cache, robots, depth + 1))
                    .min()
                    .unwrap_or(0)
            };
            cache.insert(saved, min_len);
            count += min_len;
        }
        start = next;

        count
    })
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Combo {
    from: Position,
    to: Position,
    depth: usize,
    robots: usize,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut cache = HashMap::new();
    // let mut output = 0;


    // let char_neighbors = get_neighbors(num_grid);
    // let dir_neighbors = get_neighbors(dir_grid);
    // // dbg!(&char_neighbors);
    // // dbg!(&dir_neighbors);

    let output: usize = input
        .lines()
            .map(|line| {
                let numeric: usize = line[..3].parse().unwrap();
                let length = new_shortest_length(line, &mut cache, 2, 0);

                length * numeric
            }).sum();
    //     .map(|code| number_pad(code, &char_positions) )
    //     .map(|commands| dir_pad(commands, &arrow_positions) )
    //     .map(|commands| dir_pad(commands, &arrow_positions) )
    //     .collect::<Vec<_>>();

    // // dbg!(&output.len());

    // let final_output: usize = input.lines().enumerate().map(|(idx, line)| {
    //     // dbg!(&line);

    //     let num: usize = line.chars().filter_map(|c| {
    //         if c.is_ascii_digit() {
    //             Some(c)
    //         } else {
    //             None
    //         }
    //     }).collect::<String>().parse().unwrap();

    //     // dbg!(num, output[idx].len());

    //     num * (output[idx].len())
    // }).sum();

    // Ok(final_output.to_string())

    Ok(output.to_string())
}


fn find_shortest_path(
    from: char,
    to: char,
    char_positions: &HashMap<char, Position>,
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
        .filter(|&c| c != 'A')  // Remove the 'a' since number_pad/dir_pad add it later
        .collect::<Vec<_>>()
        .into()
}

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
        // let char_neighbors = get_neighbors(num_grid);
        // let dir_neighbors = get_neighbors(dir_grid);

        assert_eq!(number_pad(input, &char_positions).len(), expected);
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
    // #[case("029A", 1856)] // 2030??, 1856??
    // #[case("980A", 58800)] // 60760??, 58800??
    // #[case("179A", 12888)] // 12530??, 12888??
    // #[case("456A", 31008)] // 30096??, 31008??
    // #[case("379A", 27288)] // 26530??, 27288??
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
