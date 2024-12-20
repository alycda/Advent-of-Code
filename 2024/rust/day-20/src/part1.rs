use std::collections::{HashMap, HashSet, VecDeque};

pub type Position = glam::IVec2;

/// a Region or set of Positions
pub type UniquePositions = HashSet<Position>;

/// Up, Right, Down, Left
pub const DIRECTIONS: [Position; 4] = [Position::NEG_Y, Position::X, Position::Y, Position::NEG_X];

/// stores all chars, not recommended for NUMBERS (u8 vs char)
#[derive(Debug)]
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> std::ops::Deref for Grid<T> {
    type Target = Vec<Vec<T>>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: std::fmt::Debug + Copy + PartialEq> Grid<T> {
    pub fn get_width(&self) -> usize {
        self[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.len()
    }

    pub fn to_position(&self, idx: usize) -> Position {
        let cols = self.get_width();
        let chars_per_row = cols + 1;
        let col = idx % chars_per_row;
        let row = idx / chars_per_row;
        Position::new(col as i32, row as i32)
    }

    /// Walks the grid from top-left to bottom-right
    pub fn walk<F: FnMut(Position) -> O, O>(&self, mut see: F) {
        for row in 0..self.get_height() {
            for col in 0..self.get_width() {
                let pos = Position::new(col as i32, row as i32);

                see(pos);
            }
        }
    }

    /// Bounded by the grid's dimensions
    pub fn get_at(&self, pos: Position) -> Option<T> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.get_width() as i32 || pos.y >= self.get_height() as i32 {
            return None;
        }

        Some(self[pos.y as usize][pos.x as usize])
        // Some(self.get_at_unbounded(pos))
    }

    pub fn get_at_unbounded(&self, pos: Position) -> T {
        self[pos.y as usize][pos.x as usize]
    }

    pub fn to_maze(&self, to_match: T) -> PhantomGrid {
        let mut walls = UniquePositions::new();

        self.walk(|pos| {
            if self.get_at_unbounded(pos) == to_match {
                walls.insert(pos);
            }
        });

        PhantomGrid(walls, (Position::ZERO, Position::new(self.get_width() as i32 - 1, self.get_height() as i32 - 1)))
    }
}

#[derive(Debug, Clone)]
/// only stores the interesting positions and minmax bounds
pub struct PhantomGrid(pub UniquePositions, pub (Position, Position));

impl std::ops::Deref for PhantomGrid {
    type Target = UniquePositions;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for PhantomGrid {
    fn deref_mut(&mut self) -> &mut UniquePositions {
        &mut self.0
    }
}

/// TODO: Day 10, height/rating & Day 20
pub struct Something<T>(HashMap<Position, T>);

impl<T> std::ops::Deref for Something<T> {
    type Target = HashMap<Position, T>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Something<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

fn bfs() {

}


// #[tracing::instrument]
pub fn process(input: &str, target_ps: i32) -> miette::Result<String, crate::AocError> {
    let grid = Grid(input.lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>());

    let rows = grid.get_height();
    let cols = grid.get_width();

    let start = grid.to_position(input.find("S").unwrap());
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
