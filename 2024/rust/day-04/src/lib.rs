use ornaments::{AocError, Grid, Solution, ALL_DIRECTIONS};
use tracing::{debug, instrument};

use std::collections::HashSet;

use glam::IVec2;

pub mod custom_error;

pub mod part2;

// pub struct Day4(HashSet<IVec2>);

pub struct Day4 {
    grid: Grid,

    /// position of all the X's (Part 1)
    set: HashSet<IVec2>
}

// impl std::ops::Deref for Day4 {
//     type Target = HashSet<IVec2>;
    
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

impl Solution for Day4 {
    type Output = usize;
    type Item = HashSet<IVec2>; //HashMap<IVec2, Direction>

    fn parse(input: &'static str) -> Self {
        // let grid = <Day4 as Solution>::to_grid(input);
        let grid = Day4::to_grid(input);
        let mut x_positions: Self::Item = HashSet::new();

        grid.walk(|pos| {
            if grid.get_at(pos).unwrap() == 'X' {
                x_positions.insert(pos);
            }
        });

        assert!(!x_positions.is_empty());

        Self {
            grid,
            set: x_positions
        }
    }

    // #[instrument]
    fn part1(&mut self) -> Result<Self::Output, AocError> {
        let output = self.set.iter()
            .flat_map(|pos| {
                debug!("Found X at {pos:?}");

                ALL_DIRECTIONS.iter().filter_map(|dir| {
                    self.grid.go_straight(*pos, *dir, 3)
                })
            })
            .filter(|new_thing| {
                // "MAS" == dbg!(new_thing.iter().collect::<String>())
                "MAS" == new_thing.iter().collect::<String>()
            })
            .count();

        Ok(output)
    }

//     fn part2(&mut self) -> Result<Self::Output, AocError> {
//         Ok(part2::count_valid(input))
//     }
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
        assert_eq!("18", Day4::parse(input).solve(Part::One)?);
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
//         assert_eq!("9", Day4::parse(input).solve(Part::Two)?);
//         Ok(())
//     }
}
