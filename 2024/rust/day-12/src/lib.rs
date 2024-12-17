use glam::IVec2;
use ornaments::{AocError, Solution, UniquePositions};

pub mod custom_error;

pub mod part2;

fn get_neighbors(pos: IVec2, grid: &Vec<Vec<char>>) -> Vec<IVec2> {
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

fn flood_fill(grid: &Vec<Vec<char>>, start: glam::IVec2, visited: &mut std::collections::HashSet<IVec2>) -> std::collections::HashSet<IVec2> {
    let mut region = std::collections::HashSet::new();
    let mut stack = vec![start];
    let target_char = grid[start.y as usize][start.x as usize];
    
    while let Some(pos) = stack.pop() {
        if !region.insert(pos) {
            continue;
        }
        visited.insert(pos);
        
        for neighbor in get_neighbors(pos, grid) {
            let neighbor_char = grid[neighbor.y as usize][neighbor.x as usize];
            if neighbor_char == target_char && !region.contains(&neighbor) {
                stack.push(neighbor);
            }
        }
    }
    
    region
}

/// char, count, edges
pub struct Day12(Vec<(char, usize, usize)>);

impl std::ops::Deref for Day12 {
    type Target = Vec<(char, usize, usize)>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Solution for Day12 {
    type Output = usize;
    type Item = ();

    fn parse(input: &str) -> Self {
        let grid = Day12::to_grid(input);

        let mut visited = UniquePositions::new();
        let mut regions = Vec::new();

        grid.walk(|pos| {
            if !visited.contains(&pos) {
                // let region = grid.flood_fill(pos, &mut visited);

                let region = flood_fill(&grid, pos, &mut visited);
                let target = grid.get_at_unbounded(pos);

                // dbg!(target);

                let count = region.len();

                // dbg!(count);
                let edges = grid.count_region_edges(&region);
                regions.push((target, count, edges));
            }
        });

        Self(regions)
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        let total: usize = self.iter()
            .map(|(_, count, edges)| count * edges)
            .sum();
        
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