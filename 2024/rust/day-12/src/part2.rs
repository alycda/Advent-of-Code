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
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    for &pos in region {
        // Count horizontal edges (left/right)
        if pos.x == 0 || !region.contains(&IVec2::new(pos.x - 1, pos.y)) {
            edge_count += 1;
        }
        if pos.x == cols - 1 || !region.contains(&IVec2::new(pos.x + 1, pos.y)) {
            edge_count += 1;
        }

        // Count vertical edges (up/down)
        if pos.y == 0 || !region.contains(&IVec2::new(pos.x, pos.y - 1)) {
            edge_count += 1;
        }
        if pos.y == rows - 1 || !region.contains(&IVec2::new(pos.x, pos.y + 1)) {
            edge_count += 1;
        }
    }

    // Since we counted each edge twice (once from each side), divide by 2
    edge_count / 2
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
