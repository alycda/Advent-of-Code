use std::collections::{HashMap, HashSet};

use glam::IVec2;

use crate::custom_error::AocError;

// fn visualize_grid(positions: &[IVec2], grid_size: IVec2) -> String {
//     let mut output = String::new();
    
//     for y in 0..grid_size.y {
//         for x in 0..grid_size.x {
//             let current = IVec2::new(x, y);
//             if positions.contains(&current) {
//                 output.push('T');
//             } else {
//                 output.push('.');
//             }
//         }
//         output.push('\n');
//     }
    
//     output
// }

fn visualize_grid(positions: &HashSet<IVec2>, grid_size: IVec2) -> String {
    let mut output = String::new();
    
    for y in 0..grid_size.y {
        for x in 0..grid_size.x {
            let current = IVec2::new(x, y);
            if positions.contains(&current) {
                output.push('T');
            } else {
                output.push('.');
            }
        }
        output.push('\n');
    }
    
    output
}

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
        // .inspect(|(position, c)| {
        //     dbg!(position, c);
        // })
        .fold(HashMap::new(), |mut map, (position, c)| {
            map.entry(c).or_insert_with(Vec::new).push(position);
            map
        });

    // ignore any single antennas
    antennas.retain(|_k, v| v.len() != 1);

    // dbg!(&antennas);

    let offsets_by_char = antennas.iter()
    .flat_map(|(_k, v)| {
        (0..v.len()).flat_map(move |i| {
            v[i+1..].iter().flat_map(move |second| {
                let first = v[i];
                let delta = *second - first;
                let grid = grid; // Capture grid by value here
                
                // Generate all positions in both directions
                let mut positions = Vec::new();
                
                // Forward direction
                let mut pos = first;
                while pos.x >= 0 && pos.x < grid.x && pos.y >= 0 && pos.y < grid.y {
                    positions.push(pos);
                    pos += delta;
                }
                
                // Backward direction
                let mut pos = first;
                pos -= delta; // Start one step back
                while pos.x >= 0 && pos.x < grid.x && pos.y >= 0 && pos.y < grid.y {
                    positions.push(pos);
                    pos -= delta;
                }
                
                positions
            })
        })
    })
        // .flat_map(|(_k, v)| {
        //     (0..v.len()).flat_map(move |i| {
        //         v[i+1..].iter().flat_map(move |second| {
        //             let first = v[i];
        //             let delta = *second - first;
                    
        //             // Get positions in both directions from first antenna
        //             let forward = (0..).map(move |i| first + (delta * i))
        //                 .take_while(|pos| {
        //                     pos.x >= 0 && pos.x < grid.x && 
        //                     pos.y >= 0 && pos.y < grid.y
        //                 });
                    
        //             let backward = (1..).map(move |i| first - (delta * i))
        //                 .take_while(|pos| {
        //                     pos.x >= 0 && pos.x < grid.x && 
        //                     pos.y >= 0 && pos.y < grid.y
        //                 });
                    
        //             forward.chain(backward)
        //         })
        //     })
        // })
        // .flat_map(|(k, v)| {
        //     v.iter().flat_map(move |antenna| {
        //         v.iter()
        //             .filter(move |x| *x != antenna)
        //             .map(move |other| (k, antenna, (antenna - other)))
        //             // .inspect(|(c, antenna, offset)| {
        //             //     dbg!(c, antenna, offset);
        //             // })
        //     })
        // })
        // // .inspect(|new_pos| {
        // //     dbg!(new_pos);
        // // })
        // // .flat_map(|(_, a, b)| {
        // //     (1..).map(move |i| *a + (b * i))  // Generate infinite sequence of positions
        // //         .take_while(|pos| {            // Stop when we hit grid bounds
        // //             pos.x >= 0 && pos.x < grid.x && 
        // //             pos.y >= 0 && pos.y < grid.y
        // //         })
        // // })
        // .flat_map(|(_, a, b)| {
        //     // Start from the second antenna and move in the direction
        //     (0..).map(move |i| *a + (b * (i + 1)))  // Note the i + 1 to start after the antenna
        //     .take_while(|pos| {
        //         pos.x >= 0 && pos.x < grid.x && 
        //         pos.y >= 0 && pos.y < grid.y
        //     })
        // })
        // // .inspect(|new_pos| {
        // //     dbg!(new_pos);
        // // })
        // // .fold(HashSet::new(), |mut set, offset| {
        // //     set.insert(offset);
        // //     set
        // // });
        // // .count();
        .collect::<HashSet<_>>()  // Collect into a HashSet to deduplicate positions
.len();

    // let grid_str = visualize_grid(&offsets_by_char, grid);
    // println!("{}", grid_str);

    // dbg!(offsets_by_char.len());
    // dbg!(offsets_by_char);

    // panic!("halt");

    let z = antennas.iter().flat_map(|(_k, v)| {
        v
    }).count();

    // panic!("{:?}", z);

    // panic!("{:?}", antennas);

    Ok((offsets_by_char).to_string())
    // Ok((offsets_by_char + antennas.len()).to_string())
    // Ok((offsets_by_char + z).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........", "9")]
//     #[case("............
// ........0...
// .....0......
// .......0....
// ....0.......
// ......A.....
// ............
// ............
// ........A...
// .........A..
// ............
// ............", "34")]
    fn test_cases(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process(input).unwrap(), expected);
    }

//     #[test]
//     fn test_process() -> miette::Result<()> {
//         let input = "............
// ........0...
// .....0......
// .......0....
// ....0.......
// ......A.....
// ............
// ............
// ........A...
// .........A..
// ............
// ............";
//         assert_eq!("34", process(input)?);
//         Ok(())
    // }
}
