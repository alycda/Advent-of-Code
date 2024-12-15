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

type Warehouse = Vec<Vec<Cell>>;
type Position = (usize, usize);

fn parse_input(input: &str) -> (Warehouse, String) {
    let mut parts = input.split("\n\n");
    let warehouse = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().map(Cell::from).collect())
        .collect();
    let instructions = parts.next().unwrap().trim().to_string();
    (warehouse, instructions)
}

fn find_robot(warehouse: &Warehouse) -> Position {
    for (i, row) in warehouse.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == Cell::Robot {
                return (i, j);
            }
        }
    }
    panic!("Robot not found");
}

fn is_valid_pos(x: i32, y: i32, warehouse: &Warehouse) -> bool {
    x >= 0 && x < warehouse.len() as i32 && y >= 0 && y < warehouse[0].len() as i32
}

fn calculate_total_gps(warehouse: &Warehouse) -> u32 {
    let mut total = 0;
    for (i, row) in warehouse.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == Cell::Box {
                total += 100 * i as u32 + j as u32;
            }
        }
    }
    total
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (mut warehouse, instructions) = parse_input(&input);
    let mut robot_pos = find_robot(&warehouse);
    
    // dbg!(&instructions);

    instructions.chars()
        // .enumerate()
        .filter(|c| !c.is_whitespace())
        .for_each(|inst| {
        // if idx == 70 {
        //     panic!("{inst}");
        // }

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
    fn test_process_large() -> miette::Result<()> {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!("10092", process(input)?);
        Ok(())
    }
}
