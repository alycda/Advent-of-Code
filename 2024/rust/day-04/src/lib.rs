//! Day 4: Ceres Search
//! 
//! Word Search
//! 
//! --- Part One ---
//! 
//! find XMAS in all 8 directions
//! 
//! --- Part Two ---
//! 
//! find two MAS in the shape of an X
//! 

use std::marker::PhantomData;

use ornaments::{AocError, Grid, Position, Solution, UniquePositions, ALL_DIRECTIONS, DIAGONALS};

pub use crate::Day4 as Day;

pub trait Pattern: Default {
    fn matches(&self, grid: &Grid<char>, pos: Position) -> bool;
    fn find_starting_positions(grid: &Grid<char>) -> UniquePositions;
}

/// Pattern implementation for finding "XMAS" strings
/// Starting with 'X' and searching in all directions
#[derive(Default)]
pub struct XmasPattern;

/// Pattern implementation for finding crossed "MAS" patterns
/// Forms an X shape with specific character arrangements
#[derive(Default)]
pub struct CrossPattern;

impl Pattern for XmasPattern {
    fn matches(&self, grid: &Grid<char>, pos: Position) -> bool {
        if grid.get_at(pos).unwrap() != 'X' {
            return false;
        }

        ALL_DIRECTIONS.iter()
            .filter_map(|dir| {
                grid.go_straight(pos, *dir, 3)
            })
            .any(|chars| {
                chars.iter().collect::<String>() == "MAS"
            })
    }

    /// Finds all 'X' characters in the grid
    fn find_starting_positions(grid: &Grid<char>) -> UniquePositions {
        let mut positions = UniquePositions::new();
        grid.walk(|pos| {
            if grid.get_at(pos).unwrap() == 'X' {
                positions.insert(pos);
            }
        });
        positions
    }
}

impl Pattern for CrossPattern {
    /// Checks diagonal positions for valid "MAS" cross patterns
    fn matches(&self, grid: &Grid<char>, pos: Position) -> bool {        
        let chars: Vec<char> = DIAGONALS.iter()
            .filter_map(|&dir| {
                let new_pos = pos + dir;
                grid.get_at(new_pos)
            })
            .collect();

        if chars.len() != 4 {
            return false;
        }

        matches!(&chars[..], 
            &['M', 'S', 'M', 'S'] | 
            &['S', 'M', 'S', 'M'] | 
            &['M', 'M', 'S', 'S'] |
            &['S', 'S', 'M', 'M'])
    }

    /// Finds all 'A' characters in the grid
    fn find_starting_positions(grid: &Grid<char>) -> UniquePositions {
        let mut positions = UniquePositions::new();
        grid.walk(|pos| {
            if grid.get_at(pos).unwrap() == 'A' {
                positions.insert(pos);
            }
        });
        positions
    }
}

/// Using generics and PhantomData for parsing flexibility
pub struct Day4<P: Pattern> {
    grid: Grid<char>,
    set: UniquePositions,
    /// only used for type checking, not actual ownership
    _pattern: PhantomData<P>
}

impl<P: Pattern> Solution for Day<P> {
    type Output = usize;
    type Item = UniquePositions;

    fn parse(input: &str) -> Self {
        let grid = Self::to_grid(input);
        let set = P::find_starting_positions(&grid);
        
        Self {
            grid,
            set,
            _pattern: PhantomData
        }
    }

    /// Counts occurrences of "MAS" pattern in all directions
    fn part1(&mut self) -> Result<Self::Output, AocError> {
        // XmasPattern impl maintains original functionality
        let output = self.set.iter()
            .flat_map(|pos| {
                ALL_DIRECTIONS.iter().filter_map(|dir| {
                    self.grid.go_straight(*pos, *dir, 3)
                })
            })
            .filter(|chars| {
                chars.iter().collect::<String>() == "MAS"
            })
            .count();

        Ok(output)
    }

    /// Counts occurrences of "MAS" pattern in diagonals
    fn part2(&mut self) -> Result<Self::Output, AocError> {
        // Create temporary Pattern instance just for matching
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
