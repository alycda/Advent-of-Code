use crate::AocError;

use std::collections::HashSet;

type Position = (usize, usize);

struct Warehouse(Vec<Vec<char>>);

impl std::ops::Deref for Warehouse {
    type Target = Vec<Vec<char>>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Warehouse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

const EXPAND_MAP: [(&str, &str); 4] = [
    ("#", "##"),
    ("O", "[]"),
    (".", ".."),
    ("@", "@."),
];

fn parse_input(input: &str) -> (Vec<Vec<char>>, Position, String) {
    let mut parts = input.split("\n\n");
    let box_part = parts.next().unwrap();
    let mut robot_start = None;
    
    let warehouse = box_part
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                if c == '@' {
                    robot_start = Some((i, 2 * j));
                }
                let expanded = EXPAND_MAP.iter()
                    .find(|(from, _)| from.chars().next().unwrap() == c)
                    .map(|(_, to)| to.chars())
                    .unwrap();
                row.extend(expanded);
            }
            Some(row)
        })
        .collect();

    let instructions = parts.next().unwrap().replace('\n', "");
    (warehouse, robot_start.unwrap(), instructions)
}

const DIR_MAP: [(char, (i32, i32)); 4] = [
    ('v', (1, 0)),
    ('<', (0, -1)),
    ('>', (0, 1)),
    ('^', (-1, 0)),
];

fn calculate_total_gps(warehouse: &[Vec<char>]) -> u32 {
    let mut total = 0;
    for (i, line) in warehouse.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if c == '[' {
                total += 100 * i as u32 + j as u32;
            }
        }
    }
    total
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (mut warehouse, mut robot_pos, instructions) = parse_input(input);
    
    for inst in instructions.chars() {
        let (dx, dy) = DIR_MAP.iter()
            .find(|(c, _)| *c == inst)
            .map(|(_, dir)| *dir)
            .unwrap();
            
        let (rx, ry) = robot_pos;
        let nx = (rx as i32 + dx) as usize;
        let ny = (ry as i32 + dy) as usize;
        
        if nx >= warehouse.len() || ny >= warehouse[0].len() {
            continue;
        }

        match warehouse[nx][ny] {
            '.' => {
                warehouse[rx][ry] = '.';
                warehouse[nx][ny] = '@';
                robot_pos = (nx, ny);
            }
            '#' => continue,
            _ => {
                if dx == 0 {
                    // Horizontal box pushing
                    let mut tx = nx;
                    let mut ty = ny;
                    let mut dist = 0;
                    
                    // When moving right
                    if dy > 0 {
                        while ty < warehouse[0].len() && 
                              (warehouse[tx][ty] == '[' || warehouse[tx][ty] == ']') {
                            dist += 1;
                            ty += 1;
                        }
                        
                        if ty >= warehouse[0].len() || warehouse[tx][ty] == '#' {
                            continue;
                        }
                        
                        // Push from right to left
                        for _ in 0..dist {
                            warehouse[tx][ty] = warehouse[tx][ty - 1];
                            ty -= 1;
                        }
                    } else {
                        // When moving left
                        while ty > 0 && 
                              (warehouse[tx][ty] == '[' || warehouse[tx][ty] == ']') {
                            dist += 1;
                            ty -= 1;
                        }
                        
                        if ty == 0 || warehouse[tx][ty] == '#' {
                            continue;
                        }
                        
                        // Push from left to right
                        for _ in 0..dist {
                            warehouse[tx][ty] = warehouse[tx][ty + 1];
                            ty += 1;
                        }
                    }
                    
                    warehouse[nx][ny] = '@';
                    warehouse[rx][ry] = '.';
                    robot_pos = (nx, ny);
                } else {
                    // Vertical box pushing
                    let mut to_push: Vec<HashSet<Position>> = vec![HashSet::from([(rx, ry)])];
                    let mut no_wall = true;
                    let mut all_empty = false;
                    
                    while no_wall && !all_empty {
                        let mut next_push = HashSet::new();
                        all_empty = true;
                        
                        for &(cx, cy) in &to_push[to_push.len() - 1] {
                            if warehouse[cx][cy] == '.' {
                                continue;
                            }
                            
                            let tx = (cx as i32 + dx) as usize;
                            let ty = cy;
                            
                            if warehouse[tx][ty] != '.' {
                                all_empty = false;
                            }
                            
                            next_push.insert((tx, ty));
                            
                            match warehouse[tx][ty] {
                                '#' => {
                                    no_wall = false;
                                    break;
                                }
                                '[' => { next_push.insert((tx, ty + 1)); }
                                ']' => { next_push.insert((tx, ty - 1)); }
                                _ => {}
                            }
                        }
                        
                        if !no_wall {
                            break;
                        }
                        
                        to_push.push(next_push);
                    }
                    
                    if !no_wall {
                        continue;
                    }
                    
                    for i in (1..to_push.len()).rev() {
                        for (cx, cy) in &to_push[i] {
                            let fx = (*cx as i32 - dx) as usize;
                            let fy = *cy;
                            
                            if to_push[i - 1].contains(&(fx, fy)) {
                                warehouse[*cx][*cy] = warehouse[fx][fy];
                            } else {
                                warehouse[*cx][*cy] = '.';
                            }
                        }
                    }
                    
                    warehouse[rx][ry] = '.';
                    robot_pos = (nx, ny);
                }
            }
        }
    }

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

//     #[test]
//     fn new_map() {
//         let input = "##########
// #..O..O.O#
// #......O.#
// #.OO..O.O#
// #..O@..O.#
// #O#..O...#
// #O..O..O.#
// #.OO.O.OO#
// #....O...#
// ##########";

//         let expected = "####################
// ##....[]....[]..[]##
// ##............[]..##
// ##..[][]....[]..[]##
// ##....[]@.....[]..##
// ##[]##....[]......##
// ##[]....[]....[]..##
// ##..[][]..[]..[][]##
// ##........[]......##
// ####################";

//         let (mut warehouse, _) = parse_input(&input);

//         assert_eq!(expected, warehouse.expand().0.iter().collect::<String>());
//     }

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
        assert_eq!("9021", process(input)?);
        Ok(())
    }
}
