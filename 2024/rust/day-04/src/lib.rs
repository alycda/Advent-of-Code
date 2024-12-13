use std::{collections::HashSet, marker::PhantomData};

use glam::IVec2;
use ornaments::{AocError, Grid, Solution, ALL_DIRECTIONS};

pub trait Pattern: Default {
    fn matches(&self, chars: &[char]) -> bool;
}

pub struct XmasPattern;
pub struct CrossPattern;

impl Default for XmasPattern {
    fn default() -> Self { Self }
}

impl Pattern for XmasPattern {
    fn matches(&self, chars: &[char]) -> bool {
        chars.iter().collect::<String>() == "MAS"
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
        let mut x_positions = HashSet::new();

        grid.walk(|pos| {
            if grid.get_at(pos).unwrap() == 'X' {
                x_positions.insert(pos);
            }
        });

        assert!(!x_positions.is_empty());

        Self {
            grid,
            set: x_positions,
            _pattern: PhantomData
        }
    }

    fn part1(&mut self) -> Result<Self::Output, AocError> {
        let pattern = P::default();
        let output = self.set.iter()
            .flat_map(|pos| {
                ALL_DIRECTIONS.iter().filter_map(|dir| {
                    self.grid.go_straight(*pos, *dir, 3)
                })
            })
            .filter(|chars| pattern.matches(chars))
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

//     #[test]
//     fn test_part2() -> miette::Result<()> {
//         let input = "MMMSXXMASM
// MSAMXMSMSA
// AMXSXMAAMM
// MSAMASMSMX
// XMASAMXAMM
// XXAMMXXAMA
// SMSMSASXSS
// SAXAMASAAA
// MAMMMXMMMM
// MXMXAXMASX";
//         assert_eq!("9", Day4::<CrossPattern>::parse(input).solve(Part::Two)?);
//         Ok(())
//     }
}
