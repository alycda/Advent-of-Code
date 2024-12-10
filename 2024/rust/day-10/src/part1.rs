use glam::IVec2;
use std::collections::HashMap;
use crate::custom_error::AocError;

/// Grid structure to handle our topographic map
#[derive(Debug)]
struct Grid {
    // Store heights in a HashMap for easy coordinate access
    // Key is (x,y) coordinate, Value is height
    map: HashMap<IVec2, u32>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut map = HashMap::new();
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();
        
        // Parse input into coordinate map, similar to Python version
        for (x, line) in input.lines().enumerate() {
            for (y, height) in line.chars().enumerate() {
                map.insert(
                    IVec2::new(x as i32, y as i32),
                    height.to_digit(10).unwrap()
                );
            }
        }
        
        Self { map, rows, cols }
    }

    /// Find all starting points (positions with height 0)
    fn get_trail_heads(&self) -> Vec<IVec2> {
        self.map
            .iter()
            .filter(|&(_, &height)| height == 0)
            .map(|(&pos, _)| pos)
            .collect()
    }

        fn count_paths(&self, pos: IVec2) -> usize {
            // Base case: if we're at a 9, we've found a valid path
            if let Some(&height) = self.map.get(&pos) {
                if height == 9 {
                    return 1;
                }
                
                // Look for next number in all four directions
                let mut count = 0;
                for delta in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let next_pos = IVec2::new(pos.x + delta.0, pos.y + delta.1);
                    
                    // Only recurse if we find exactly the next number in sequence
                    if let Some(&next_height) = self.map.get(&next_pos) {
                        if next_height == height + 1 {
                            count += self.count_paths(next_pos);
                        }
                    }
                }
                return count;
            }
            0
        }
    

    // fn count_paths(&self, pos: IVec2, current_path: &mut Vec<IVec2>) -> usize {
    //     // Add current position to path
    //     current_path.push(pos);

    //     // Base case: we've reached a 9
    //     if self.map.get(&pos) == Some(&9) {
    //         println!("Found path to 9:");
    //         for pos in current_path.iter() {
    //             print!("({},{})={} -> ", pos.x, pos.y, self.map.get(pos).unwrap());
    //         }
    //         println!();
    //         current_path.pop();
    //         return 1;
    //     }

    //     let current_height = self.map.get(&pos).unwrap();
    //     let mut total_paths = 0;

    //     // Check all four directions
    //     for delta in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
    //         let next_pos = IVec2::new(
    //             pos.x + delta.0,
    //             pos.y + delta.1
    //         );

    //         if let Some(&height) = self.map.get(&next_pos) {
    //             if height == current_height + 1 && !current_path.contains(&next_pos) {
    //                 total_paths += self.count_paths(next_pos, current_path);
    //             }
    //         }
    //     }

    //     current_path.pop();
    //     total_paths
    // }

    // /// Count paths from a starting point to any 9
    // fn count_paths(&self, pos: IVec2) -> usize {
    //     // Base case: we've reached a 9, this is a valid path
    //     if self.map.get(&pos) == Some(&9) {
    //         return 1;
    //     }

    //     let current_height = self.map.get(&pos).unwrap();
    //     let mut total_paths = 0;

    //     // Check all four directions
    //     for delta in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
    //         let next_pos = IVec2::new(
    //             pos.x + delta.0,
    //             pos.y + delta.1
    //         );

    //         // Look for next higher number (current + 1)
    //         if let Some(&height) = self.map.get(&next_pos) {
    //             if height == current_height + 1 {
    //                 total_paths += self.count_paths(next_pos);
    //             }
    //         }
    //     }

    //     total_paths
    // }
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid = Grid::new(input);
    
    println!("Grid:");
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            print!("{}", grid.map.get(&IVec2::new(row as i32, col as i32)).unwrap());
        }
        println!();
    }
    println!();

    let mut total = 0;
    for start_pos in grid.get_trail_heads() {
        println!("\nStarting from zero at ({}, {})", start_pos.x, start_pos.y);
        let paths = grid.count_paths(start_pos, &mut Vec::new());
        println!("Found {} paths from this zero", paths);
        total += paths;
    }

    Ok(total.to_string())
}

// pub fn process(input: &str) -> miette::Result<String, AocError> {
//     let grid = Grid::new(input);
    
//     // Find all trail heads (zeros) and sum their path counts
//     let total: usize = grid
//         .get_trail_heads()
//         .into_iter()
//         .map(|start_pos| grid.count_paths(start_pos))
//         .sum();

//     Ok(total.to_string())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let input = "0123\n1234\n8765\n9876";
        assert_eq!(process(input).unwrap(), "1");
    }
}

// use glam::IVec2;

// use crate::custom_error::AocError;

// /// COL, ROW
// #[derive(Debug, PartialEq, Eq)]
// struct Grid(usize, usize, String);


// #[derive(Debug, PartialEq, Eq)]
// enum Direction {
//     Up(IVec2, char),
//     Down(IVec2, char),
//     Left(IVec2, char),
//     Right(IVec2, char),
// }

// impl Grid {
//     fn new(cols: usize, rows: usize, data: String) -> Self {
//         Self(cols, rows, data)
//     }

//     // chars per row is cols + 1
//     fn get_cols(&self) -> usize {
//         self.0 + 1
//     }

//     fn get_rows(&self) -> usize {
//         self.1
//     }

//         fn to_idx(&self, pos: &IVec2) -> usize {
//             let idx = self.get_cols() * (pos.y) as usize + (pos.x) as usize;
//             println!(
//                 "Converting position ({}, {}) to index {}:",
//                 pos.x, pos.y, idx
//             );
//             println!(
//                 "- cols: {}", self.get_cols(),
//                 "- calculation: {} * {} + {}",
//                 self.get_cols(), pos.y, pos.x
//             );
//             println!(
//                 "- string length: {}", self.2.len(),
//                 "- data: {:?}", self.2
//             );
            
//             // Add bounds checking
//             if idx >= self.2.len() {
//                 panic!(
//                     "Index {} is out of bounds for string of length {}. Position: ({}, {})",
//                     idx, self.2.len(), pos.x, pos.y
//                 );
//             }
            
//             idx
//         }

//     // /// accounts for newline characters
//     // fn to_idx(&self, pos: &IVec2) -> usize {
//     //     // let chars_per_row = self.get_cols() + 1;

//     //     self.get_cols() * (pos.y) as usize + (pos.x) as usize
//     // }

//     fn get_char(&self, pos: &IVec2) -> char {
//         self.2.as_bytes()[self.to_idx(pos)] as char
//     }

//     fn get_neighbors(&self, pos: IVec2) -> Vec<Direction> {
//         // todo: Hashmap
//         let mut neighbors = Vec::new();
//         let cols = self.get_cols();
//         let rows = self.get_rows();

//         println!(
//             "Getting neighbors for position ({}, {})",
//             pos.x, pos.y
//         );
//         println!(
//             "Grid dimensions: {} cols x {} rows",
//             cols, rows
//         );

//         let col = pos.x;
//         let row = pos.y;

//         // Check up (decrease Y)
//         if row > 0 {
//             let neighbor = IVec2::new(col, row - 1);
//             neighbors.push(Direction::Up(neighbor, self.get_char(&neighbor)));
//         }

//         // Check down (increase Y)
//         if row + 1 < rows as i32 {
//             let neighbor = IVec2::new(col, row + 1);
//             neighbors.push(Direction::Down(neighbor, self.get_char(&neighbor)));
//         }

//         // Check left (decrease X)
//         if col > 0 {
//             let neighbor = IVec2::new(col - 1, row);
//             neighbors.push(Direction::Left(neighbor, self.get_char(&neighbor)));
//         }

//         // Check right (increase X)
//         if col + 1 < cols as i32 {
//             let neighbor = IVec2::new(col + 1, row);
//             neighbors.push(Direction::Right(neighbor, self.get_char(&neighbor)));
//         }

//         neighbors
//     }

//     fn count_paths_from_zero(&self, pos: IVec2) -> usize {
//         // If we've reached a 9, we've found a valid path
//         if self.get_char(&pos).to_digit(10).unwrap() == 9 {
//             return 1;
//         }

//         let current_value = self.get_char(&pos).to_digit(10).unwrap();
        
//         // Look for the next higher number in each direction
//         self.get_neighbors(pos)
//             .into_iter()
//             .filter_map(|direction| {
//                 let (next_pos, c) = match direction {
//                     Direction::Up(p, c) |
//                     Direction::Down(p, c) |
//                     Direction::Left(p, c) |
//                     Direction::Right(p, c) => (p, c)
//                 };
                
//                 c.to_digit(10).and_then(|value| {
//                     // We want exactly one more than our current value
//                     if value == current_value + 1 {
//                         Some(next_pos)
//                     } else {
//                         None
//                     }
//                 })
//             })
//             .map(|next_pos| self.count_paths_from_zero(next_pos))
//             .sum()
//     }

//     fn count_paths_from_trailhead(&self, start_pos: IVec2, trailhead_index: usize) -> usize {
//         println!("\n=== Processing Trailhead #{} at {:?} ===", trailhead_index + 1, start_pos);
//         let mut current_path = Vec::new();
//         let paths = self.find_paths(start_pos, 9, &mut current_path);
//         println!("Total paths for this trailhead: {}\n", paths);
//         paths
//     }

//     fn find_paths(&self, pos: IVec2, current_value: u32, path: &mut Vec<IVec2>) -> usize {
//         if current_value == 0 {
//             // Print the complete path when we find one
//             println!("Found valid path:");
//             path.iter().for_each(|pos| {
//                 print!("({},{})={} -> ", pos.x, pos.y, self.get_char(pos));
//             });
//             println!("({},{})=0", pos.x, pos.y);
//             return 1;
//         }

//         path.push(pos);
//         let target = current_value - 1;
        
//         // First collect valid neighbors
//         let valid_neighbors: Vec<IVec2> = self.get_neighbors(pos)
//             .into_iter()
//             .filter_map(|direction| {
//                 let (next_pos, c) = match direction {
//                     Direction::Up(p, c) |
//                     Direction::Down(p, c) |
//                     Direction::Left(p, c) |
//                     Direction::Right(p, c) => (p, c)
//                 };
                
//                 if !path.contains(&next_pos) {
//                     c.to_digit(10).and_then(|value| {
//                         if value == target {
//                             Some(next_pos)
//                         } else {
//                             None
//                         }
//                     })
//                 } else {
//                     None
//                 }
//             })
//             .collect();

//         let paths: usize = valid_neighbors.into_iter()
//             .map(|next_pos| {
//                 let count = self.find_paths(next_pos, target, path);
//                 path.pop();
//                 count
//             })
//             .sum();

//         path.pop();
//         paths
//     }

//         // This function will be our entry point to find paths for a single trailhead (9)
//         // fn count_paths_from_trailhead(&self, start_pos: IVec2) -> usize {
//         //     // Start a fresh path for each trailhead
//         //     let mut current_path = Vec::new();
//         //     self.find_paths(start_pos, 9, &mut current_path)
//         // }

//         // fn find_paths(&self, pos: IVec2, current_value: u32, path: &mut Vec<IVec2>) -> usize {
//         //     if current_value == 0 {
//         //         return 1;
//         //     }
        
//         //     path.push(pos);
//         //     let target = current_value - 1;
            
//         //     // Collect valid neighbors first
//         //     let valid_neighbors: Vec<IVec2> = self.get_neighbors(pos)
//         //         .into_iter()
//         //         .filter_map(|direction| {
//         //             let (next_pos, c) = match direction {
//         //                 Direction::Up(p, c) |
//         //                 Direction::Down(p, c) |
//         //                 Direction::Left(p, c) |
//         //                 Direction::Right(p, c) => (p, c)
//         //             };
                    
//         //             if !path.contains(&next_pos) {
//         //                 c.to_digit(10).and_then(|value| {
//         //                     if value == target {
//         //                         Some(next_pos)
//         //                     } else {
//         //                         None
//         //                     }
//         //                 })
//         //             } else {
//         //                 None
//         //             }
//         //         })
//         //         .collect();
            
//         //     // Now process neighbors sequentially
//         //     let paths: usize = valid_neighbors.into_iter()
//         //         .map(|next_pos| {
//         //             let count = self.find_paths(next_pos, target, path);
//         //             path.pop();  // Clean up after exploring this path
//         //             count
//         //         })
//         //         .sum();
        
//         //     path.pop();  // Remove current position
//         //     paths
//         // }

//         // fn find_paths(&self, pos: IVec2, current_value: u32, path: &mut Vec<IVec2>) -> usize {
//         //     // If we've found a 0, we've found a valid path
//         //     if current_value == 0 {
//         //         return 1;
//         //     }
    
//         //     // Track this position in our current path to avoid cycles
//         //     path.push(pos);
            
//         //     let target = current_value - 1; // We're looking for exactly one less
            
//         //     // Get paths count from valid neighbors
//         //     let paths = self.get_neighbors(pos)
//         //         .into_iter()
//         //         .filter_map(|direction| {
//         //             let (next_pos, c) = match direction {
//         //                 Direction::Up(p, c) |
//         //                 Direction::Down(p, c) |
//         //                 Direction::Left(p, c) |
//         //                 Direction::Right(p, c) => (p, c)
//         //             };
                    
//         //             // Only proceed if:
//         //             // 1. This position isn't in our current path
//         //             // 2. The value is exactly what we're looking for
//         //             if !path.contains(&next_pos) {
//         //                 c.to_digit(10).and_then(|value| {
//         //                     if value == target {
//         //                         Some(next_pos)
//         //                     } else {
//         //                         None
//         //                     }
//         //                 })
//         //             } else {
//         //                 None
//         //             }
//         //         })
//         //         .map(|next_pos| {
//         //             // Recursively explore this path
//         //             let count = self.find_paths(next_pos, target, path);
//         //             path.pop(); // Remove this position before trying other branches
//         //             count
//         //         })
//         //         .sum();
    
//         //     // Remove current position before returning
//         //     path.pop();
//         //     paths
//         // }

// }

// pub fn process(input: &str) -> miette::Result<String, AocError> {    
//     let mut peekable = input.lines().peekable();
//     let cols = peekable.peek().unwrap().chars().count();
//     let rows = peekable.count();

//     let grid = Grid::new(cols, rows, input.to_string());

//     // Find all zeros first
//     let total: usize = input
//         .lines()
//         .enumerate()
//         .flat_map(|(row, line)| {
//             line.chars()
//                 .enumerate()
//                 .filter(|(_, c)| *c == '0')
//                 .map(move |(col, _)| IVec2::new(col as i32, row as i32))
//         })
//         .map(|zero_pos| grid.count_paths_from_zero(zero_pos))
//         .sum();

//     Ok(total.to_string())
// }

// // pub fn process(input: &str) -> miette::Result<String, AocError> {    
// //     let mut peekable = input.lines().peekable();
// //     let cols = peekable.peek().unwrap().chars().count();
// //     let rows = peekable.count();

// //     let grid = Grid::new(cols, rows, input.to_string());

// //     // Find all 9s in reading order
// //     let trailhead_scores: Vec<usize> = input
// //         .lines()
// //         .enumerate()
// //         .flat_map(|(row, line)| {
// //             line.chars()
// //                 .enumerate()
// //                 .filter(|(_, c)| *c == '9')
// //                 .map(move |(col, _)| IVec2::new(col as i32, row as i32))
// //         })
// //         .enumerate()
// //         .map(|(idx, start_pos)| {
// //             grid.count_paths_from_trailhead(start_pos, idx)
// //         })
// //         // .map(|start_pos| {
// //         //     println!("Processing trailhead at {:?}", start_pos);
// //         //     let paths = grid.count_paths_from_trailhead(start_pos);
// //         //     println!("Found {} paths", paths);
// //         //     paths
// //         // })
// //         .collect();

// //     let total:usize = trailhead_scores.iter().sum();
// //     Ok(total.to_string())
// // }

// // pub fn process(input: &str) -> miette::Result<String, AocError> {    
// //     let mut peekable = input.lines().peekable();
// //     let cols = peekable.peek().unwrap().chars().count();
// //     let rows = peekable.count();

// //     let grid = Grid::new(cols, rows, input.to_string());

// //     let mut trailhead_scores = Vec::new();

// //     let output: usize = input
// //         .lines()
// //         .enumerate()
// //         .flat_map(|(row, line)| {
// //             line.chars()
// //                 .enumerate()
// //                 // .inspect(|(col, d)| {
// //                 //     let d = d.to_digit(10).unwrap();

// //                 //     if d == 9 {
// //                 //         dbg!((row, col, d));

// //                 //         dbg!(grid.get_neighbors(IVec2::new(*col as i32, *row as i32)));
// //                 //     }

// //                 // })
// //                 // .count();
// //                 .filter(|(_, c)| *c == '9')
// //                 .map(move |(col, _)| IVec2::new(col as i32, row as i32))
// //         })
// //         .map(|start_pos| {
// //             dbg!(&start_pos);

// //             let paths = dbg!(grid.find_paths(start_pos, 9, Vec::new()));
// //             trailhead_scores.push(paths.clone());

// //             paths
// //         })
// //         .sum();

// //     dbg!(&trailhead_scores);
// //     let total = trailhead_scores.iter().sum::<usize>();
// //     dbg!(total);

// //     panic!("halt");

// //     Ok(output.to_string())
// // }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use rstest::rstest;

//     #[rstest]
//     #[case("0123
// 1234
// 8765
// 9876", "1")]
// //     #[case("...0...
// // ...1...
// // ...2...
// // 6543456
// // 7.....7
// // 8.....8
// // 9.....9", "2")]
// // #[case("..90..9
// // ...1.98
// // ...2..7
// // 6543456
// // 765.987
// // 876....
// // 987....", "4")]
// // #[case("10..9..
// // 2...8..
// // 3...7..
// // 4567654
// // ...8..3
// // ...9..2
// // .....01", "3")]
// // #[case("89010123
// // 78121874
// // 87430965
// // 96549874
// // 45678903
// // 32019012
// // 01329801
// // 10456732", "9")]
//     fn test_cases(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
//         // assert_eq!(process(input).unwrap(), expected);
//         assert_eq!(expected, process(input)?);

//         Ok(())
//     }

// //     #[test]
// //     fn test_process() -> miette::Result<()> {
// //         let input = "89010123
// // 78121874
// // 87430965
// // 96549874
// // 45678903
// // 32019012
// // 01329801
// // 10456732";
// //         assert_eq!("9", process(input)?);
// //         Ok(())
// //     }
// }
