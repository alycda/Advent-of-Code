use crate::AocError;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Wall,
    Empty,
    Box,
    Robot,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            'O' => Cell::Box,
            '@' => Cell::Robot,
            _ => panic!("Invalid cell character"),
        }
    }
}

// Add new conversion for expanded format
impl From<&str> for Cell {
    fn from(s: &str) -> Self {
        match s {
            "##" => Cell::Wall,
            ".." => Cell::Empty,
            "[]" => Cell::Box,
            "@" => Cell::Robot,
            _ => panic!("Invalid cell string"),
        }
    }
}

impl From<Cell> for char {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Wall => '#',
            Cell::Empty => '.',
            Cell::Box => 'O',
            Cell::Robot => '@',
        }
    }
}

impl From<Cell> for String {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Wall => "##".to_string(),
            Cell::Empty => "..".to_string(),
            Cell::Box => "[]".to_string(),
            Cell::Robot => "@.".to_string(),
        }
    }
}

struct Warehouse<T>(Vec<Vec<T>>);


impl<T> std::ops::Deref for Warehouse<T> {
    type Target = Vec<Vec<T>>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Warehouse<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> FromIterator<Vec<T>> for Warehouse<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        Warehouse(iter.into_iter().collect())
    }
}

impl Warehouse<char> {
    fn expand(&self) -> Warehouse<String> {
        // let rows = self.0.len();
        // let expanded: Vec<Vec<Cell>> = Vec::with_capacity(rows);
        
        // self.0
        //     .iter()
        //     .map(|row| {
        //         row.iter()
        //             .map(|&cell| {
        //                 // First convert char to Cell using From<char>
        //                 let cell: Cell = Cell::from(cell);
        //                 // Then match on the Cell

        //                 match cell {
        //                     Cell::Wall => "##".to_string(),
        //                     Cell::Empty => "..".to_string(),
        //                     Cell::Box => "[]".to_string(),
        //                     // this assumes the robot is not next to any boxes, and its not but it's a bad assumption in production
        //                     Cell::Robot => "@.".to_string(),
        //                 }
        //             })
        //             .collect::<Vec<String>>()
        //     })
        //     .collect::<Vec<Vec<String>>>();

        let expanded = self.0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&c| {
                        match Cell::from(c) {
                            Cell::Wall => WALL_EXPANDED.to_string(),
                            Cell::Empty => EMPTY_EXPANDED.to_string(),
                            Cell::Box => BOX_EXPANDED.to_string(),
                            Cell::Robot => "@.".to_string(),
                        }
                    })
                    .collect::<Vec<String>>()  // Explicitly collect to Vec<String>
            })
            .collect::<Vec<Vec<String>>>();
        
        Warehouse(expanded)
    }
}

const BOX_EXPANDED: &str = "[]";
const ROBOT_EXPANDED: &str = "@";
const WALL_EXPANDED: &str = "##";
const EMPTY_EXPANDED: &str = "..";

type Position = (usize, usize);

fn parse_input(input: &str) -> (Warehouse<char>, String) {
    let mut parts = input.split("\n\n");
    let warehouse = Warehouse(
        parts
            .next()
            .unwrap()
            .lines()
            .map(|line| line.chars().collect())  // Just collect chars, don't convert to Cell yet
            .collect()
    );
    let instructions = parts.next().unwrap().trim().to_string();
    (warehouse, instructions)
}

fn find_robot(warehouse: &Warehouse<String>) -> Position {
    for (i, row) in warehouse.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell == ROBOT_EXPANDED {  // Looking for just "@" after expansion
                return (i, j);
            }
        }
    }
    panic!("Robot not found");
}

fn is_valid_pos(x: i32, y: i32, warehouse: &Warehouse::<String>) -> bool {
    x >= 0 && x < warehouse.len() as i32 && y >= 0 && y < warehouse[0].len() as i32
}

fn calculate_total_gps(warehouse: &Warehouse::<String>) -> u32 {
    let mut total = 0;
    for (i, row) in warehouse.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == BOX_EXPANDED {
                total += 100 * i as u32 + j as u32;
            }
        }
    }
    total
}


#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (mut warehouse, instructions) = parse_input(&input);
    let warehouse = warehouse.expand();
    let mut robot_pos = find_robot(&warehouse);

    instructions.chars()
        .filter(|c| !c.is_whitespace())
        .for_each(|inst| {

        let (dx, dy) = match inst {
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            // _ => (0, 0),
            _ => panic!("Invalid instruction: {inst}"),
        };
        
        let (rx, ry) = robot_pos;
        let (nx, ny) = (rx as i32 + dx, ry as i32 + dy);
        
        if is_valid_pos(nx, ny, &warehouse) {
            if warehouse[nx as usize][ny as usize] == Cell::Empty {
                warehouse[rx][ry] = Cell::Empty;
                warehouse[nx as usize][ny as usize] = Cell::Robot;
                robot_pos = (nx as usize, ny as usize);
            } else if warehouse[nx as usize][ny as usize] == Cell::Box {
                let mut tx = nx;
                let mut ty = ny;
                while is_valid_pos(tx + dx, ty + dy, &warehouse) && warehouse[(tx + dx) as usize][(ty + dy) as usize] == Cell::Box {
                    tx += dx;
                    ty += dy;
                }
                
                if is_valid_pos(tx + dx, ty + dy, &warehouse) && warehouse[(tx + dx) as usize][(ty + dy) as usize] == Cell::Empty {
                    warehouse[(tx + dx) as usize][(ty + dy) as usize] = Cell::Box;
                    warehouse[nx as usize][ny as usize] = Cell::Robot;
                    warehouse[rx][ry] = Cell::Empty;
                    robot_pos = (nx as usize, ny as usize);
                }
            }
        }
    });
    
    Ok(calculate_total_gps(&warehouse).to_string())  
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
    fn new_map() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########";

        let expected = "####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################";

        let (mut warehouse, _) = parse_input(&input);

        assert_eq!(expected, warehouse.expand().0.iter().collect::<String>());
    }

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
//         assert_eq!("9021", process(input)?);
//         Ok(())
//     }
}
