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
    for (&pos, &start_steps) in &track {
        // For each point we reached, do a mini-BFS up to 20 steps
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((pos, 0));
        seen.insert(pos);
    
        while let Some((cur_pos, cheat_steps)) = queue.pop_front() {
            if cheat_steps >= 20 {
                continue;
            }
    
            // Try all directions
            for dir in DIRECTIONS {
                let next = cur_pos + dir;
                
                if next.x >= 0 && next.x < cols as i32 && 
                   next.y >= 0 && next.y < rows as i32 && 
                   !seen.contains(&next) {
                       
                    seen.insert(next);
                    
                    // If this point is in our original track
                    if let Some(&end_steps) = track.get(&next) {
                        let new_path = start_steps + cheat_steps + 1 + end_steps;
                        let original = track[&end];
                        
                        if original - new_path >= target_ps as i32 {
                            count += 1;
                        }
                    }
                    
                    queue.push_back((next, cheat_steps + 1));
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
