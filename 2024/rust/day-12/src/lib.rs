use ornaments::{AocError, Solution, UniquePositions};

pub mod custom_error;

pub mod part1;
pub mod part2;

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
                let region = grid.flood_fill(pos, &mut visited);
                let target = grid.get_at_unbounded(pos);
                let count = region.len();
                let edges = grid.count_region_edges(region);
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
        assert_eq!("1930", Day12::parse(input).solve(Part::One)?);
        Ok(())
    }
}