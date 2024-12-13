use std::marker::PhantomData;
use std::collections::HashSet;
use glam::IVec2;
use ornaments::{AocError, Grid, Solution, ALL_DIRECTIONS};

pub trait Pattern {
    fn matches(&self, grid: &Grid, pos: IVec2) -> bool;
    fn find_starting_positions(grid: &Grid) -> HashSet<IVec2>;
}

#[derive(Default)]
pub struct XmasPattern;

#[derive(Default)]
pub struct CrossPattern;

impl Pattern for XmasPattern {
    fn matches(&self, grid: &Grid, pos: IVec2) -> bool {
        ALL_DIRECTIONS.iter().any(|dir| {
            if let Some(chars) = grid.go_straight(pos, *dir, 3) {
                chars.iter().collect::<String>() == "MAS"
            } else {
                false
            }
        })
    }

    fn find_starting_positions(grid: &Grid) -> HashSet<IVec2> {
        let mut positions = HashSet::new();
        grid.walk(|pos| {
            if grid.get_at(pos).unwrap() == 'X' {
                positions.insert(pos);
            }
        });
        positions
    }
}

impl Pattern for CrossPattern {
    fn matches(&self, grid: &Grid, pos: IVec2) -> bool {
        let diagonals = [
            (IVec2::new(1, 1), IVec2::new(-1, -1)),
            (IVec2::new(-1, 1), IVec2::new(1, -1)),
        ];
        
        diagonals.iter().any(|(dir1, dir2)| {
            let path1 = grid.go_straight(pos, *dir1, 3);
            let path2 = grid.go_straight(pos, *dir2, 3);
            
            matches!((path1, path2), (Some(p1), Some(p2)) 
                if p1.iter().collect::<String>() == "MAS" 
                && p2.iter().collect::<String>() == "MAS")
        })
    }

    fn find_starting_positions(grid: &Grid) -> HashSet<IVec2> {
        let mut positions = HashSet::new();
        grid.walk(|pos| {
            if grid.get_at(pos).unwrap() == 'M' {
                positions.insert(pos);
            }
        });
        positions
    }
}

pub struct Day4<P> {
    grid: Grid,
    set: HashSet<IVec2>,
    _pattern: PhantomData<P>
}

impl<P: Pattern + Default> Solution for Day4<P> {
    type Output = usize;
    type Item = HashSet<IVec2>;

    fn parse(input: &'static str) -> Self {
        let grid = Self::to_grid(input);
        let set = P::find_starting_positions(&grid);
        assert!(!set.is_empty());
        
        Self {
            grid,
            set,
            _pattern: PhantomData
        }
    }

    fn part1(&mut self) -> Result<Self::Output, AocError> {
        let pattern = P::default();
        let output = self.set.iter()
            .filter(|&pos| pattern.matches(&self.grid, *pos))
            .count();
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", Day4::<XmasPattern>::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", Day4::<CrossPattern>::parse(input).solve(Part::Two)?);
        Ok(())
    }
}
