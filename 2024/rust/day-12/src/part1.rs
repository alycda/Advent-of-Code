
use crate::custom_error::AocError;

use std::collections::HashSet;
use glam::IVec2;

fn get_all_neighbors(pos: IVec2, grid: &Vec<Vec<char>>) -> Vec<IVec2> {
    let mut neighbors = Vec::new();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    
    // Check all four directions
    for delta in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let new_pos = IVec2::new(pos.x + delta.0, pos.y + delta.1);
        if new_pos.x >= 0 && new_pos.x < cols && 
           new_pos.y >= 0 && new_pos.y < rows {
            neighbors.push(new_pos);
        }
    }
    neighbors
}

fn flood_fill(grid: &Vec<Vec<char>>, start: IVec2, visited: &mut HashSet<IVec2>) -> HashSet<IVec2> {
    let mut region = HashSet::new();
    let mut stack = vec![start];
    let target_char = grid[start.y as usize][start.x as usize];
    
    while let Some(pos) = stack.pop() {
        if !region.insert(pos) {
            continue;
        }
        visited.insert(pos);
        
        for neighbor in get_all_neighbors(pos, grid) {
            let neighbor_char = grid[neighbor.y as usize][neighbor.x as usize];
            if neighbor_char == target_char && !region.contains(&neighbor) {
                stack.push(neighbor);
            }
        }
    }
    
    region
}

fn count_region_edges(grid: &Vec<Vec<char>>, region: &HashSet<IVec2>) -> usize {
    let mut edges = 0;
    
    for &pos in region {
        for neighbor in get_all_neighbors(pos, grid) {
            // let row = neighbor.y as usize;
            // let col = neighbor.x as usize;
            
            // If neighbor is outside region, it's an edge
            if !region.contains(&neighbor) {
                edges += 1;
            }
        }
        
        // Count border edges
        if pos.x == 0 || pos.x == (grid[0].len() - 1) as i32 { edges += 1; }
        if pos.y == 0 || pos.y == (grid.len() - 1) as i32 { edges += 1; }
    }
    
    edges
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    
    let mut visited = HashSet::new();
    let mut regions = Vec::new();
    
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let pos = IVec2::new(col as i32, row as i32);
            if !visited.contains(&pos) {
                let region = flood_fill(&grid, pos, &mut visited);
                let char_type = grid[row][col];
                let count = region.len();
                let edges = count_region_edges(&grid, &region);
                regions.push((char_type, count, edges));
            }
        }
    }
    
    // Debug
    for (c, count, edges) in &regions {
        println!("Region of {} plants with price {} * {} = {}", 
            c, count, edges, count * edges);
    }
    
    let total: usize = regions.iter()
        .map(|(_, count, edges)| count * edges)
        .sum();
    
    Ok(total.to_string())
}


// // Helper to get unique region ID for a position
// fn flood_fill(grid: &Vec<Vec<char>>, pos: IVec2, visited: &mut HashSet<IVec2>) -> HashSet<IVec2> {
//     let mut region = HashSet::new();
//     let mut stack = vec![pos];
//     let target_char = grid[pos.y as usize][pos.x as usize];
    
//     while let Some(current) = stack.pop() {
//         if !visited.insert(current) {
//             continue;
//         }
        
//         region.insert(current);
        
//         // Check all adjacent positions
//         for (neighbor_pos, neighbor_char) in get_forward_neighbors(current, grid) {
//             if neighbor_char == target_char && !visited.contains(&neighbor_pos) {
//                 stack.push(neighbor_pos);
//             }
//         }
//     }
    
//     region
// }

// fn count_region_edges(grid: &Vec<Vec<char>>, region: &HashSet<IVec2>) -> usize {
//     let mut edges = 0;
    
//     for &pos in region {
//         let row = pos.y as usize;
//         let col = pos.x as usize;
        
//         // Check borders
//         if col == 0 || col == grid[0].len() - 1 { edges += 1; }
//         if row == 0 || row == grid.len() - 1 { edges += 1; }
        
//         // Check neighbors
//         for (neighbor_pos, neighbor_char) in get_forward_neighbors(pos, grid) {
//             if !region.contains(&neighbor_pos) {
//                 edges += 1;
//             }
//         }
//     }
    
//     edges
// }

// fn analyze_regions(grid: &Vec<Vec<char>>) -> Vec<(char, usize, usize)> {
//     let mut regions = Vec::new();
//     let mut visited = HashSet::new();
    
//     for row in 0..grid.len() {
//         for col in 0..grid[0].len() {
//             let pos = IVec2::new(col as i32, row as i32);
//             if !visited.contains(&pos) {
//                 let region = flood_fill(grid, pos, &mut visited);
//                 let char_type = grid[row][col];
//                 let count = region.len();
//                 let edges = count_region_edges(grid, &region);
//                 regions.push((char_type, count, edges));
//             }
//         }
//     }
    
//     regions
// }

// pub fn process(input: &str) -> miette::Result<String, AocError> {
//     let grid: Vec<Vec<char>> = input.lines()
//         .map(|line| line.chars().collect())
//         .collect();
    
//     let regions = analyze_regions(&grid);
//     let total: usize = regions.iter()
//         .map(|(_, count, edges)| count * edges)
//         .sum();
    
//     // Debug print
//     for (c, count, edges) in &regions {
//         println!("Region of {} plants with price {} * {} = {}", 
//             c, count, edges, count * edges);
//     }
    
//     Ok(total.to_string())
// }

// fn get_forward_neighbors(pos: IVec2, grid: &Vec<Vec<char>>) -> Vec<(IVec2, char)> {
//     let mut neighbors = Vec::new();
//     let cols = grid[0].len();
//     let rows = grid.len();
//     let col = pos.x as usize;
//     let row = pos.y as usize;
    
//     // Only check right and down
//     if col + 1 < cols {
//         let neighbor = IVec2::new((col + 1) as i32, row as i32);
//         neighbors.push((neighbor, grid[row][col + 1]));
//     }
//     if row + 1 < rows {
//         let neighbor = IVec2::new(col as i32, (row + 1) as i32);
//         neighbors.push((neighbor, grid[row + 1][col]));
//     }

//     neighbors
// }

// fn count_char_edges(grid: &Vec<Vec<char>>) -> HashMap<char, (usize, usize)> {
//     let mut char_map: HashMap<char, (usize, usize)> = HashMap::new();
//     let mut edge_updates: Vec<(char, char)> = Vec::new();
    
//     for row in 0..grid.len() {
//         for col in 0..grid[0].len() {
//             let current_char = grid[row][col];
//             let current_pos = IVec2::new(col as i32, row as i32);
            
//             // Increment character count
//             char_map.entry(current_char)
//                    .and_modify(|e| e.0 += 1)
//                    .or_insert((1, 0));
            
//             // Collect edge information
//             for (_, neighbor_char) in get_forward_neighbors(current_pos, &grid) {
//                 if neighbor_char != current_char {
//                     edge_updates.push((current_char, neighbor_char));
//                 }
//             }
            
//             // Count border edges
//             if col == 0 || col == grid[0].len() - 1 {
//                 char_map.entry(current_char)
//                        .and_modify(|e| e.1 += 1)
//                        .or_insert((1, 1));
//             }
//             if row == 0 || row == grid.len() - 1 {
//                 char_map.entry(current_char)
//                        .and_modify(|e| e.1 += 1)
//                        .or_insert((1, 1));
//             }
//         }
//     }
    
//     // Process edge updates
//     for (char1, char2) in edge_updates {
//         char_map.entry(char1).and_modify(|e| e.1 += 1);
//         char_map.entry(char2).and_modify(|e| e.1 += 1);
//     }
    
//     dbg!(char_map)
// }

// pub fn process(input: &str) -> miette::Result<String, AocError> {
//     let grid: Vec<Vec<char>> = input.lines()
//         .map(|line| line.chars().collect())
//         .collect();
    
//     let char_edges = count_char_edges(&grid);
//     dbg!(&char_edges);
    
//     let total: usize = char_edges.iter()
//         .map(|(_, (count, edges))| count * edges)
//         .sum();
    
//     Ok(total.to_string())
// }

// fn get_all_neighbors(pos: IVec2, grid: &Vec<Vec<char>>) -> Vec<(IVec2, char)> {
//     let mut neighbors = Vec::new();
//     let cols = grid[0].len();
//     let rows = grid.len();
//     let col = pos.x as usize;
//     let row = pos.y as i32 as usize;  // Explicit conversion
    
//     // Check all four directions
//     // Right
//     if col + 1 < cols {
//         let neighbor = IVec2::new((col + 1) as i32, row as i32);
//         neighbors.push((neighbor, grid[row][col + 1]));
//     }
//     // Left
//     if col > 0 {
//         let neighbor = IVec2::new((col - 1) as i32, row as i32);
//         neighbors.push((neighbor, grid[row][col - 1]));
//     }
//     // Down
//     if row + 1 < rows {
//         let neighbor = IVec2::new(col as i32, (row + 1) as i32);
//         neighbors.push((neighbor, grid[row + 1][col]));
//     }
//     // Up
//     if row > 0 {
//         let neighbor = IVec2::new(col as i32, (row - 1) as i32);
//         neighbors.push((neighbor, grid[row - 1][col]));
//     }

//     neighbors
// }

// fn count_char_edges(grid: &Vec<Vec<char>>) -> HashMap<char, (usize, usize)> {
//     let mut char_map: HashMap<char, (usize, usize)> = HashMap::new();
    
//     for row in 0..grid.len() {
//         for col in 0..grid[0].len() {
//             let current_char = grid[row][col];
//             let current_pos = IVec2::new(col as i32, row as i32);
//             let entry = char_map.entry(current_char).or_insert((0, 0));
            
//             // Increment character count
//             entry.0 += 1;
            
//             // Count all neighboring edges
//             for (_, neighbor_char) in get_all_neighbors(current_pos, &grid) {
//                 if neighbor_char != current_char {
//                     entry.1 += 1;
//                 }
//             }
            
//             // From borders
//             if col == 0 || col == grid[0].len() - 1 {
//                 entry.1 += 1;
//             }
//             if row == 0 || row == grid.len() - 1 {
//                 entry.1 += 1;
//             }
//         }
//     }
    
//     char_map
// }

// #[tracing::instrument]
// pub fn process(input: &str) -> miette::Result<String, AocError> {
//     let grid: Vec<Vec<char>> = input.lines()
//         .map(|line| line.chars().collect())
//         .collect();
    
//     let char_edges = count_char_edges(&grid);
    
//     // Debug print to see our counts
//     dbg!(&char_edges);
    
//     let total: usize = char_edges.iter()
//         .map(|(_, (count, edges))| count * edges)
//         .sum();
    
//     Ok(total.to_string())
// }

// // First we'll count characters and their edges
// fn count_char_edges(grid: &Vec<Vec<char>>) -> HashMap<char, (usize, usize)> {
//     let mut char_map: HashMap<char, (usize, usize)> = HashMap::new();
    
//     for row in 0..grid.len() {
//         for col in 0..grid[0].len() {
//             let current_char = grid[row][col];
//             let current_pos = IVec2::new(col as i32, row as i32);
//             let entry = char_map.entry(current_char).or_insert((0, 0));
            
//             // Increment character count
//             entry.0 += 1;
            
//             // Count edges
//             // From neighbors
//             for (_, neighbor_char) in get_forward_neighbors(current_pos, &grid) {
//                 if neighbor_char != current_char {
//                     entry.1 += 1;
//                 }
//             }
            
//             // From borders
//             if col == 0 || col == grid[0].len() - 1 {
//                 entry.1 += 1;
//             }
//             if row == 0 || row == grid.len() - 1 {
//                 entry.1 += 1;
//             }
//         }
//     }
    
//     char_map
// }

// pub fn process(input: &str) -> miette::Result<String, AocError> {
//     let grid: Vec<Vec<char>> = input.lines()
//         .map(|line| line.chars().collect())
//         .collect();
    
//     let char_edges = count_char_edges(&grid);
    
//     // Calculate total sum
//     let total: usize = char_edges.iter()
//         .map(|(_, (count, edges))| count * edges)
//         .sum();
    
//     Ok(total.to_string())
// }


// fn get_forward_neighbors(pos: IVec2, grid: &Vec<Vec<char>>) -> Vec<(IVec2, char)> {
//     let mut neighbors = Vec::new();
//     let cols = grid[0].len();
//     let rows = grid.len();
//     let col = pos.x as usize;
//     let row = pos.y as usize;
    
//     // Check right
//     if col + 1 < cols {
//         let neighbor = IVec2::new((col + 1) as i32, row as i32);
//         neighbors.push((neighbor, grid[row][col + 1]));
//     }
    
//     // Check down
//     if row + 1 < rows {
//         let neighbor = IVec2::new(col as i32, (row + 1) as i32);
//         neighbors.push((neighbor, grid[row + 1][col]));
//     }

//     neighbors
// }

// #[tracing::instrument]
// pub fn process(input: &str) -> miette::Result<String, AocError> {
//     let grid: Vec<Vec<char>> = input.lines()
//         .map(|line| line.chars().collect())
//         .collect();
    
//     let mut edges = 0;
//     for row in 0..grid.len() {
//         for col in 0..grid[0].len() {
//             let current_pos = IVec2::new(col as i32, row as i32);
//             let current_char = grid[row][col];
            
//             // Count edges from neighbors
//             for (_, neighbor_char) in get_forward_neighbors(current_pos, &grid) {
//                 if neighbor_char != current_char {
//                     edges += 1;
//                 }
//             }
            
//             // Count border edges
//             if col == 0 || col == grid[0].len() - 1 {
//                 edges += 1;
//             }
//             if row == 0 || row == grid.len() - 1 {
//                 edges += 1;
//             }
//         }
//     }

//     Ok(edges.to_string())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1930", process(input)?);
        Ok(())
    }
}

// // use std::collections::HashMap;

// use glam::IVec2;

// use crate::custom_error::AocError;

// fn get_forward_neighbors(pos: IVec2, grid: &Vec<Vec<char>>) -> Vec<(IVec2, char)> {
//     let mut neighbors = Vec::new();
//     let cols = grid[0].len();
//     let rows = grid.len();
//     let col = pos.x as usize;
//     let row = pos.y as usize;
    
//     // We only need to check right and down to avoid double counting
//     // Check right (increase X)
//     if col + 1 < cols {
//         let neighbor = IVec2::new((col + 1) as i32, row as i32);
//         neighbors.push((neighbor, grid[row][col + 1]));
//     }
    
//     // Check down (increase Y)
//     if row + 1 < rows {
//         let neighbor = IVec2::new(col as i32, (row + 1) as i32);
//         neighbors.push((neighbor, grid[row + 1][col]));
//     }

//     neighbors
// }

// fn count_edges(grid: &Vec<Vec<char>>) -> usize {
//     let rows = grid.len();
//     let cols = grid[0].len();
//     let mut edges = 0;

//     for r in 0..rows {
//         for c in 0..cols {
//             let current = grid[r][c];
            
//             // Each position where get_neighbors returns a different character is an edge
//             for (nr, nc) in get_forward_neighbors(r, c, rows, cols) {
//                 if grid[nr][nc] != current {
//                     edges += 1;
//                 }
//             }

//             // Add edges for borders
//             if r == 0 || r == rows - 1 || c == 0 || c == cols - 1 {
//                 edges += match (r, c) {
//                     (0, 0) => 2,                    // top-left corner
//                     (0, c) if c == cols-1 => 2,     // top-right corner
//                     (r, 0) if r == rows-1 => 2,     // bottom-left corner
//                     (r, c) if r == rows-1 && c == cols-1 => 2, // bottom-right corner
//                     (0, _) | (_, 0) | (r, _) if r == rows-1 
//                         | (_, c) if c == cols-1 => 1, // edges
//                     _ => 0,                         // interior
//                 };
//             }
//         }
//     }

//     edges
// }

// #[tracing::instrument]
// pub fn process(input: &str) -> miette::Result<String, AocError> {
//     let grid: Vec<Vec<char>> = input.lines()
//         .map(|line| line.chars().collect())
//         .collect();
    
//     let mut edges = 0;
//     for row in 0..grid.len() {
//         for col in 0..grid[0].len() {
//             let current_pos = IVec2::new(col as i32, row as i32);
//             let current_char = grid[row][col];
            
//             // Count edges from neighbors
//             for (_, neighbor_char) in get_forward_neighbors(current_pos, &grid) {
//                 if neighbor_char != current_char {
//                     edges += 1;
//                 }
//             }
            
//             // Count border edges
//             if col == 0 || col == grid[0].len() - 1 {
//                 edges += 1;
//             }
//             if row == 0 || row == grid.len() - 1 {
//                 edges += 1;
//             }
//         }
//     }

//     Ok(edges.to_string())
// }
// // pub fn process(input: &str) -> miette::Result<String, AocError> {
// //     // Convert input to 2D grid
// //     let grid: Vec<Vec<char>> = input.lines()
// //         .map(|line| line.chars().collect())
// //         .collect();

// //     let total_edges = count_edges(&grid);
    
// //     Ok(total_edges.to_string())
// // }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // use rstest::rstest;

//     // #[rstest]
//     // #[case("", "")]
//     // fn test_cases(#[case] input: &str, #[case] expected: &str) {
//     //     assert_eq!(process(input).unwrap(), expected);
//     // }

//     #[test]
//     fn test_process() -> miette::Result<()> {
//         let input = "RRRRIICCFF
// RRRRIICCCF
// VVRRRCCFFF
// VVRCCCJFFF
// VVVVCJJCFE
// VVIVCCJJEE
// VVIIICJJEE
// MIIIIIJJEE
// MIIISIJEEE
// MMMISSJEEE";
//         assert_eq!("1930", process(input)?);
//         Ok(())
//     }
// }
