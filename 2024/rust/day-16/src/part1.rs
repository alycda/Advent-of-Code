use std::{cmp::Ordering, collections::{BinaryHeap, HashSet}};

use glam::IVec2;

use crate::AocError;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Day16 {
    position: IVec2,
    direction: Direction,
    cost: usize,
}

impl Ord for Day16 {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Day16 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_position(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::NEG_Y,
            Direction::Down => IVec2::Y,
            Direction::Right => IVec2::X,
            Direction::Left => IVec2::NEG_X,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {

    let mut peekable = input.lines().peekable();
    let cols = peekable.peek().unwrap().chars().count();
    let rows = peekable.count();

    let grid = Grid::new(cols, rows);

    let walls = input.match_indices("#")
        .map(|(idx, _)| grid.to_position(idx))
        .collect::<HashSet<_>>();
    let mut start = grid.to_position(input.find("S").unwrap());
    let mut end = grid.to_position(input.find("E").unwrap());
    let mut direction = Direction::Right;

    dbg!(start, end);
    // dbg!(start, end, walls);

    let initial_state = Day16 {
        position: start,
        direction: Direction::Right,
        cost: 0,
    };

    // Create priority queue for states to explore
    let mut queue = BinaryHeap::new();
    
    // Track visited states (position + direction)
    let mut visited = HashSet::new();

    queue.push(initial_state);

    while let Some(current) = queue.pop() {
        // If we reached the end, return the cost
        if current.position == end {
            return Ok(current.cost.to_string());
        }

        // Skip if we've seen this state
        let state_key = (current.position, current.direction);
        if !visited.insert(state_key) {
            continue;
        }

        // Generate next possible moves:
        
        // 1. Try moving forward
        let next_pos = current.position + current.direction.to_position();
        if !walls.contains(&next_pos) {
            queue.push(Day16 {
                cost: current.cost + 1,
                position: next_pos,
                direction: current.direction,
            });
        }

        // 2. Try turning right (cost 1000)
        let right_dir = current.direction.turn_right();
        queue.push(Day16 {
            cost: current.cost + 1000,
            position: current.position,
            direction: right_dir,
        });

        // 3. Try turning left (cost 1000)
        let left_dir = current.direction.turn_left();
        queue.push(Day16 {
            cost: current.cost + 1000,
            position: current.position,
            direction: left_dir,
        });
    }

    // dbg!(visited.len());
    // dbg!(queue);

    Ok(visited.len().to_string())
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
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!("7036", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_2() -> miette::Result<()> {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!("11048", process(input)?);
        Ok(())
    }
}
