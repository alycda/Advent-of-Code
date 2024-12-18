use std::collections::{HashSet, VecDeque};

use glam::IVec2;

use crate::AocError;

#[derive(Debug)]
/// only stores the interesting positions and minmax bounds
pub struct PhantomGrid(pub HashSet<IVec2>, pub (IVec2, IVec2));

impl PhantomGrid {
    pub fn print(&self, steps: Option<HashSet<IVec2>>) {
        let obstacles = &self.0;
        let bounds = self.1;

        // let (obstacles, bounds) = self;
        let (min, max) = bounds;
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let pos = IVec2::new(x, y);
                if obstacles.contains(&pos) {
                    print!("#");
                } else if steps.is_some() && steps.as_ref().unwrap().contains(&pos) {
                    print!("0");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

const DIRECTIONS: [IVec2; 4] = [IVec2::Y, IVec2::NEG_Y, IVec2::X, IVec2::NEG_X];

// fn get_all_neighbors(pos: IVec2, grid: &Vec<Vec<char>>) -> Vec<IVec2> {
//     let mut neighbors = Vec::new();
//     let rows = grid.len() as i32;
//     let cols = grid[0].len() as i32;
    
//     // Check all four directions
//     for delta in DIRECTIONS.iter() {
//         let new_pos = IVec2::new(pos.x + delta.0, pos.y + delta.1);
//         if new_pos.x >= 0 && new_pos.x < cols && 
//            new_pos.y >= 0 && new_pos.y < rows {
//             neighbors.push(new_pos);
//         }
//     }
//     neighbors
// }

fn get_all_neighbors(pos: IVec2, grid: &PhantomGrid) -> Vec<IVec2> {
    let mut neighbors = Vec::new();
    // let (_, bounds) = grid;
    
    for delta in DIRECTIONS.iter() {
        let new_pos = pos + *delta;
        if new_pos.x >= 0 && new_pos.x <= IVec2::new(6, 6).x && 
           new_pos.y >= 0 && new_pos.y <= IVec2::new(6, 6).y &&
           !grid.0.contains(&new_pos) {  // Check if not an obstacle
            neighbors.push(new_pos);
        }
    }
    neighbors
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    #[cfg(debug_assertions)]
    let space = IVec2::new(6, 6);
    let postiion = IVec2::new(0, 0);
    let steps = 12;

    // #[cfg(not(debug_assertions))]
    // let space = IVec2::new(70, 70);
    // let steps = 1024;

    let obstacles = input.lines().take(steps).map(|line| {
        let parts = line.split(',').collect::<Vec<_>>();
        let x = parts[0].parse::<i32>().unwrap();
        let y = parts[1].parse::<i32>().unwrap();
        // tracing::debug!(x, y);

        IVec2::new(x, y)
    })
    .fold(HashSet::new(), |mut acc, pos| {
        acc.insert(pos);
        acc
    });
    // .collect::<Vec<_>>();


    let mut visited = HashSet::new();
    // let mut queue = vec![start];
    let mut queue = VecDeque::new();

    queue.push_back((IVec2::ZERO, 0)); // (position, steps)
    visited.insert(postiion);

    // dbg!(&obstacles);

    let grid = PhantomGrid(obstacles, (IVec2::ZERO, space));


    let mut final_count = 0;

    while let Some(pos) = queue.pop_front() {
        if pos.0 == space {
            final_count = steps;
            break;
            // return Some(steps);
        }

        // for next in get_all_neighbors(pos, grid) {
        //     if !visited.contains(&next) {
        //         visited.insert(next);
        //         queue.push_back((next, steps + 1));
        //     }
        // }

        for next in get_all_neighbors(pos.0, &grid) {
            if !visited.contains(&next) {
                visited.insert(next);
                queue.push_back((next, steps + 1));
            }
        }
    }

    grid.print(Some(visited));

    Ok(final_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // use rstest::rstest;

    // #[rstest]
    // #[case("", "")]
    // fn test_cases(#[case] input: &str, #[case] expected: &str) {
    //     assert_eq!(process(input).unwrap(), expected);
    // }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!("22", process(input)?);
        Ok(())
    }
}
