use crate::custom_error::AocError;

use std::collections::HashSet;
use glam::IVec2;

enum Direction {
    North,
    South,
    East,
    West
}

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
    let mut edge_count = 0;
    // let mut visited_positions = HashSet::new();

    // Pre-compute grid boundaries
    let max_x = (grid[0].len() - 1) as i32;
    let max_y = (grid.len() - 1) as i32;

    // Find the bounding box of the region to reduce iteration space
    let min_x = region.iter().map(|p| p.x).min().unwrap_or(0);
    let max_region_x = region.iter().map(|p| p.x).max().unwrap_or(max_x);
    let min_y = region.iter().map(|p| p.y).min().unwrap_or(0);
    let max_region_y = region.iter().map(|p| p.y).max().unwrap_or(max_y);

    // Process only the bounding box of the region
    for y in min_y..=max_region_y {
        let mut in_region = false;
        let mut last_char = None;

        for x in min_x..=max_region_x {
            let pos = IVec2::new(x, y);
            let is_in_region = region.contains(&pos);

            // Count vertical edges when we enter/exit the region
            if is_in_region != in_region {
                edge_count += 1;
                in_region = is_in_region;
            }

            // Count horizontal edges
            if is_in_region {
                // Top edge
                if y == 0 || !region.contains(&IVec2::new(x, y - 1)) {
                    if last_char != Some(true) {
                        edge_count += 1;
                        last_char = Some(true);
                    }
                } else {
                    last_char = None;
                }
            }
        }
    }

    // Add border edges if we touch the grid boundaries
    if region.iter().any(|p| p.x == 0 || p.x == max_x || p.y == 0 || p.y == max_y) {
        edge_count += 1;
    }

    edge_count
}

/// COLS, ROWS, GRID
#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid(usize, usize, String);

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
    // for (c, count, edges) in &regions {
    //     println!("Region of {} plants with price {} * {} = {}", 
    //         c, count, edges, count * edges);
    // }
    
    let total: usize = regions.iter()
        .map(|(_, count, edges)| count * edges)
        .sum();
    
    Ok(total.to_string())
}

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
        assert_eq!("1206", process(input)?);
        Ok(())
    }
}
