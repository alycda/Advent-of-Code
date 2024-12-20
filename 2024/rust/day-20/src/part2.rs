use std::collections::{HashMap, HashSet, VecDeque};

use crate::AocError;

use glam::*;

pub type Position = glam::IVec2;

/// Up, Right, Down, Left
pub const DIRECTIONS: [Position; 4] = [Position::NEG_Y, Position::X, Position::Y, Position::NEG_X];

// #[tracing::instrument]
pub fn process(input: &str, target_ps: usize) -> miette::Result<String, AocError> {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    // Find start and end (same as part 1)
    let mut start = IVec2::ZERO;
    let mut end = IVec2::ZERO;
    for (i, line) in lines.iter().enumerate() {
        if let Some(j) = line.find('S') {
            start = IVec2::new(j as i32, i as i32);
        }
        if let Some(j) = line.find('E') {
            end = IVec2::new(j as i32, i as i32);
        }
    }

    // Track distances using BFS (same as part 1)
    let mut track = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    track.insert(start, 0);

    while let Some((pos, steps)) = queue.pop_front() {
        for dir in DIRECTIONS {
            let next = pos + dir;
            if next.x >= 0 && next.x < cols as i32 && 
               next.y >= 0 && next.y < rows as i32 &&
               !track.contains_key(&next) &&
               lines[next.y as usize].chars().nth(next.x as usize).unwrap() != '#' {
                track.insert(next, steps + 1);
                queue.push_back((next, steps + 1));
            }
        }
    }

    let mut count = 0;
    for (&pos, &steps) in &track {
        for dy in -20..=20 {
            for dx in -20..=20 {
                let offset = IVec2::new(dx, dy);
                let manhattan_dist = offset.x.abs() + offset.y.abs();
                
                if manhattan_dist > 20 {
                    continue;
                }
                
                let target = pos + offset;
                
                // Check if target position is valid and in track
                if target.x >= 0 && target.x < cols as i32 && 
                   target.y >= 0 && target.y < rows as i32 && 
                   track.contains_key(&target) {
                    
                    // Check if path between pos and target has walls
                    let mut has_wall = false;
                    // Interpolate points along the path
                    for i in 1..manhattan_dist {
                        let check = pos + (offset * i) / manhattan_dist;
                        if check.x >= 0 && check.x < cols as i32 && 
                           check.y >= 0 && check.y < rows as i32 {
                            if lines[check.y as usize].chars().nth(check.x as usize).unwrap() == '#' {
                                has_wall = true;
                                break;
                            }
                        }
                    }
                    
                    if !has_wall {
                        let new_path = steps + manhattan_dist + track[&target];
                        let original = track[&end];
                        
                        if original - new_path >= target_ps as i32 {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

//     use rstest::rstest;

//     #[rstest]
//     #[case("50", "32")]
//     // #[case("52", "31")]
//     // #[case("54", "29")]
//     // #[case("56", "39")]
//     // #[case("58", "25")]
//     // #[case("60", "23")]
//     // #[case("62", "20")]
//     // #[case("64", "19")]
//     // #[case("66", "12")]
//     // #[case("68", "14")]
//     // #[case("70", "12")]
//     // #[case("72", "22")]
//     // #[case("74", "4")]
//     // #[case("76", "3")]
//     fn test_cases(#[case] input: &str, #[case] expected: &str) {
//         let map = "###############
// #...#...#.....#
// #.#.#.#.#.###.#
// #S#...#.#.#...#
// #######.#.#.###
// #######.#.#...#
// #######.#.###.#
// ###..E#...#...#
// ###.#######.###
// #...###...#...#
// #.#####.#.###.#
// #.#...#.#.#...#
// #.#.#.#.#.#.###
// #...#...#...###
// ###############";

//         assert_eq!(process(map, input.parse::<usize>().unwrap()).unwrap(), expected);
//     }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!("0", process(input, 100)?);
        Ok(())
    }
}
