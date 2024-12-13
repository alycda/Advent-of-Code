use ornaments::{Solution, AocError};

use std::collections::HashSet;

use glam::IVec2;

pub mod custom_error;

pub mod part1;
pub mod part2;

pub struct Day4(HashSet<IVec2>);

// pub struct Day4 {
//     grid: Grid,
//     set: HashSet<IVec2>
// }

impl std::ops::Deref for Day4 {
    type Target = HashSet<IVec2>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Solution for Day4 {
    type Output = usize;
    type Item = HashSet<IVec2>; //HashMap<IVec2, Direction>

    fn parse(input: &'static str) -> Self {
        // let grid = <Day4 as Solution>::to_grid(input);
        let grid = Day4::to_grid(input);
        let mut x_positions: Self::Item = HashSet::new();

        // dbg!(&grid);

        // for row in 0..grid.len() {
        //     for col in 0..grid[0].len() {
        //         let pos = IVec2::new(col as i32, row as i32);

        //         if grid[row][col] == 'X' {
        //             x_positions.insert(pos);
        //         }
        //     }
        // }

        grid.walk(|pos| {
            if grid.get_at(pos) == 'X' {
                x_positions.insert(pos);
            }
        });

        assert!(!x_positions.is_empty());

        // Self(dbg!(x_positions))
        Self(x_positions)
    }

    fn part1(&mut self) -> Result<Self::Output, AocError> {

        // let grid = Day4::to_grid(input);

        self.iter().for_each(|pos| {
            dbg!(pos);
        });

        Ok(0)
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
