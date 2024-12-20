use std::collections::{HashSet, VecDeque};

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
                .collect::<Vec<IVec2>>()
        }).fold(HashSet::new(), |mut set, position| {
            set.insert(position);
            set
        });

    let start = grid.to_position(input.find("S").unwrap());
    let end = grid.to_position(input.find("E").unwrap());
 
    // Create distance matrices initialized to u32::MAX
    let mut start_distance = vec![vec![u32::MAX; cols]; rows];
    let mut end_distance = vec![vec![u32::MAX; cols]; rows];
 
    // Calculate distances from start
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    start_distance[start.y as usize][start.x as usize] = 0;
    
    while let Some((pos, cost)) = queue.pop_front() {
        for dir in DIRECTIONS {
            let next = pos + dir;
            if next.x >= 0 && next.x < cols as i32 && 
               next.y >= 0 && next.y < rows as i32 {
                let next_cost = cost + 1;
                if !walls.contains(&next) && 
                   start_distance[next.y as usize][next.x as usize] == u32::MAX {
                    start_distance[next.y as usize][next.x as usize] = next_cost;
                    queue.push_back((next, next_cost));
                }
            }
        }
    }
 
    // Calculate distances from end (same process)
    queue.clear();
    queue.push_back((end, 0));
    end_distance[end.y as usize][end.x as usize] = 0;
 
    while let Some((pos, cost)) = queue.pop_front() {
        for dir in DIRECTIONS {
            let next = pos + dir;
            if next.x >= 0 && next.x < cols as i32 && 
               next.y >= 0 && next.y < rows as i32 {
                let next_cost = cost + 1;
                if !walls.contains(&next) && 
                   end_distance[next.y as usize][next.x as usize] == u32::MAX {
                    end_distance[next.y as usize][next.x as usize] = next_cost;
                    queue.push_back((next, next_cost));
                }
            }
        }
    }
 
    let orig_distance = start_distance[end.y as usize][end.x as usize];
    let mut count = 0;
 
    // Check all possible midpoints
    for y in 0..rows {
        for x in 0..cols {
            let mid = IVec2::new(x as i32, y as i32);
            if walls.contains(&mid) || start_distance[y][x] == u32::MAX {
                continue;
            }
 
            // Check all points at manhattan distance 2
            for dy in -2..=2 {
                for dx in -2..=2 {
                    let offset = IVec2::new(dx, dy);
                    if offset.x.abs() + offset.y.abs() != 2 {
                        continue;
                    }
 
                    let end_point = mid + offset;
                    if end_point.x < 0 || end_point.x >= cols as i32 || 
                       end_point.y < 0 || end_point.y >= rows as i32 {
                        continue;
                    }
 
                    if walls.contains(&end_point) {
                        continue;
                    }
 
                    let end_y = end_point.y as usize;
                    let end_x = end_point.x as usize;
                    if end_distance[end_y][end_x] == u32::MAX {
                        continue;
                    }
 
                    let new_distance = start_distance[y][x] + end_distance[end_y][end_x] + 2;
                    if new_distance + target_ps as u32 == orig_distance {
                        count += 1;
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
