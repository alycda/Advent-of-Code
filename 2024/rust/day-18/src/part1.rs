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

use std::collections::{HashSet, VecDeque};
use glam::IVec2;

const ORTHOGONAL: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

// fn bfs(grid: &PhantomGrid, todo: &mut VecDeque<(IVec2, u32)>, id: usize) -> u32 {
//     todo.clear();
//     let mut seen = HashSet::new();
    
//     todo.push_back((IVec2::ZERO, 0));
//     seen.insert(IVec2::ZERO);

//     while let Some((position, cost)) = todo.pop_front() {
//         #[cfg(debug_assertions)]
//         if position == IVec2::new(6, 6) {
//             return cost;
//         }

//         #[cfg(not(debug_assertions))]
//         if position == IVec2::new(70, 70) {
//             return cost;
//         }

//         for offset in ORTHOGONAL {
//             let next = position + offset;
//             if next.x >= 0 && next.x <= 6 && 
//                next.y >= 0 && next.y <= 6 && 
//                !grid.0.contains(&next) && 
//                !seen.contains(&next) {
//                 todo.push_back((next, cost + 1));
//                 seen.insert(next);
//             }
//         }
//     }

//     u32::MAX
// }

fn bfs(grid: &PhantomGrid, todo: &mut VecDeque<(IVec2, u32)>, id: usize) -> u32 {
    todo.clear();
    let mut seen = HashSet::new();
    
    todo.push_back((IVec2::ZERO, 0));
    seen.insert(IVec2::ZERO);

    println!("Starting BFS with obstacles: {:?}", grid.0);

    while let Some((position, cost)) = todo.pop_front() {
        println!("At position {:?} with cost {}", position, cost);
        
        #[cfg(debug_assertions)]
        let a = 6;

        #[cfg(not(debug_assertions))]
        let a = 70;

        if position == IVec2::new(a, a) {
            println!("Found path with cost {}", cost);
            return cost;
        }

        for offset in ORTHOGONAL {
            let next = position + offset;
            if next.x >= 0 && next.x <= a && 
               next.y >= 0 && next.y <= a && 
               !grid.0.contains(&next) && 
               !seen.contains(&next) {
                println!("Adding next position: {:?}", next);
                todo.push_back((next, cost + 1));
                seen.insert(next);
            }
        }
    }

    println!("No path found!");
    u32::MAX
}

pub fn process(input: &str) -> miette::Result<String, crate::AocError> {
    #[cfg(debug_assertions)]
    let space = IVec2::new(6, 6);
    #[cfg(debug_assertions)]
    let take_how_many = 12;

    #[cfg(not(debug_assertions))]
    let space = IVec2::new(70, 70);
    
    #[cfg(not(debug_assertions))]
    let take_how_many = 1024;

    let obstacles = input.lines()
        .take(take_how_many)
        .map(|line| {
            let parts = line.split(',').collect::<Vec<_>>();
            IVec2::new(
                parts[0].parse::<i32>().unwrap(),
                parts[1].parse::<i32>().unwrap()
            )
        })
        .collect::<HashSet<_>>();

    let grid = PhantomGrid(obstacles, (IVec2::ZERO, space));
    let mut todo = VecDeque::new();
    
    let shortest_path = bfs(&grid, &mut todo, 0);
    
    // Now find all positions at shortest_path distance
    let mut all_positions = HashSet::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((IVec2::ZERO, 0));
    visited.insert(IVec2::ZERO);

    while let Some((pos, steps)) = queue.pop_front() {
        if steps == shortest_path {
            all_positions.insert(pos);
            continue;
        }
        
        if steps > shortest_path {
            break;
        }

        for offset in ORTHOGONAL {
            let next = pos + offset;
            if next.x >= 0 && next.x <= 6 && 
               next.y >= 0 && next.y <= 6 && 
               !grid.0.contains(&next) && 
               !visited.contains(&next) {
                queue.push_back((next, steps + 1));
                visited.insert(next);
            }
        }
    }

    #[cfg(debug_assertions)]
    grid.print(Some(&all_positions));

    // dbg!(all_positions.len());
    // panic!("halt");

    Ok(shortest_path.to_string())
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
