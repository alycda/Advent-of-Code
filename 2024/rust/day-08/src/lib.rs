use std::collections::{HashMap, HashSet};

use glam::IVec2;
use ornaments::{AocError, Solution};

/// , grid max size
pub struct Day8(HashMap<char, Vec<IVec2>>, IVec2);

impl std::ops::Deref for Day8 {
    type Target = HashMap<char, Vec<IVec2>>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Solution for Day8 {
    type Output = usize;
    type Item = IVec2;

    fn parse(input: &'static str) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;

        let mut antennas = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line
                    .chars()
                    .enumerate()
                    .filter_map(|(col, c)| {
                        max_x = col;
                        max_y = row;

                        if c != '.' && c != '#' {
                            Some((IVec2::new(col as i32, row as i32), c))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(IVec2, char)>>()
            })
            .fold(HashMap::new(), |mut map, (position, c)| {
                map.entry(c).or_insert_with(Vec::new).push(position);
                map
            });

        // ignore any single antennas
        antennas.retain(|_k, v| v.len() != 1);

        Self(antennas, IVec2::new(max_x as i32, max_y as i32))
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        let offsets_by_char = self.iter()
            .flat_map(|(k, v)| {
                v.iter().flat_map(move |antenna| {
                    v.iter()
                        .filter(move |x| *x != antenna)
                        .map(move |other| (k, antenna, (antenna - other)))
                })

            })
            .filter_map(|(_, a, b)|{
                let new_pos = *a + b;
                
                if new_pos.x >= 0 && new_pos.x < self.1.x + 1 
                && new_pos.y >= 0 && new_pos.y < self.1.y + 1 {
                    Some(new_pos)
                } else {
                    None
                }
            })
            .fold(HashSet::new(), |mut set, offset| {
                set.insert(offset);
                set
            });

        Ok(offsets_by_char.len())
    }

    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", Day8::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("34", Day8::parse(input).solve(Part::Two)?);
        Ok(())
    }
}