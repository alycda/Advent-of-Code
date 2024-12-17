use crate::{custom_error::AocError, Day12};

use std::collections::HashSet;
use glam::IVec2;
use ornaments::{Solution, UniquePositions};

fn count_region_edges(grid: &Vec<Vec<char>>, region: &HashSet<IVec2>) -> usize {
    let mut edges = 0;
    
    for &pos in region {
        for neighbor in crate::get_neighbors(pos, grid) {
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
    let grid = <Day12 as Solution>::to_grid(input);
    
    let mut visited = UniquePositions::new();
    let mut regions = Vec::new();
    
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let pos = IVec2::new(col as i32, row as i32);
            if !visited.contains(&pos) {
                let region = crate::flood_fill(&grid, pos, &mut visited);
                let char_type = grid[row][col];

                dbg!(char_type);

                let count = region.len();

                dbg!(count);
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