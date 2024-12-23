use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash};
use tracing::{debug, instrument};

pub type Position = glam::IVec2;
pub type Velocity = glam::IVec2;

pub fn manhattan_distance(a: &Position, b: &Position) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

// pub type Dijkstra<T> = BinaryHeap<T>;

// fn dijkstra_search<T>(mut queue: Dijkstra<T>, mut distances: impl std::hash::BuildHasher) {
//     while let Some(T) = queue.pop() {
//         if position == end { return cost; }
        
//         for neighbor in get_neighbors(position) {
//             let next = State {
//                 cost: cost + edge_weight(position, neighbor),
//                 position: neighbor,
//             };
//             queue.push(next);
//         }
//     }
// }

// #[derive(Eq, PartialEq, Ord, PartialOrd)]
// struct State<P> {
//     cost: usize,
//     position: P,
// }

// pub type Dijkstra<P> = BinaryHeap<Reverse<State<P>>>; // Note: Reverse for min-heap

// fn dijkstra_search<P, H>(
//     mut queue: Dijkstra<P>,
//     mut distances: H,
//     end: P,
//     get_neighbors: impl Fn(&P) -> Vec<P>,
//     edge_weight: impl Fn(&P, &P) -> usize,
// ) -> Option<usize> 
// where 
//     P: Eq + Hash + Clone,
//     H: BuildHasher
// {
//     while let Some(Reverse(State { cost, position })) = queue.pop() {
//         if position == end { 
//             return Some(cost); 
//         }
        
//         for neighbor in get_neighbors(&position) {
//             let next = State {
//                 cost: cost + edge_weight(&position, &neighbor),
//                 position: neighbor,
//             };
//             queue.push(Reverse(next));
//         }
//     }
//     None
// }


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Direction {
    /// A, North
    Up,
    /// X, South
    Down,
    /// #, West
    Left,
    /// O, East
    Right
}

impl Direction {
    pub fn to_offset(&self) -> Position {
        match self {
            Direction::Up => Position::NEG_Y,
            Direction::Down => Position::Y,
            Direction::Left => Position::NEG_X,
            Direction::Right => Position::X
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down
        }
    }
}

pub struct MyString(String, HashSet<String>);

impl MyString {
    /// Breadth-first search
    pub fn bfs(&self, target: &str) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(self.0.to_owned());

        while let Some(current) = queue.pop_front() {
            for pattern in self.1.iter() {
                // Using string buffer (more complex but potentially more efficient)
                // let mut next = String::with_capacity(current.len() + pattern.len());
                // next.push_str(&current);
                // next.push_str(pattern);
                let next = current.to_owned() + pattern;
                
                if next == target {
                    return true;
                }
                if target.starts_with(&next) {
                    queue.push_back(next);
                }
            }
        }

        false
    }
}

/// Up, Right, Down, Left
pub const DIRECTIONS: [Position; 4] = [Position::NEG_Y, Position::X, Position::Y, Position::NEG_X];

/// NE??, SE??, ???, ???
// pub const DIAGONALS: [Position; 4] = [Position::ONE, Position::new(1, -1), Position::new(-1, -1), Position::new(-1, 1)];
/// ???, ???, ???, ???
pub const DIAGONALS: [Position; 4] = [Position::NEG_ONE, Position::new(-1, 1), Position::new(1, -1), Position::new(1, 1)];

/// Up, NE, Right, SE, Down, SW, Left, NW
pub const ALL_DIRECTIONS: [Position; 8] = [Position::NEG_Y, Position::ONE, Position::X, Position::new(1, -1), Position::Y, Position::NEG_ONE, Position::NEG_X, Position::new(-1, 1)];

/// Breadth-first search
/// 
/// or https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html
pub fn bfs(grid: PhantomGrid, start: Position, end: Position, path_cost: &mut VecDeque<(Position, u32)>) -> u32 {
    path_cost.clear();

    let mut seen: UniquePositions = HashSet::new();

    path_cost.push_back((start, 0));
    seen.insert(start);

    // println!("Starting BFS with obstacles: {:?}", grid.0);

    while let Some((position, cost)) = path_cost.pop_front() {
        // println!("At position {:?} with cost {}", position, cost);

        if position == end {
            // println!("Found path with cost {}", cost);
            return cost;
        }

        for neighbor in DIRECTIONS {
            let next = position + neighbor;
            if next.x >= 0 && next.x <= end.x && 
               next.y >= 0 && next.y <= end.y && 
               !grid.0.contains(&next) && 
               !seen.contains(&next) {
                // println!("Adding next position: {:?}", next);

                path_cost.push_back((next, cost + 1));
                seen.insert(next);
            }
        }
    }

    // println!("No path found!");
    u32::MAX
}

/// stores all chars, not recommended for NUMBERS (u8 vs char)
#[derive(Debug)]
pub struct Grid<T>(Vec<Vec<T>>);
// pub struct Grid<T>(Vec<Vec<T>>);


/// a Region or set of Positions
pub type UniquePositions = HashSet<Position>;

/// Visited positions and the direction in which they were travelled
pub type Backtracks = HashSet<(Position, Direction)>;

/// TODO: Day 10, height/rating
pub struct Something<T>(HashMap<Position, T>);
impl<T> std::ops::Deref for Something<T> {
    type Target = HashMap<Position, T>;
    
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

    /// Walks the grid from top-left to bottom-right
    pub fn walk<F: FnMut(Position) -> O, O>(&self, mut see: F) {
        for row in 0..self.get_height() {
            for col in 0..self.get_width() {
                let pos = Position::new(col as i32, row as i32);

                see(pos);
            }
        }
    }

    // fn go_straight_up(&self, start: IVec2, steps: usize) {
    //     self.go_straight(start, IVec2::NEG_Y, steps, None);
    //     todo!()
    // }

    // move in a straight line from the start direction the given number of steps
    // pub fn go_straight<F: Fn() -> bool>(&self, start: IVec2, direction: IVec2, steps: usize, _test: Option<F>) -> Option<Vec<char>> {
    #[instrument]
    pub fn go_straight(&self, start: Position, towards: Position, steps: usize) -> Option<Vec<T>> {
        let end_pos = start + (towards * steps as i32);
        if !self.in_bounds(end_pos) {
            debug!("{steps} steps from {start} to {towards} is out of bounds");
            return None;
        }

        (1..=steps)
            .map(|i| start + (towards * i as i32))
            .map(|pos| self.get_at(pos))
            .collect::<Option<Vec<_>>>()
    }

    pub fn get_at_unbounded(&self, pos: Position) -> T {
        self[pos.y as usize][pos.x as usize]
    }

    /// Bounded by the grid's dimensions
    pub fn get_at(&self, pos: Position) -> Option<T> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.get_width() as i32 || pos.y >= self.get_height() as i32 {
            return None;
        }

        Some(self[pos.y as usize][pos.x as usize])
        // Some(self.get_at_unbounded(pos))
    }

    pub fn to_position(&self, idx: usize) -> Position {
        let cols = self.get_width();
        let chars_per_row = cols + 1;
        let col = idx % chars_per_row;
        let row = idx / chars_per_row;
        Position::new(col as i32, row as i32)
    }

    /// Bounded by the grid's dimensions
    pub fn get_neighbor(&self, from: Position, at: Direction) -> Option<T> {
        let neighbor = from + at.to_offset();

        // self[neighbor.y as usize][neighbor.x as usize]
        self.get_at(neighbor)
    }

    pub fn in_bounds(&self, pos: Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.get_width() as i32 && pos.y < self.get_height() as i32
    }

    pub fn get_orthogonal_neighbors(&self, pos: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        
        // Same deltas as the working version
        for delta in DIRECTIONS.iter() {
            let new_pos = pos + *delta;
            
            // Boundary check matching the working version
            if new_pos.x >= 0 && new_pos.x < self.get_width() as i32 && 
               new_pos.y >= 0 && new_pos.y < self.get_height() as i32 {
                neighbors.push(new_pos);
            }
        }
        
        neighbors
    }

    // pub fn get_orthogonal_neighbors(&self, from: Position) -> HashMap<Direction, (Position, T)> {
    //     let mut neighbors: HashMap<Direction, (Position, T)> = HashMap::new();

    //     // for direction in DIRECTIONS.iter() {
    //     //     let neighbor = from + *direction;
    //     //     let value = self.get_at(neighbor);

    //     //     if let Some(value) = value {
    //     //         neighbors.insert(
    //     //             match direction {
    //     //                 IVec2 { x: 0, y: 1 } => Direction::Down,
    //     //                 IVec2 { x: 0, y: -1 } => Direction::Up,
    //     //                 IVec2 { x: 1, y: 0 } => Direction::Right,
    //     //                 IVec2 { x: -1, y: 0 } => Direction::Left,
    //     //                 _ => unreachable!()
    //     //             },
    //     //             (neighbor, value)
    //     //         );
    //     //     }
    //     // }

    //     // Look up (decrease Y)
    //     if from.y > 0 {
    //         let neighbor = from + Direction::Up.to_offset();
    //         // neighbors.insert(Direction::Up, (neighbor, self.get_at(neighbor).unwrap()));
    //         neighbors.insert(Direction::Up, (neighbor, self.get_at_unbounded(neighbor)));
    //     }

    //     // Look down (increase Y)
    //     if from.y + 1 < self.get_height() as i32 {
    //         let neighbor = from + Direction::Down.to_offset();
    //         // neighbors.insert(Direction::Down, (neighbor, self.get_at(neighbor).unwrap()));
    //         neighbors.insert(Direction::Down, (neighbor, self.get_at_unbounded(neighbor)));
    //     }

    //     // Look left (decrease X)
    //     if from.x > 0 {
    //         let neighbor = from + Direction::Left.to_offset();
    //         // neighbors.insert(Direction::Left, (neighbor, self.get_at(neighbor).unwrap()));
    //         neighbors.insert(Direction::Left, (neighbor, self.get_at_unbounded(neighbor)));
    //     }

    //     // Look right (increase X)
    //     if from.x + 1 < self.get_width() as i32 {
    //         let neighbor = from + Direction::Right.to_offset();
    //         // neighbors.insert(Direction::Right, (neighbor, self.get_at(neighbor).unwrap()));
    //         neighbors.insert(Direction::Right, (neighbor, self.get_at_unbounded(neighbor)));
    //     }

    //     neighbors
    // }

    fn _get_diagonal_neghbors(&self, _from: Position) -> Vec<(Position, T)> {
        todo!()
    }

    fn _get_all_neighbors(&self, _from: Position) -> Vec<(Position, T)> {
        todo!()
    }

    /// Depth-first search
    pub fn flood_fill(&self, start: Position, visited: &mut UniquePositions) -> UniquePositions {
        let mut region = HashSet::new();
        let mut stack = vec![start];
        let target = self.get_at_unbounded(start);
        
        while let Some(pos) = stack.pop() {
            if !region.insert(pos) {
                continue;
            }
            visited.insert(pos);
            
            for neighbor in self.get_orthogonal_neighbors(pos) {
                // let (neighbor_pos, neighbor_char) = neighbor;
                // if *neighbor_char == target && !region.contains(neighbor_pos) {
                //     stack.push(*neighbor_pos);
                // }
                let neighbor_char = self.get_at_unbounded(pos);
                if neighbor_char == target && !region.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
        
        region
    }

    // pub fn flood_fill(&self, start: Position, visited: &mut HashSet<Position>) -> HashSet<Position> {
    //     let mut region = HashSet::new();
    //     let mut stack = vec![start];
    //     let target = self.get_at_unbounded(start);
        
    //     while let Some(pos) = stack.pop() {
    //         if !region.insert(pos) {
    //             continue;
    //         }
    //         visited.insert(pos);
            
    //         for neighbor in self.get_orthogonal_neighbors(pos) {
    //             // let neighbor_target = self.get_at_unbounded(neighbor);
    //             // let neighbor_target = neighbor.1;

    //             // if target == neighbor.1 && !region.contains(&neighbor.0) {
    //             //     stack.push(neighbor.0);
    //             // }
    //         }
    //     }
        
    //     region
    // }

    pub fn count_region_edges(&self, region: &UniquePositions) -> usize {
        let mut edges = 0;
    
        for pos in region {
            for neighbor in self.get_orthogonal_neighbors(*pos) {
                // Direct comparison now possible
                if !region.contains(&neighbor) {
                    edges += 1;
                }
            }
            
            // Border checks remain the same
            if pos.x == 0 || pos.x == (self.get_width() - 1) as i32 { edges += 1; }
            if pos.y == 0 || pos.y == (self.get_height() - 1) as i32 { edges += 1; }
        }
        
        edges
    }

    // /// assumes Set of Positions that are adjacent/touching...
    // pub fn count_region_edges(&self, region: &UniquePositions) -> usize {
    //     let mut edges = 0;

    //     for pos in region {
    //         for neighbor in self.get_orthogonal_neighbors(*pos) {                
    //             // If neighbor is outside region, it's an edge
    //             if !region.contains(&neighbor.1.0) {
    //                 edges += 1;
    //             }
    //         }
            
    //         // Count border edges
    //         if pos.x == 0 || pos.x == (self.get_width() - 1) as i32 { edges += 1; }
    //         if pos.y == 0 || pos.y == (self.get_height() - 1) as i32 { edges += 1; }
    //     }
        
    //     edges
    // }

    pub fn to_maze(&self, to_match: T) -> PhantomGrid {
        let mut walls = UniquePositions::new();

        self.walk(|pos| {
            if self.get_at(pos).unwrap() == to_match {
                walls.insert(pos);
            }
        });

        PhantomGrid(walls, (Position::ZERO, Position::new(self.get_width() as i32 - 1, self.get_height() as i32 - 1)))
    }
}

impl<T> std::ops::Deref for Grid<T> {
    type Target = Vec<Vec<T>>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
/// only stores the interesting positions and minmax bounds
pub struct PhantomGrid(pub UniquePositions, pub (Position, Position));

impl PhantomGrid {
    // Creates new grid with given dimensions
    pub fn new(width: i32, height: i32) -> Self {
        Self(
            HashSet::new(),
            (Position::ZERO, Position::new(width - 1, height - 1))
        )
    }

    pub fn get_bounds(&self) -> (Position, Position) {
        self.1
    }

    pub fn in_bounds(&self, pos: Position) -> bool {
        pos.x >= 0 && pos.y >= 0 
            && pos.x <= self.1.1.x 
            && pos.y <= self.1.1.y
    }

    pub fn bfs(&self) -> u32 {
        let mut todo: VecDeque<(Position, u32)> = VecDeque::new();

        bfs(self.clone(), Position::ZERO, self.get_bounds().1, &mut todo)
    }

    // pub fn print(&self, steps: Option<&HashSet<Position>>) {
    //     let obstacles = &self.0;
    //     let bounds = self.1;

    //     // let (obstacles, bounds) = self;
    //     let (min, max) = bounds;
    //     for y in min.y..=max.y {
    //         for x in min.x..=max.x {
    //             let pos = Position::new(x, y);
    //             if obstacles.contains(&pos) {
    //                 print!("#");
    //             } else if steps.is_some() && steps.as_ref().unwrap().contains(&pos) {
    //                 print!("0");
    //             } else {
    //                 print!(".");
    //             }
    //         }
    //         println!();
    //     }
    // }

    // // Prints the grid with obstacles and optionally a path
    // pub fn debug_print(&self, path: Option<&HashSet<Position>>) {
    //     let (min, max) = self.1;
    //     println!("\nGrid State ({:?} to {:?}):", min, max);
    //     println!("  " + &(0..=max.x).map(|x| format!("{}", x%10)).collect::<Vec<_>>().join(" "));
        
    //     for y in 0..=max.y {
    //         print!("{:1} ", y%10);
    //         for x in 0..=max.x {
    //             let pos = Position::new(x, y);
    //             let c = if self.0.contains(&pos) {
    //                 '█' // obstacle
    //             } else if Some(true) == path.map(|p| p.contains(&pos)) {
    //                 '•' // path
    //             } else if pos == Position::ZERO {
    //                 'S' // start
    //             } else if pos == max {
    //                 'E' // end
    //             } else {
    //                 '.' // empty
    //             };
    //             print!("{} ", c);
    //         }
    //         println!();
    //     }
    //     println!();
    // }

    // // Prints a step-by-step visualization of the path
    // pub fn debug_print_path(&self, path: &[(Position, usize)]) {
    //     println!("\nPath Steps:");
    //     for (step, (pos, count)) in path.iter().enumerate() {
    //         println!("Step {}: {:?} (total steps: {})", step, pos, count);
    //         let visited = path[0..=step]
    //             .iter()
    //             .map(|(p, _)| *p)
    //             .collect::<HashSet<_>>();
    //         self.debug_print(Some(&visited));
    //     }
    // }

    // Validates that a position is within bounds and not an obstacle
    pub fn debug_is_valid(&self, pos: Position) -> bool {
        let (min, max) = self.1;
        pos.x >= min.x && pos.x <= max.x &&
        pos.y >= min.y && pos.y <= max.y &&
        !self.0.contains(&pos)
    }

}

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

pub trait Parser {
    type Parsed;
    fn parse(input: &str) -> Self::Parsed;
}

pub trait Solution/* : Parser<Parsed = Self> */ {
    /// Ensures the output can be converted to a string
    type Output: std::fmt::Display;  

    // This represents the fully parsed solution data
    // type Parsed: Sized = Self;

    /// used for the nom parser function
    type Item;
    // type Item = (); associated type defaults are unstable

    /// TODO: fn parse(input: &'_ str) -> miette::Result<Self, AocError> where Self: Sized;
    /// 
    /// using the anonymous lifetime instead of 'static to support benchamrking (criterion, divan)
    fn parse(input: &'_ str) -> Self;

    /// generally intended to parse a single line, not the full input
    /// https://tfpk.github.io/nominomicon/introduction.html
    fn nom_parser(_input: &str) -> nom::IResult<&str, Self::Item, nom::error::Error<&str>> where Self: Sized {
        todo!()
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        todo!()
    }
    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        todo!()
    }
    
    fn solve(&mut self, part: Part) -> miette::Result<String, AocError> {
        Ok(match part {
            Part::One => self.part1()?.to_string(),
            Part::Two => self.part2()?.to_string()
        })
    } 

    fn get_mut(&mut self) -> &mut Self {
        self
    }

    fn to_grid(input: &str) -> Grid<char> {
        Grid(input.lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>())
    }

    // fn to_grid<T, F>(input: &str, parser: Option<F>) -> Grid<T>
    // where
    //     F: Fn(char) -> Option<T>,
    // {
    //     Grid(input.lines()
    //         .map(|line| {
    //             line.chars()
    //                 .map(|c| {
    //                     parser.as_ref()
    //                         .map_or_else(|| c as T, |parse| parse(c).unwrap_or_else(|| c as T))
    //                 })
    //                 .collect::<Vec<_>>()
    //         })
    //         .collect::<Vec<Vec<_>>>())
    // }

    fn print(_input: &str) {
        todo!()
    }

    // fn with_data<F, R>(&mut self, f: F) -> R 
    // where 
    //     F: for<'a> FnOnce(&'a mut Self) -> R 
    // {
    //     f(self)
    // }
}

#[derive(Debug, Clone, Copy)]
pub enum Part {
    One,
    Two
}

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
    
    #[error("Failed to parse input: {msg}")]
    #[diagnostic(code(aoc::parse_error))]
    ParseError {
        msg: String,
        #[source_code]
        input: String,
        #[label("error occurred here")]
        span: Option<(usize, usize)>,
    }
}