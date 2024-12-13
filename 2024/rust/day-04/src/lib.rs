use std::{collections::HashSet, marker::PhantomData};

use glam::IVec2;
use ornaments::{AocError, Grid, Solution, ALL_DIRECTIONS};

pub trait Pattern: Default {
    fn matches(&self, chars: &Grid, pos: IVec2) -> bool;
    fn find_starting_positions(grid: &Grid) -> HashSet<IVec2>;
}

#[derive(Default)]
pub struct XmasPattern;

#[derive(Default)]
pub struct CrossPattern;

impl Pattern for XmasPattern {
    fn matches(&self, grid: &Grid, pos: IVec2) -> bool {
        ALL_DIRECTIONS.iter()
            .filter_map(|dir| grid.go_straight(pos, *dir, 3))
            .any(|chars| chars.iter().collect::<String>() == "MAS")
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
        if grid.get_at(pos).unwrap() != 'A' {
            return false;
        }

        let diagonals = [
            (IVec2::new(-1, -1), IVec2::new(-1, 1)), // TL, TR
            (IVec2::new(1, -1), IVec2::new(1, 1)),   // BL, BR
        ];

        diagonals.iter().any(|(dir1, dir2)| {
            let pair1 = grid.go_straight(pos, *dir1, 1)
                .and_then(|v| v.first().copied());
            let pair2 = grid.go_straight(pos, *dir2, 1)
                .and_then(|v| v.first().copied());

            matches!((pair1, pair2), 
                (Some('M'), Some('S')) |
                (Some('S'), Some('M')))
        })
    }

    fn find_starting_positions(grid: &Grid) -> HashSet<IVec2> {
        let mut positions = HashSet::new();
        grid.walk(|pos| {
            if grid.get_at(pos).unwrap() == 'A' {
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

impl<P: Pattern> Solution for Day4<P> {
    type Output = usize;
    type Item = HashSet<IVec2>;

    fn parse(input: &'static str) -> Self {
        let grid = Self::to_grid(input);
        let set = P::find_starting_positions(&grid);
        
        Self {
            grid,
            set,
            _pattern: PhantomData
        }
    }

    fn part1(&mut self) -> Result<Self::Output, AocError> {
        let pattern = P::default();
        Ok(self.set.iter()
            .filter(|&pos| pattern.matches(&self.grid, *pos))
            .count())
    }

    fn part2(&mut self) -> Result<Self::Output, AocError> {
        let pattern = P::default();
        Ok(self.set.iter()
            .filter(|&pos| pattern.matches(&self.grid, *pos))
            .count())
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
