use std::ops::Add;

use glam::IVec2;
use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    multi::{many1, separated_list1},
    IResult, Parser,
};

use crate::AocError;

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Wall,
    Empty,
    Box,
    Robot,
}

// parse individual cells
fn cell(input: &str) -> IResult<&str, Cell> {
    alt((
        char('#').map(|_| Cell::Wall),
        char('.').map(|_| Cell::Empty),
        char('O').map(|_| Cell::Box),
        char('@').map(|_| Cell::Robot),
    ))(input)
}

// Parse a single row of the warehouse
fn grid_row(input: &str) -> IResult<&str, Vec<Cell>> {
    many1(cell)(input)
}

// Parse the entire warehouse grid
fn warehouse(input: &str) -> IResult<&str, Vec<Vec<Cell>>> {
    separated_list1(line_ending, grid_row)(input)
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
}

fn movement(input: &str) -> IResult<&str, Direction> {
    alt((
        char('^').map(|_| Direction::Up),
        char('v').map(|_| Direction::Down),
        char('<').map(|_| Direction::Left),
        char('>').map(|_| Direction::Right),
    ))(input)
}

fn movements(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(movement)(input)
}

#[derive(Debug)]
struct GameState {
    // warehouse: Vec<Vec<Cell>>,
    warehouse: Warehouse,
    movements: Vec<Direction>,
}

#[derive(Debug, Clone, Copy)]
struct Robot(IVec2);

impl Robot {
    fn apply_delta(&self, delta: IVec2) -> Option<IVec2> {
        let new_pos = *self + delta;
        Some(new_pos)
    }
}

impl std::ops::Deref for Robot {
    type Target = IVec2;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// support Robot + IVec2
impl Add<IVec2> for Robot {
    type Output = IVec2;  // Note: returns IVec2, not Robot
    
    fn add(self, rhs: IVec2) -> Self::Output {
        self.0 + rhs
    }
}

// If we need &Robot + IVec2
impl Add<IVec2> for &Robot {
    type Output = IVec2;
    
    fn add(self, rhs: IVec2) -> Self::Output {
        self.0 + rhs
    }
}

#[derive(Debug)]
struct Warehouse {
    grid: Vec<Vec<Cell>>,
    robot: Robot,
    // Optionally track box positions separately for easier GPS calculation later
    boxes: Vec<IVec2>,
}

impl Warehouse {
    fn from_grid(grid: Vec<Vec<Cell>>) -> Result<Self, AocError> {
        let mut robot = None;
        let mut boxes = Vec::new();
        
        // Scan grid to find robot and boxes
        for (row, line) in grid.iter().enumerate() {
            for (col, cell) in line.iter().enumerate() {
                match cell {
                    Cell::Robot => {
                        if robot.is_some() {
                            return Err(AocError::ParseError("Multiple robots found".into()));
                        }
                        robot = Some(Robot(IVec2::new(col as i32, row as i32)))
                    }
                    Cell::Box => boxes.push(IVec2::new(col as i32, row as i32)),
                    _ => {}
                }
            }
        }
        
        let robot = robot.ok_or_else(|| AocError::ParseError("No robot found".into()))?;
        
        Ok(Warehouse { grid, robot, boxes })
    }

    // Helper method to check if a position is within bounds and not a wall
    fn is_valid_move(&self, pos: IVec2) -> bool {
        pos.y < self.grid.len() as i32
            && pos.x < self.grid[0].len() as i32
            && self.grid[pos.y as usize][pos.x as usize] != Cell::Wall
    }

    // need this to calculate GPS coordinates
    fn dimensions(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

    fn try_move(&mut self, movement: Direction) {
        let delta = movement.to_position();
        
        // First, check if the next position is even valid
        let next_robot_pos = match self.robot.apply_delta(delta) {
            Some(pos) if self.is_valid_move(pos) => pos,
            _ => return,
        };
    
        // Collect any boxes that need to be moved
        let mut boxes_to_move = Vec::new();
        let mut check_pos = next_robot_pos;
        
        // Keep checking positions until we find an empty space or hit a wall
        while let Some(box_idx) = self.boxes.iter().position(|&p| p == check_pos) {
            let next_pos = check_pos + delta;
            // If we can't move to next position, abort the whole move
            if !self.is_valid_move(next_pos) {
                return;
            }
            boxes_to_move.push((box_idx, next_pos));
            check_pos = next_pos;
        }
    
        // Move all boxes (in reverse order to avoid overwriting)
        for (box_idx, new_pos) in boxes_to_move.into_iter().rev() {
            self.boxes[box_idx] = new_pos;
        }
    
        // Move robot
        self.robot = Robot(next_robot_pos);
    }

    // compute GPS coordinate for a position
    fn gps_coordinate(&self, pos: IVec2) -> u32 {
        let (rows, _) = self.dimensions();
        // Remember to include the walls in the count
        ((pos.y) * 100 + pos.x) as u32
    }

    // Calculate sum of all box GPS coordinates
    fn total_gps_score(&self) -> u32 {
        self.boxes.iter()
            .map(|&pos| self.gps_coordinate(pos))
            .sum()
    }

    // for debugging
    fn display(&self) -> String {
        let mut display = self.grid.clone();
        // Clear previous positions
        for row in display.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == Cell::Robot || *cell == Cell::Box {
                    *cell = Cell::Empty;
                }
            }
        }
        
        // Place boxes and robot
        for &pos in &self.boxes {
            display[pos.y as usize][pos.x as usize] = Cell::Box;
        }
        display[self.robot.y as usize][self.robot.x as usize] = Cell::Robot;

        display.iter()
            .map(|row| row.iter()
                .map(|cell| match cell {
                    Cell::Wall => '#',
                    Cell::Empty => '.',
                    Cell::Box => 'O',
                    Cell::Robot => '@',
                })
                .collect::<String>()
            )
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn parse_input(input: &str) -> IResult<&str, GameState> {
    let (input, warehouse_grid) = warehouse(input)?;
    let (input, _) = many1(line_ending)(input)?;  // Skip empty lines
    let (input, moves) = movements(input)?;
    
    Ok((input, GameState {
        warehouse: Warehouse::from_grid(warehouse_grid).unwrap(),
        movements: moves,
    }))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, mut game_state) = parse_input(input).unwrap();

    println!("Initial state:\n{}\n", game_state.warehouse.display());
    
    for (i, movement) in game_state.movements.iter().enumerate() {
        game_state.warehouse.try_move(*movement);
        println!("After move {}: {:?}\n{}\n", 
            i + 1, 
            movement, 
            game_state.warehouse.display()
        );
    }

    let output = &game_state.warehouse.total_gps_score();
    Ok(output.to_string())
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
    fn test_process_small() -> miette::Result<()> {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!("2028", process(input)?);
        Ok(())
    }

    #[test]
    fn test_box_chain_push() {
let input = "##########\n#...@OOO.#\n#........#\n##########\n\n>>>>>";
let expected = "##########\n#....@OOO#\n#........#\n##########";
        
        let (_, mut game_state) = parse_input(input).unwrap();
        for movement in game_state.movements {
            game_state.warehouse.try_move(movement);
        }
        assert_eq!(game_state.warehouse.display().trim(), expected.trim());
    }

    #[test]
    fn test_box_chain_push_3() {
let input = "##########
#OOO..@..#
#........#
##########

<<<";
let expected = "##########
#OOO@....#
#........#
##########";
        
        let (_, mut game_state) = parse_input(input).unwrap();
        for movement in game_state.movements {
            game_state.warehouse.try_move(movement);
        }
        assert_eq!(game_state.warehouse.display().trim(), expected.trim());
    }

//     #[test]
//     fn test_box_chain_push_vertical() {
// let expected = "##########\n#....O...#\n#....O...#\n#....@...#\n#........#\n##########\n\n^^";
// let input = "##########\n#........#\n#....O...#\n#....O...#\n#....@...#\n##########";
        
//         let (_, mut game_state) = parse_input(input).unwrap();
//         for movement in game_state.movements {
//             game_state.warehouse.try_move(movement);
//         }
//         assert_eq!(game_state.warehouse.display().trim(), expected.trim());
//     }

//     #[test]
//     fn test_box_chain_push_2() {
// let input = "##########
// #..O..O.O#
// #......O.#
// #.OO..O.O#
// #..O@..O.#
// #O#..O...#
// #O..O..O.#
// #.OO.O.OO#
// #....O...#
// ##########

// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
// let expected = "##########
// #.O.O.OOO#
// #........#
// #OO......#
// #OO@.....#
// #O#.....O#
// #O.....OO#
// #O.....OO#
// #OO....OO#
// ##########";
        
//         let (_, mut game_state) = parse_input(input).unwrap();
//         for movement in game_state.movements {
//             game_state.warehouse.try_move(movement);
//         }
//         assert_eq!(game_state.warehouse.display().trim(), expected.trim());
//     }

//     #[test]
//     fn test_process_large() -> miette::Result<()> {
//         let input = "##########
// #..O..O.O#
// #......O.#
// #.OO..O.O#
// #..O@..O.#
// #O#..O...#
// #O..O..O.#
// #.OO.O.OO#
// #....O...#
// ##########

// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
//         assert_eq!("10092", process(input)?);
//         Ok(())
//     }
}
