use std::collections::{HashSet, VecDeque};

use glam::IVec2;

use crate::AocError;

#[derive(Debug)]
/// only stores the interesting positions and minmax bounds
pub struct PhantomGrid(pub HashSet<IVec2>, pub (IVec2, IVec2));

impl PhantomGrid {
    pub fn print(&self, steps: Option<&HashSet<IVec2>>) {
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

fn get_all_neighbors(pos: IVec2, grid: &PhantomGrid) -> Vec<IVec2> {
    DIRECTIONS.iter()
        .map(|&delta| pos + delta)
        .filter(|&new_pos| {
            new_pos.x >= 0 && 
            new_pos.x <= grid.1.1.x && 
            new_pos.y >= 0 && 
            new_pos.y <= grid.1.1.y && 
            !grid.0.contains(&new_pos)
        })
        .collect()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    #[cfg(debug_assertions)]
    let space = IVec2::new(6, 6);
    let position = IVec2::new(0, 0);

    // Parse obstacles into HashSet
    let obstacles = input.lines().map(|line| {
        let parts = line.split(',').collect::<Vec<_>>();
        IVec2::new(
            parts[0].parse::<i32>().unwrap(),
            parts[1].parse::<i32>().unwrap()
        )
    }).collect::<HashSet<_>>();

    let grid = PhantomGrid(obstacles, (IVec2::ZERO, space));
    
    // Track visited positions and their steps
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut shortest_path = None;

    queue.push_back((position, 0)); // (position, steps taken)
    visited.insert(position);

    while let Some((pos, steps)) = queue.pop_front() {
        if pos == space {
            shortest_path = Some(steps);
            break;
        }

        for next in get_all_neighbors(pos, &grid) {
            if !visited.contains(&next) {
                visited.insert(next);
                queue.push_back((next, steps + 1));
            }
        }
    }

    if let Some(steps) = shortest_path {
        // Find all positions that take exactly 'steps' moves to reach
        visited.clear();
        queue.clear();
        queue.push_back((position, 0));
        visited.insert(position);
        let mut positions_at_target = HashSet::new();

        while let Some((pos, current_steps)) = queue.pop_front() {
            if current_steps == steps {
                positions_at_target.insert(pos);
                continue;
            }
            
            if current_steps > steps {
                break;
            }

            for next in get_all_neighbors(pos, &grid) {
                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back((next, current_steps + 1));
                }
            }
        }

        grid.print(Some(&positions_at_target));
        Ok(positions_at_target.len().to_string())
    } else {
        Ok("0".to_string())
    }
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
