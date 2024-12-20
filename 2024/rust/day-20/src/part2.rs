use std::collections::{HashMap, VecDeque};

use crate::{AocError, Position, DIRECTIONS};

// #[tracing::instrument]
pub fn process(input: &str, target_ps: usize) -> miette::Result<String, AocError> {
    // Build map of non-wall positions and their distances
    // let mut grid: Something::<i32> = Something::new();
    let mut grid = HashMap::new();
    let mut start = Position::ZERO;
    
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '#' {
                grid.insert(Position::new(x as i32, y as i32), c);
                if c == 'S' {
                    start = Position::new(x as i32, y as i32);
                }
            }
        }
    }

    // Calculate distances from start using BFS
    let mut dist = HashMap::new();
    let mut todo = VecDeque::new();
    dist.insert(start, 0);
    todo.push_back(start);

    while let Some(pos) = todo.pop_front() {
        for dir in DIRECTIONS {
            let next = pos + dir;
            if grid.contains_key(&next) && !dist.contains_key(&next) {
                dist.insert(next, dist[&pos] + 1);
                todo.push_back(next);
            }
        }
    }

    // Count paths that save enough time
    let mut count = 0;
    for (&p, &p_dist) in &dist {
        for (&q, &q_dist) in &dist {
            let d = (p.x - q.x).abs() + (p.y - q.y).abs();
            // For part 2: d < 21 instead of d == 2
            if d < 21 && p_dist - q_dist - d >= target_ps as i32 {
                count += 1;
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
