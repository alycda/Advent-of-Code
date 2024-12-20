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

// Add this to track state
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
    let mut todo: VecDeque<State> = VecDeque::new();
    // Need to track position AND walls broken in seen states
    let mut seen: HashSet<(Position, u8)> = HashSet::new();

    todo.push_back(State {
        position: start,
        cost: 0,
        walls_broken: 0,
    });
    seen.insert((start, 0));

    while let Some(State { position, cost, walls_broken }) = todo.pop_front() {
        if position == end {
            path_costs.push(cost);
            continue; // Don't return - we want all possible paths
        }

        for neighbor in DIRECTIONS {
            let next = position + neighbor;
            
            if !grid.in_bounds(next) {
                continue;
            }

            let is_wall = grid.0.contains(&next);
            // Can we move here?
            if is_wall && walls_broken >= max_breaks {
                continue;
            }

            let new_walls_broken = if is_wall { 
                walls_broken + 1 
            } else { 
                walls_broken 
            };

            if !seen.contains(&(next, new_walls_broken)) {
                todo.push_back(State {
                    position: next,
                    cost: cost + 1,
                    walls_broken: new_walls_broken,
                });
                seen.insert((next, new_walls_broken));
            }
        }
    }

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
pub fn process(input: &str, picoseconds: usize) -> miette::Result<String, AocError> {

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

    dbg!(&walls, start, end, picoseconds);
    // dbg!(walls.bfs());
    // let path = dbg!(bfs(PhantomGrid(walls, (start, end)), start, end, &mut VecDeque::new()));

    // Ok(path.to_string())

    let base_path = bfs(PhantomGrid(walls.clone(), (Position::ZERO, Position::new(cols as i32, rows as i32))), start, end, &mut VecDeque::new());
    let all_paths = dbg!(bfs_with_breaks(PhantomGrid(walls, (Position::ZERO, Position::new(cols as i32, rows as i32))), start, end, 2));
    
    // Debug the paths and savings
    let paths_with_savings: Vec<_> = all_paths.iter()
        .map(|&path_length| {
            let saved = base_path - path_length;
            (path_length, saved)
        })
        .collect();
    dbg!(&paths_with_savings);

    let count = paths_with_savings.iter()
        .filter(|(_, saved)| *saved as usize == picoseconds)
        .count();

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("2", "14")]
    // #[case("4", "14")]
    // #[case("6", "2")]
    // #[case("8", "4")]
    // #[case("10", "2")]
    // #[case("12", "3")]
    // #[case("20", "1")]
    // #[case("36", "1")]
    // #[case("38", "1")]
    // #[case("40", "1")]
    // #[case("64", "1")]
    fn test_cases(#[case] input: &str, #[case] expected: &str) {
        let map = "###############
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

        assert_eq!(process(map, input.parse::<usize>().unwrap()).unwrap(), expected);
    }

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
