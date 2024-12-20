use std::collections::{HashMap, VecDeque};

use crate::{Grid, DIRECTIONS};

// #[tracing::instrument]
pub fn process(input: &str, target_ps: i32) -> miette::Result<String, crate::AocError> {
    let grid = Grid(input.lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>());

    let rows = grid.get_height();
    let cols = grid.get_width();

    let start = grid.to_position(input.find("S").unwrap());
    // let start = Position::ZERO;
    let _end = grid.to_position(input.find("E").unwrap());
    let maze = grid.to_maze('#');

    // Track distances using BFS
    // let mut track: Something::<i32> = Something::new();
    let mut track = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    track.insert(start, 0);

    while let Some((pos, steps)) = queue.pop_front() {
        for dir in DIRECTIONS {
            let next = pos + dir;
            if next.x >= 0 && next.x < cols as i32 && 
               next.y >= 0 && next.y < rows as i32 &&
               !track.contains_key(&next) && !maze.contains(&next) {
                track.insert(next, steps + 1);
                queue.push_back((next, steps + 1));
            }
        }
    }

    let mut count = 0;
    for (&pos, &steps) in &track {
        for dir in DIRECTIONS {
            let wall_pos = pos + dir;
            let two_away = pos + dir * 2;
            
            if !track.contains_key(&wall_pos) && 
               track.contains_key(&two_away) && 
               track[&two_away] - steps >= target_ps + 2 {
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
//     #[case("2", "14")]
//     // #[case("4", "14")]
//     // #[case("6", "2")]
//     // #[case("8", "4")]
//     // #[case("10", "2")]
//     // #[case("12", "3")]
//     // #[case("20", "1")]
//     // #[case("36", "1")]
//     // #[case("38", "1")]
//     // #[case("40", "1")]
//     // #[case("64", "1")]
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

//         assert_eq!(process(map, input.parse::<i32>().unwrap()).unwrap(), expected);
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
