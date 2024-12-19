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

fn bfs(grid: &PhantomGrid, todo: &mut VecDeque<(IVec2, u32)>, id: usize) -> u32 {
    todo.clear();
    let mut seen = HashSet::new();
    
    todo.push_back((IVec2::ZERO, 0));
    seen.insert(IVec2::ZERO);

    #[cfg(debug_assertions)]
    println!("Starting BFS with obstacles: {:?}", grid.0);

    while let Some((position, cost)) = todo.pop_front() {
        #[cfg(debug_assertions)]
        println!("At position {:?} with cost {}", position, cost);
        
        #[cfg(debug_assertions)]
        let a = 6;

        #[cfg(not(debug_assertions))]
        let a = 70;

        if position == IVec2::new(a, a) {
            #[cfg(debug_assertions)]
            println!("Found path with cost {}", cost);
            return cost;
        }

        for offset in ORTHOGONAL {
            let next = position + offset;
            if next.x >= 0 && next.x <= a && 
               next.y >= 0 && next.y <= a && 
               !grid.0.contains(&next) && 
               !seen.contains(&next) {
                #[cfg(debug_assertions)]
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
    #[cfg(not(debug_assertions))]
    let space = IVec2::new(70, 70);

    let mut obstacles = HashSet::new();
    let mut todo = VecDeque::new();
    
    // Process obstacles one by one until we find the blocker
    for line in input.lines() {
        let parts = line.split(',').collect::<Vec<_>>();
        let pos = IVec2::new(
            parts[0].parse::<i32>().unwrap(),
            parts[1].parse::<i32>().unwrap()
        );
        
        // Add this obstacle
        obstacles.insert(pos);
        let grid = PhantomGrid(obstacles.clone(), (IVec2::ZERO, space));
        
        // Try to find path with current obstacles
        let path_length = bfs(&grid, &mut todo, 0);
        
        // If no path exists, we found our blocker
        if path_length == u32::MAX {
            // Return the coordinates of the blocking position
            return Ok(format!("{},{}", pos.x, pos.y));
        }
    }

    Ok("No blocking obstacle found".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
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
        assert_eq!("6,1", process(input)?);
        Ok(())
    }
}
