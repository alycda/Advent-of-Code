use std::collections::HashSet;

use glam::IVec2;
use ornaments::{AocError, Grid, Position, Solution, UniquePositions, DIRECTIONS};

pub use crate::Day12 as Day;

#[deprecated(note = "use `Grid::flood_fill(&self)` instead")]
fn flood_fill(grid: &Grid<char>, start: Position, visited: &mut UniquePositions) -> UniquePositions {
    let mut region = HashSet::new();
    let mut stack = vec![start];
    let target_char = grid[start.y as usize][start.x as usize];
    
    while let Some(pos) = stack.pop() {
        if !region.insert(pos) {
            continue;
        }
        visited.insert(pos);
        
        for neighbor in Grid::get_orthogonal_neighbors(grid, pos) {
            let neighbor_char = grid[neighbor.y as usize][neighbor.x as usize];
            if neighbor_char == target_char && !region.contains(&neighbor) {
                stack.push(neighbor);
            }
        }
    }
    
    region
}

#[deprecated]
fn explore(start: IVec2, grid: &Grid<char>, seen: &mut HashSet<IVec2>) -> (usize, usize) {
    let target_char = grid[start.y as usize][start.x as usize];
    let mut stack = vec![start];
    let mut region = HashSet::new();
    let mut edges = HashSet::new();
    let mut area = 0;

    // First pass: collect region and edges
    while let Some(pos) = stack.pop() {
        if !region.insert(pos) {
            continue;
        }
        seen.insert(pos);
        area += 1;

        for &neighbor in DIRECTIONS.iter() {

            let new_pos = pos + neighbor;
            
            if Grid::in_bounds(&grid, new_pos) && 
               grid[new_pos.y as usize][new_pos.x as usize] == target_char {
                if !region.contains(&new_pos) {
                    stack.push(new_pos);
                }
            } else {
                edges.insert(pos);
            }
        }
    }

    // Second pass: count unique sides
    let mut side_count = 0;
    for &neighbor in DIRECTIONS.iter() {
        let mut potential_side = HashSet::new();
        
        // Find potential sides in this direction
        for &pos in &region {
            let peek = pos + neighbor;
            if !region.contains(&peek) {
                potential_side.insert(peek);
            }
        }

        // Remove continuous edges
        let mut to_remove = HashSet::new();
        for &pos in &potential_side {
            let mut temp = IVec2::new(pos.x + neighbor.y, pos.y + neighbor.x);
            while potential_side.contains(&temp) {
                to_remove.insert(temp);
                temp = IVec2::new(temp.x + neighbor.y, temp.y + neighbor.x);
            }
        }

        side_count += potential_side.len() - to_remove.len();
    }

    (area, side_count)
}

/// char, count, edges
pub struct Day12(Grid<char>);

impl std::ops::Deref for Day12 {
    type Target = Grid<char>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Solution for Day12 {
    type Output = usize;
    type Item = ();

    fn parse(input: &str) -> Self {
        let grid = Day12::to_grid(input);


        Self(grid)
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        let mut visited = UniquePositions::new();
        let mut regions = Vec::new();

        self.walk(|pos| {
            if !visited.contains(&pos) {
                // let region = grid.flood_fill(pos, &mut visited);

                let region = flood_fill(&self, pos, &mut visited);
                let target = self.get_at_unbounded(pos);

                // dbg!(target);

                let count = region.len();

                // dbg!(count);
                let edges = self.count_region_edges(&region);
                regions.push((target, count, edges));
            }
        });

        let total: usize = regions.iter()
            .map(|(_, count, edges)| count * edges)
            .sum();
        
        Ok(total)
    }

    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        let mut seen = HashSet::new();
        let mut total = 0;

        self.walk(|pos| {
            if !seen.contains(&pos) {

                if !seen.contains(&pos) {
                    let (area, sides) = explore(pos, &self, &mut seen);
                    total += area * sides;
                }
            }
        });

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
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
        assert_eq!("1930", Day12::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
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
        assert_eq!("1206", Day12::parse(input).solve(Part::Two)?);
        Ok(())
    }
}