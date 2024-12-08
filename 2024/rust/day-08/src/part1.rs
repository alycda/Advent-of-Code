use std::collections::{HashMap, HashSet};

use glam::IVec2;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut peekable = input.lines().peekable();
    let cols = peekable.peek().unwrap().chars().count();
    let rows = peekable.count();

    let grid = IVec2::new(cols as i32, rows as i32);

    let mut antennas = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line
                .chars()
                .enumerate()
                .filter_map(|(col, c)| {

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

    let offsets_by_char = antennas.iter()
        .flat_map(|(k, v)| {
            v.iter().flat_map(move |antenna| {
                v.iter()
                    .filter(move |x| *x != antenna)
                    .map(move |other| (k, antenna, (antenna - other)))
            })

        })
        .filter_map(|(_, a, b)|{
            let new_pos = *a + b;
            
            if new_pos.x >= 0 && new_pos.x < grid.x 
            && new_pos.y >= 0 && new_pos.y < grid.y {
                Some(new_pos)
            } else {
                None
            }
        })
        .inspect(|new_pos| {
            dbg!(new_pos);
        })
        .fold(HashSet::new(), |mut set, offset| {
            set.insert(offset);
            set
        });

    Ok(offsets_by_char.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
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
        assert_eq!("14", process(input)?);
        Ok(())
    }

//     #[test]
//     fn test_skip_antenna() -> miette::Result<()> {
//         let input = "..........
// ...#......
// #.........
// ....a.....
// ........a.
// .....a....
// ..#.......
// ......A...
// ..........
// ..........";
//         assert_eq!("4", process(input)?);
//         Ok(())
//     }
}
