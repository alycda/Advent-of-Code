use std::collections::{HashMap, HashSet, VecDeque};

use crate::AocError;

use glam::*;


pub type Position = glam::IVec2;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// COL, ROW
#[derive(Debug, PartialEq, Eq)]
struct Grid(usize, usize, Vec<(IVec2, Direction)>);

impl Grid {
    fn new(cols: usize, rows: usize) -> Self {
        Self(cols, rows, Vec::new())
    }

    fn get_cols(&self) -> usize {
        self.0 + 1
    }

    fn get_rows(&self) -> usize {
        self.1
    }

    fn to_position(&self, idx: usize) -> IVec2 {
        let cols = self.get_cols();
        let col = idx % cols;
        let row = idx / cols;
        IVec2::new(col as i32, row as i32)
    }
}

#[derive(Debug, Clone)]
/// only stores the interesting positions and minmax bounds
pub struct PhantomGrid(pub HashSet<IVec2>, pub (IVec2, IVec2));

impl PhantomGrid {

    pub fn get_bounds(&self) -> (Position, Position) {
        self.1
    }

    pub fn in_bounds(&self, pos: Position) -> bool {
        // Note: We want to include the bounds
        pos.x >= 0 && pos.y >= 0 
            && pos.x <= self.1.1.x 
            && pos.y <= self.1.1.y
    }

    pub fn bfs(&self) -> u32 {
        let mut todo: VecDeque<(Position, u32)> = VecDeque::new();

        bfs(self.clone(), Position::ZERO, self.get_bounds().1, &mut todo)
    }

}


/// Up, Right, Down, Left
pub const DIRECTIONS: [Position; 4] = [Position::NEG_Y, Position::X, Position::Y, Position::NEG_X];

/// a Region or set of Positions
pub type UniquePositions = HashSet<Position>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    position: Position,
    cost: u32,
    walls_broken: u8,
}

pub fn bfs_with_breaks(
    grid: PhantomGrid, 
    start: Position, 
    end: Position,
    max_breaks: u8,
) -> Vec<u32> {
    let mut path_costs = Vec::new();
    let mut todo = VecDeque::new();
    let mut seen = HashSet::new();
    
    let initial = State {
        position: start,
        cost: 0,
        walls_broken: 0,
    };
    
    todo.push_back(initial);
    seen.insert((start, 0));

    while let Some(current) = todo.pop_front() {
        if current.position == end {
            path_costs.push(current.cost);
        }

        for dir in DIRECTIONS {
            let next_pos = current.position + dir;
            if !grid.in_bounds(next_pos) {
                continue;
            }

            let next_is_wall = grid.0.contains(&next_pos);
            if next_is_wall && current.walls_broken >= max_breaks {
                continue;
            }

            let next_breaks = if next_is_wall {
                current.walls_broken + 1
            } else {
                current.walls_broken
            };

            let next_state = (next_pos, next_breaks);
            if !seen.contains(&next_state) {
                seen.insert(next_state);
                todo.push_back(State {
                    position: next_pos,
                    cost: current.cost + 1,
                    walls_broken: next_breaks,
                });
            }
        }
    }

    dbg!(&path_costs);  
    path_costs
}

/// Breadth-first search
/// 
/// or https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html
pub fn bfs(grid: PhantomGrid, start: Position, end: Position, path_cost: &mut VecDeque<(Position, u32)>) -> u32 {
    path_cost.clear();
    let mut seen: UniquePositions = HashSet::new();

    path_cost.push_back((start, 0));
    seen.insert(start);

    while let Some((position, cost)) = path_cost.pop_front() {
        if position == end {
            return cost;
        }

        for neighbor in DIRECTIONS {
            let next = position + neighbor;
            
            // First check bounds
            if !grid.in_bounds(next) {
                continue;
            }

            // Then check if it's a wall or seen
            if !seen.contains(&next) && !grid.0.contains(&next) {
                path_cost.push_back((next, cost + 1));
                seen.insert(next);
            }
        }
    }

    u32::MAX
}

// #[tracing::instrument]
pub fn process(input: &str, target_ps: usize) -> miette::Result<String, AocError> {
    let mut peekable = input.lines().peekable();
    let cols = peekable.peek().unwrap().chars().count();
    let rows = peekable.count();

    let grid = Grid::new(cols, rows);

    let walls = input.lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| {
                    if c == '#' {
                        Some(IVec2::new(col as i32, row as i32))
                    } else {
                        None
                    }
                })
        }).collect::<HashSet<_>>();

    let start = grid.to_position(input.find("S").unwrap());
    let end = grid.to_position(input.find("E").unwrap());

    // Track minimum distances to each position
    let mut track = HashMap::new();
    let mut current = start;
    let mut current_step = 0;
    track.insert(current, current_step);

    // Build the minimal path
    while current != end {
        current_step += 1;
        
        for dir in DIRECTIONS.iter() {
            let next = current + *dir;
            if !track.contains_key(&next) && 
               next.x >= 0 && next.x < cols as i32 && 
               next.y >= 0 && next.y < rows as i32 && 
               !walls.contains(&next) {
                current = next;
                track.insert(current, current_step);
                break;
            }
        }
    }

    let mut count = 0;
    
    // Check each position we've found
    for (&pos, &steps) in &track {
        for dir in DIRECTIONS.iter() {
            let wall_pos = pos + *dir;
            let two_away = pos + *dir * 2;
            
            // Must not have been in our path
            if !track.contains_key(&wall_pos) && 
               // Must lead to a position we did reach
               track.contains_key(&two_away) {
                // Check if this shortcut saves enough time
                if track[&two_away] - steps >= target_ps as u32 + 2 {
                    count += 1;
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
