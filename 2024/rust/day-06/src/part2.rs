use std::collections::{HashMap, HashSet};

use tracing::instrument;

use crate::custom_error::AocError;

// Represents an ABSOLUTE position in the grid (COL, ROW)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize, 
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn get_col(&self) -> usize {
        self.x
    }

    fn get_row(&self) -> usize {
        self.y
    }
}

/// TODO replace with [glam::IVec2]. Rename this to NEIGHBOR
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up(Position, Option<char>), // A
    Down(Position, Option<char>), // X
    Left(Position, Option<char>), // #
    Right(Position, Option<char>), // O
    // TopLeft(Position, Option<char>), // A#
    // TopRight(Position, Option<char>), // AO
    // BottomLeft(Position, Option<char>), // X#
    // BottomRight(Position, Option<char>), // XO
}

impl Direction {
    fn is_up(&self) -> bool {
        matches!(self, Direction::Up(_, _))
    }
    
    fn is_right(&self) -> bool {
        matches!(self, Direction::Right(_, _))
    }
    
    fn is_down(&self) -> bool {
        matches!(self, Direction::Down(_, _))
    }
    
    fn is_left(&self) -> bool {
        matches!(self, Direction::Left(_, _))
    }
}

/// COL, ROW
#[derive(Debug, PartialEq, Eq)]
struct Grid(usize, usize, String, Vec<(Position, Direction)>);

impl Grid {
    fn new(cols: usize, rows: usize, grid: String) -> Self {
        Self(cols, rows, grid, Vec::new())
    }

    #[instrument]
    fn to_position(&self, idx: usize) -> Position {
        // panic!("{idx}");

        let cols = self.get_cols();
        // let chars_per_row = cols + 1;
        let col = idx % cols;
        let row = idx / cols;
        Position::new(col, row)
    }

    // chars per row is cols + 1
    fn get_cols(&self) -> usize {
        self.0 + 1
    }

    fn get_rows(&self) -> usize {
        self.1
    }

    /// accounts for newline characters
    #[instrument]
    fn to_idx(&self, pos: &Position) -> usize {
        // let chars_per_row = self.get_cols() + 1;

        self.get_cols() * pos.get_row() + pos.get_col()
    }

    #[instrument]
    fn get_char(&self, pos: &Position) -> char {
        self.2.as_bytes()[self.to_idx(pos)] as char
    }

    fn insert(&mut self, pos: Position, cardinal: Direction) {
        // self.2.remove(self.to_idx(&pos));
        // self.2.insert(self.to_idx(&pos), 'X');

        // dbg!(self.3.insert(dbg!(pos)));
        // self.3.insert(pos);
        self.3.push((pos, cardinal));
    }

    fn get_neighbors(&self, pos: Position) -> HashMap<&str, (Position, char)> {
        let mut neighbors = Vec::new();
        let cols = self.get_cols();
        // let chars_per_row = cols + 1;
        let rows = self.get_rows();
        let col = pos.get_col();
        let row = pos.get_row();
        // let char = dbg!(self.get_char(pos));

        // Check up (decrease Y)
        if row > 0 {
            let neighbor = Position::new(col, row - 1);
            neighbors.push(Direction::Up(neighbor, Some(self.get_char(&neighbor))));
        }
        // Check down (increase Y)
        if row + 1 < rows {
            let neighbor = Position::new(col, row + 1);
            neighbors.push(Direction::Down(neighbor, Some(self.get_char(&neighbor))));
        }
        // Check left (decrease X)
        if col > 0 {
            let neighbor = Position::new(col - 1, row);
            neighbors.push(Direction::Left(neighbor, Some(self.get_char(&neighbor))));
        }
        // Check right (increase X)
        if col + 1 < cols {
            let neighbor = Position::new(col + 1, row);
            neighbors.push(Direction::Right(neighbor, Some(self.get_char(&neighbor))));
        }
        neighbors.iter().fold(HashMap::new(), |mut map, dir| {
            match dir {
                Direction::Up(new_pos, char) => { map.insert("Up", (*new_pos, char.unwrap_or('?'))); }
                Direction::Right(new_pos, char) => { map.insert("Right", (*new_pos, char.unwrap_or('?'))); }
                Direction::Down(new_pos, char) => { map.insert("Down", (*new_pos, char.unwrap_or('?'))); }
                Direction::Left(new_pos, char) => { map.insert("Left", (*new_pos, char.unwrap_or('?'))); }
            }
    
            map
        })
    }

    fn find_cycles(&self, ) -> usize {
        // exit_path: &[(Position, Direction)]
        let exit_path = &self.3;

        let mut valid_obstacles = HashSet::new();

        for (idx, (pos, dir)) in exit_path.iter().enumerate() {
            // Try placing obstacle here (excluding start position)
            if idx == 0 { continue; }
            
            // Simulate path with this obstacle
            if self.would_create_cycle(pos, exit_path, idx) {
                valid_obstacles.insert(*pos);
            }
        }

        valid_obstacles.len()
    }

    fn get_forced_direction(&self, obstacle: &Position, incoming_dir: Direction) -> Direction {
        // When we hit obstacle, we must turn right relative to our incoming direction
        // Get neighbors of the obstacle position
        let neighbors = self.get_neighbors(*obstacle);
        
        match incoming_dir {
            Direction::Up(_, _) => {
                // If we were going Up and hit obstacle, we must go Right
                if let Some((right_pos, _)) = neighbors.get("Right") {
                    Direction::Right(*right_pos, None)
                } else {
                    // Handle case where right isn't available
                    // Could return None or handle differently depending on your needs
                    Direction::Right(*obstacle, None)  // Fallback
                }
            },
            Direction::Right(_, _) => {
                // If we were going Right and hit obstacle, we must go Down
                if let Some((down_pos, _)) = neighbors.get("Down") {
                    Direction::Down(*down_pos, None)
                } else {
                    Direction::Down(*obstacle, None)
                }
            },
            Direction::Down(_, _) => {
                // If we were going Down and hit obstacle, we must go Left
                if let Some((left_pos, _)) = neighbors.get("Left") {
                    Direction::Left(*left_pos, None)
                } else {
                    Direction::Left(*obstacle, None)
                }
            },
            Direction::Left(_, _) => {
                // If we were going Left and hit obstacle, we must go Up
                if let Some((up_pos, _)) = neighbors.get("Up") {
                    Direction::Up(*up_pos, None)
                } else {
                    Direction::Up(*obstacle, None)
                }
            }
        }
    }

    // fn get_forced_direction(&self, obstacle: &Position, incoming_dir: Direction) -> Direction {
    //     // When we hit obstacle, we must turn right relative to our incoming direction
    //     match incoming_dir {
    //         Direction::Up(_, _) => {
    //             // If we were going Up and hit obstacle, we must go Right
    //             Direction::Right(
    //                 Position::new(obstacle.x + 1, obstacle.y), 
    //                 None
    //             )
    //         },
    //         Direction::Right(_, _) => {
    //             // If we were going Right and hit obstacle, we must go Down
    //             Direction::Down(
    //                 Position::new(obstacle.x, obstacle.y + 1),
    //                 None
    //             )
    //         },
    //         Direction::Down(_, _) => {
    //             // If we were going Down and hit obstacle, we must go Left
    //             Direction::Left(
    //                 Position::new(obstacle.x - 1, obstacle.y),
    //                 None
    //             )
    //         },
    //         Direction::Left(_, _) => {
    //             // If we were going Left and hit obstacle, we must go Up
    //             Direction::Up(
    //                 Position::new(obstacle.x, obstacle.y - 1),
    //                 None
    //             )
    //         }
    //     }
    // }

    fn would_create_cycle(&self, obstacle: &Position, path: &[(Position, Direction)], step: usize) -> bool {
        let mut tortoise = *obstacle;  // Start at obstacle position
        let mut hare = *obstacle;
        
        // Get initial forced direction when hitting obstacle
        let mut tortoise_dir = self.get_forced_direction(obstacle, path[step].1);
        let mut hare_dir = tortoise_dir;

        // Floyd's algorithm
        loop {
            // Move tortoise one step
            if let Some((next_pos, next_dir)) = self.next_move(tortoise, tortoise_dir) {
                tortoise = next_pos;
                tortoise_dir = next_dir;
            } else {
                return false;  // Hit dead end or exit
            }

            // Move hare two steps
            for _ in 0..2 {
                if let Some((next_pos, next_dir)) = self.next_move(hare, hare_dir) {
                    hare = next_pos;
                    hare_dir = next_dir;
                } else {
                    return false;  // Hit dead end or exit
                }
            }

            if tortoise == hare {
                return true;  // Found cycle!
            }
        }
    }

    fn next_move(&self, current_pos: Position, current_dir: Direction) -> Option<(Position, Direction)> {
        let neighbors = self.get_neighbors(current_pos);
        
        match current_dir {
            Direction::Up(_, _) => {
                if let Some((pos, cell)) = neighbors.get("Up") {
                    match cell {
                        '.' | 'X' | '^' => {
                            // Just pass the position we're moving to
                            Some((*pos, Direction::Up(*pos, None)))
                        },
                        '#' => {
                            if let Some((right_pos, right_cell)) = neighbors.get("Right") {
                                match right_cell {
                                    '.' | 'X' | '^' => {
                                        Some((*right_pos, Direction::Right(*right_pos, None)))
                                    },
                                    _ => None
                                }
                            } else {
                                None
                            }
                        },
                        _ => None
                    }
                } else {
                    None
                }
            },
            Direction::Right(_, _) => {
                if let Some((pos, cell)) = neighbors.get("Right") {
                    match cell {
                        '.' | 'X' | '^' => {
                            Some((*pos, Direction::Right(*pos, None)))
                        },
                        '#' => {
                            if let Some((down_pos, down_cell)) = neighbors.get("Down") {
                                match down_cell {
                                    '.' | 'X' | '^' => {
                                        Some((*down_pos, Direction::Down(*down_pos, None)))
                                    },
                                    _ => None
                                }
                            } else {
                                None
                            }
                        },
                        _ => None
                    }
                } else {
                    None
                }
            },
            Direction::Down(_, _) => {
                if let Some((pos, cell)) = neighbors.get("Down") {
                    match cell {
                        '.' | 'X' | '^' => {
                            Some((*pos, Direction::Down(*pos, None)))
                        },
                        '#' => {
                            if let Some((left_pos, left_cell)) = neighbors.get("Left") {
                                match left_cell {
                                    '.' | 'X' | '^' => {
                                        Some((*left_pos, Direction::Left(*left_pos, None)))
                                    },
                                    _ => None
                                }
                            } else {
                                None
                            }
                        },
                        _ => None
                    }
                } else {
                    None
                }
            },
            Direction::Left(_, _) => {
                if let Some((pos, cell)) = neighbors.get("Left") {
                    match cell {
                        '.' | 'X' | '^' => {
                            Some((*pos, Direction::Left(*pos, None)))
                        },
                        '#' => {
                            if let Some((up_pos, up_cell)) = neighbors.get("Up") {
                                match up_cell {
                                    '.' | 'X' | '^' => {
                                        Some((*up_pos, Direction::Up(*up_pos, None)))
                                    },
                                    _ => None
                                }
                            } else {
                                None
                            }
                        },
                        _ => None
                    }
                } else {
                    None
                }
            }
        }
    }

    // fn next_move(&self, current_pos: Position, current_dir: Direction) -> Option<(Position, Direction)> {
    //     // First get all valid neighbors
    //     let neighbors = self.get_neighbors(current_pos);
        
    //     // Try to continue in current direction first (following relative right-turn rules)
    //     match current_dir {
    //         Direction::Up(_, _) => {
    //             if let Some((pos, cell)) = neighbors.get("Up") {
    //                 match cell {
    //                     '.' | 'X' | '^' => {
    //                         // Can continue up
    //                         Some((*pos, Direction::Up(
    //                             Position::new(pos.x, pos.y - 1),
    //                             None
    //                         )))
    //                     },
    //                     '#' => {
    //                         // Hit wall, must turn right
    //                         if let Some((right_pos, right_cell)) = neighbors.get("Right") {
    //                             match right_cell {
    //                                 '.' | 'X' | '^' => {
    //                                     Some((*right_pos, Direction::Right(
    //                                         Position::new(right_pos.x + 1, right_pos.y),
    //                                         None
    //                                     )))
    //                                 },
    //                                 _ => None // Can't go right either
    //                             }
    //                         } else {
    //                             None
    //                         }
    //                     },
    //                     _ => None
    //                 }
    //             } else {
    //                 None
    //             }
    //         },
    //         Direction::Right(_, _) => {
    //             if let Some((pos, cell)) = neighbors.get("Right") {
    //                 match cell {
    //                     '.' | 'X' | '^' => {
    //                         Some((*pos, Direction::Right(
    //                             Position::new(pos.x + 1, pos.y),
    //                             None
    //                         )))
    //                     },
    //                     '#' => {
    //                         // Hit wall, must turn right (down)
    //                         if let Some((down_pos, down_cell)) = neighbors.get("Down") {
    //                             match down_cell {
    //                                 '.' | 'X' | '^' => {
    //                                     Some((*down_pos, Direction::Down(
    //                                         Position::new(down_pos.x, down_pos.y + 1),
    //                                         None
    //                                     )))
    //                                 },
    //                                 _ => None
    //                             }
    //                         } else {
    //                             None
    //                         }
    //                     },
    //                     _ => None
    //                 }
    //             } else {
    //                 None
    //             }
    //         },
    //         Direction::Down(_, _) => {
    //             if let Some((pos, cell)) = neighbors.get("Down") {
    //                 match cell {
    //                     '.' | 'X' | '^' => {
    //                         Some((*pos, Direction::Down(
    //                             Position::new(pos.x, pos.y + 1),
    //                             None
    //                         )))
    //                     },
    //                     '#' => {
    //                         // Hit wall, must turn right (left)
    //                         if let Some((left_pos, left_cell)) = neighbors.get("Left") {
    //                             match left_cell {
    //                                 '.' | 'X' | '^' => {
    //                                     Some((*left_pos, Direction::Left(
    //                                         Position::new(left_pos.x - 1, left_pos.y),
    //                                         None
    //                                     )))
    //                                 },
    //                                 _ => None
    //                             }
    //                         } else {
    //                             None
    //                         }
    //                     },
    //                     _ => None
    //                 }
    //             } else {
    //                 None
    //             }
    //         },
    //         Direction::Left(_, _) => {
    //             if let Some((pos, cell)) = neighbors.get("Left") {
    //                 match cell {
    //                     '.' | 'X' | '^' => {
    //                         Some((*pos, Direction::Left(
    //                             Position::new(pos.x - 1, pos.y),
    //                             None
    //                         )))
    //                     },
    //                     '#' => {
    //                         // Hit wall, must turn right (up)
    //                         if let Some((up_pos, up_cell)) = neighbors.get("Up") {
    //                             match up_cell {
    //                                 '.' | 'X' | '^' => {
    //                                     Some((*up_pos, Direction::Up(
    //                                         Position::new(up_pos.x, up_pos.y - 1),
    //                                         None
    //                                     )))
    //                                 },
    //                                 _ => None
    //                             }
    //                         } else {
    //                             None
    //                         }
    //                     },
    //                     _ => None
    //                 }
    //             } else {
    //                 None
    //             }
    //         }
    //     }
    // }

    fn analyze_potential_cycles(&self) -> (HashSet<Position>, HashSet<Position>) {
        // First get crossings (your current method)
        let mut direction_map: HashMap<Position, HashSet<Direction>> = HashMap::new();
        for (pos, dir) in &self.3 {
            direction_map.entry(*pos)
                .or_insert_with(HashSet::new)
                .insert(*dir);
        }
    
        // Get crossing points
        let crossing_points: HashSet<Position> = direction_map.iter()
            .filter(|(_, dirs)| {
                dirs.iter().any(|d| d.is_up()) && dirs.iter().any(|d| d.is_right()) ||
                dirs.iter().any(|d| d.is_right()) && dirs.iter().any(|d| d.is_down()) ||
                dirs.iter().any(|d| d.is_down()) && dirs.iter().any(|d| d.is_left()) ||
                dirs.iter().any(|d| d.is_left()) && dirs.iter().any(|d| d.is_up())
            })
            .map(|(pos, _)| *pos)
            .collect();
    
        // Get neighbor-check points (ordered by path sequence)
        let neighbor_points: HashSet<Position> = self.3.iter()
            .enumerate()  // Add sequence info
            .filter_map(|(idx, (pos, dir))| {
                match dir {
                    Direction::Up(_, _) => {
                        // When going Up, check right neighbor's history
                        let right_pos = Position::new(pos.x + 1, pos.y);
                        // Look back in path before this point
                        if self.3[..idx].iter().any(|(_, prev_dir)| 
                            matches!(prev_dir, Direction::Right(_, _))
                        ) {
                            Some(*pos)
                        } else {
                            None
                        }
                    },
                    // Similar for other directions...
                    _ => None  // Add other cases back
                }
            })
            .collect();
    
        (crossing_points, neighbor_points)
    }

    fn find_stuff(&self) -> usize {
        let (crossing_points, neighbor_points) = self.analyze_potential_cycles();
        
        dbg!("Crossing points:", crossing_points.len());
        dbg!("Neighbor check points:", neighbor_points.len());
        dbg!("Points in both:", crossing_points.intersection(&neighbor_points).count());
        dbg!("Points in either:", crossing_points.union(&neighbor_points).count());
        
        // Return whichever count proves to be correct
        crossing_points.union(&neighbor_points).count()
    }

    // fn find_stuff(&self) -> usize {
    //     // Group by position
    //     let mut visited: HashMap<Position, HashSet<Direction>> = HashMap::new();
                
    //     for (pos, dir) in &self.3 {
    //         visited.entry(*pos)
    //             .or_insert_with(HashSet::new)
    //             .insert(*dir);
    //     }


    //     self.3.iter()
    //         // .take(3)
    //         // .inspect(|(pos, dir)| {
    //         //     dbg!(pos, dir);

    //         //     match dir {
    //         //         Direction::Up => { if let Some(_) = visited. }
    //         //     }

    //         //     dbg!(visited.get(pos));
    //         // })
    //         .filter(|(pos, dir)| {
    //             match dir {
    //                 Direction::Up(next_pos, _) => {
    //                     // When going Up, check right neighbor for Right movement
    //                     let right_pos = Position::new(pos.x + 1, pos.y);
    //                     if let Some(previous_dirs) = visited.get(&right_pos) {
    //                         previous_dirs.iter().any(|prev_dir| 
    //                             matches!(prev_dir, Direction::Right(_, _))
    //                         )
    //                     } else {
    //                         false
    //                     }
    //                 },
    //                 Direction::Right(next_pos, _) => {
    //                     // When going Right, check bottom neighbor for Down movement
    //                     let down_pos = Position::new(pos.x, pos.y + 1);
    //                     if let Some(previous_dirs) = visited.get(&down_pos) {
    //                         previous_dirs.iter().any(|prev_dir| 
    //                             matches!(prev_dir, Direction::Down(_, _))
    //                         )
    //                     } else {
    //                         false
    //                     }
    //                 },
    //                 Direction::Down(next_pos, _) => {
    //                     // When going Down, check left neighbor for Left movement
    //                     let left_pos = Position::new(pos.x - 1, pos.y);
    //                     if let Some(previous_dirs) = visited.get(&left_pos) {
    //                         previous_dirs.iter().any(|prev_dir| 
    //                             matches!(prev_dir, Direction::Left(_, _))
    //                         )
    //                     } else {
    //                         false
    //                     }
    //                 },
    //                 Direction::Left(next_pos, _) => {
    //                     // When going Left, check top neighbor for Up movement
    //                     let up_pos = Position::new(pos.x, pos.y - 1);
    //                     if let Some(previous_dirs) = visited.get(&up_pos) {
    //                         previous_dirs.iter().any(|prev_dir| 
    //                             matches!(prev_dir, Direction::Up(_, _))
    //                         )
    //                     } else {
    //                         false
    //                     }
    //                 }
    //             }
    //         })
    //         .count()
    // }

    fn find_crossings(&mut self) -> usize {
        // Group by position
        let mut direction_map: HashMap<Position, HashSet<Direction>> = HashMap::new();
                
        for (pos, dir) in &self.3 {
            direction_map.entry(*pos)
                .or_insert_with(HashSet::new)
                .insert(*dir);
        }

        // #[cfg(debug_assertions)]
        // dbg!(&direction_map);
        // dbg!(&direction_map.keys().count());

        // direction_map.iter()
        //     .inspect(|(pos, dirs)| {
        //         dbg!(pos, dirs);
        //     }).count();
        // panic!("halt");


        // Find position with both West and North
        direction_map.into_iter()
            .filter(|(_, dirs)| {
                // dirs.contains(&Direction::Up) && dirs.contains(&Direction::Right) ||
                // dirs.contains(&Direction::Right) && dirs.contains(&Direction::Down) ||
                // dirs.contains(&Direction::Down) && dirs.contains(&Direction::Left) ||
                // dirs.contains(&Direction::Left) && dirs.contains(&Direction::Up)
                dirs.iter().any(|d| d.is_up()) && dirs.iter().any(|d| d.is_right()) ||
                dirs.iter().any(|d| d.is_right()) && dirs.iter().any(|d| d.is_down()) ||
                dirs.iter().any(|d| d.is_down()) && dirs.iter().any(|d| d.is_left()) ||
                dirs.iter().any(|d| d.is_left()) && dirs.iter().any(|d| d.is_up())
            }).count()
        //     .find(|(_, dirs)| {
        //         dirs.contains(&Direction::West) && 
        //         dirs.contains(&Direction::North)
        //     })
        //     .map(|(pos, _)| pos).count()

    }

    fn distinct_positions(&self) -> usize {
        self.3.iter().map(|(pos, _)| pos).collect::<HashSet<_>>().len()
    }

    fn exit(&self, pos: Position, dir: char) -> bool {
        let cols = self.get_cols();
        let rows = self.get_rows();
        let col = pos.get_col();
        let row = pos.get_row();

        match dir {
            '^' => {
                if row == 0 {
                    return true;
                }
            },
            'v' => {
                if row + 1 == rows {
                    return true;
                }
            },
            '<' => {
                if col == 0 {
                    return true;
                }
            },
            '>' => {
                if col + 1 == cols {
                    return true;
                }
            }
            _ => todo!()
        }

        false
    }

    fn print(&mut self) {
        // dbg!(&self.3);

        let mut clone = self.2.clone();
        let _ = &self.3.iter().for_each(|(pos, _)| {
            // dbg!(self.to_idx(*pos));
            clone.remove(self.to_idx(pos));
            clone.insert(self.to_idx(pos), 'X');
        });

        // #[cfg(debug_assertions)]
        // self.2 = clone;

        // #[cfg(debug_assertions)]
        // self.2.lines().for_each(|l| {
        //     dbg!(l);
        // }); 
        // dbg!(());       
    }
    
}

// fn _move_forward(mut last_position: &Position, pos: Position, cell: char, neighbors: &mut Option<HashMap<&str, (Position, char)>>, grid: &mut Grid) -> Option<HashMap<&str, (Position, char)>> {
//     let current_pos = last_position;
//     let next_pos = pos;
//     let should_mark_x = cell != 'X';
    
//     // Drop the borrow by setting neighbors to None
//     // neighbors = None;
    
//     // Now we can modify grid
//     if should_mark_x {
//         grid.insert('X', *current_pos);
//     }
//     grid.insert('^', next_pos);
    
//     last_position = &next_pos;
//     // Get new neighbors after modification
//     Some(grid.get_neighbors(next_pos))
// }

// // fn turn_right() {

// // }

// #[instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut peekable = input.lines().peekable();
    let cols = peekable.peek().unwrap().chars().count();
    let rows = peekable
        // .enumerate()
        // .inspect(|(row, line)| { 
        // dbg!(row, line); 

        // // line.chars().enumerate().inspect(|(col, c)| {
        // //     dbg!(row, col, c);
        // // }).count();
        // })
        .count();

    // dbg!(cols, rows); // (3, 4)

    let start = input.find("^");

    let mut grid = Grid::new(cols, rows, input.to_owned());
    let mut last_position = grid.to_position(start.unwrap());

    // dbg!(&last_position);

    let mut neighbors: Option<HashMap<&str, (Position, char)>> = Some(grid.get_neighbors(last_position));
    let mut direction = '^';

    // let mut pass = 0;

    while let Some(ref n) = neighbors {
        // pass += 1;
        // dbg!(pass);
        // // grid.print();
        // grid.2.split("\n").for_each(|c| {
        //     dbg!(c);
        // });

//         dbg!(last_position, &n);
//         // dbg!(last_position);
//         panic!("halt");

        if (grid.exit(last_position, direction)) {
            // dbg!("we are leaving:", last_position, direction);
            break;
        }

        match direction {
            // we start facing up
            '^' => {

                // forward
                if let Some((pos, cell)) = n.get("Up") {
                    match cell {
                        '.' | 'X' | '^' => {
                            // Store the values we need before modifying grid
                            let current_pos = last_position;
                            let next_pos = *pos;

                            // Now we can modify grid
                            grid.insert(current_pos, Direction::Up(next_pos, None));

                            direction = '^';
                            
                            last_position = next_pos;
                            // Get new neighbors after modification
                            neighbors = Some(grid.get_neighbors(next_pos));
                        },
                        // we hit a wall, turn right (relative)
                        '#' => {
                            if let Some((pos, cell)) = n.get("Right") {
                                match cell {
                                    '.' | 'X' | '^' => {

                                        // Store the values we need before modifying grid
                                        let current_pos = last_position;
                                        let next_pos = *pos;

                                        // Now we can modify grid
                                        grid.insert(current_pos, Direction::Right(next_pos, None));

                                        direction = '>';
                                        
                                        last_position = next_pos;
                                        // Get new neighbors after modification
                                        neighbors = Some(grid.get_neighbors(next_pos));
                                    },
                                    '#' => {
                                        // panic!("We should never have a '#' here with a '^' character");
                                        if let Some((pos, cell)) = n.get("Down") {
                                            match cell {
                                                '.' | 'X' | '^' => {
                                                    // Store the values we need before modifying grid
                                                    let current_pos = last_position;
                                                    let next_pos = *pos;

                                                    // Now we can modify grid
                                                    grid.insert(current_pos, Direction::Down(next_pos, None));

                                                    direction = 'v';
                                                    
                                                    last_position = next_pos;
                                                    // Get new neighbors after modification
                                                    neighbors = Some(grid.get_neighbors(next_pos));
                                                },
                                                '#' => {
                                                    if let Some((pos, cell)) = n.get("Left") {
                                                        match cell {
                                                            '.' | 'X' | '^' => {
                                                                // Store the values we need before modifying grid
                                                                let current_pos = last_position;
                                                                let next_pos = *pos;

                                                                // Now we can modify grid
                                                                grid.insert(current_pos, Direction::Left(next_pos, None));

                                                                direction = '<';
                                                                
                                                                last_position = next_pos;
                                                                // Get new neighbors after modification
                                                                neighbors = Some(grid.get_neighbors(next_pos));
                                                            },
                                                            _ => todo!()
                                                        }
                                                    }
                                                }
                                                _ => todo!()
                                            }
                                        }
                                    },
                                    _ => todo!()
                                }
                            } else {
                                todo!("break??");
                            }
                        },
                        unknown => {
                            dbg!(unknown);
                            panic!("unknown character");
                        }

                    }
                } else {
                    break;
                }
            },
            '>' => {

                // forward
                if let Some((pos, cell)) = n.get("Right") {
                    match cell {
                        '.' | 'X' | '^' => {
                            // Store the values we need before modifying grid
                            let current_pos = last_position;
                            let next_pos = *pos;

                            // Now we can modify grid
                            grid.insert(current_pos, Direction::Right(next_pos, None));

                            direction = '>';
                            
                            last_position = next_pos;
                            // Get new neighbors after modification
                            neighbors = Some(grid.get_neighbors(next_pos));
                        }
                        // we hit a wall, turn right (relative)
                        '#' => {
                            if let Some((pos, cell)) = n.get("Down") {
                                match cell {
                                    '.' | 'X' | '^' => {
                                        // Store the values we need before modifying grid
                                        let current_pos = last_position;
                                        let next_pos = *pos;

                                        // Now we can modify grid
                                        grid.insert(current_pos, Direction::Down(next_pos, None));

                                        direction = 'v';
                                        
                                        last_position = next_pos;
                                        // Get new neighbors after modification
                                        neighbors = Some(grid.get_neighbors(next_pos));
                                    },
                                    '#' => {
                                        if let Some((pos, cell)) = n.get("Left") {
                                            match cell {
                                                '.' | 'X' | '^' => {
                                                    // Store the values we need before modifying grid
                                                    let current_pos = last_position;
                                                    let next_pos = *pos;

                                                    // Now we can modify grid
                                                    grid.insert(current_pos, Direction::Left(next_pos, None));

                                                    direction = '<';
                                                    
                                                    last_position = next_pos;
                                                    // Get new neighbors after modification
                                                    neighbors = Some(grid.get_neighbors(next_pos));
                                                },
                                                '#' => {
                                                    if let Some((pos, cell)) = n.get("Up") {
                                                        match cell {
                                                            '.' | 'X' | '^' => {
                                                                // Store the values we need before modifying grid
                                                                let current_pos = last_position;
                                                                let next_pos = *pos;

                                                                // Now we can modify grid
                                                                grid.insert(current_pos, Direction::Up(next_pos, None));

                                                                direction = '^';
                                                                
                                                                last_position = next_pos;
                                                                // Get new neighbors after modification
                                                                neighbors = Some(grid.get_neighbors(next_pos));
                                                            },
                                                            _ => todo!()
                                                        }
                                                    }
                                                }
                                                _ => todo!()
                                            }
                                        }
                                    }
                                    _ => todo!()
                                }
                            }
                        }
                        '\n' => {
                            dbg!("we are leaving");
                            break;
                        }
                        unknown => {
                            dbg!(unknown);
                            panic!("invalid character")
                        }
                    }
                } else {
                    todo!("break?");
                }
            },
            'v' => {
                if let Some((pos, cell)) = n.get("Down") {
                    match cell {
                        '.' | 'X' | '^' => {
                            // Store the values we need before modifying grid
                            let current_pos = last_position;
                            let next_pos = *pos;

                            // Now we can modify grid
                            grid.insert(current_pos, Direction::Down(next_pos, None));

                            direction = 'v';

                            
                            last_position = next_pos;
                            // Get new neighbors after modification
                            neighbors = Some(grid.get_neighbors(next_pos));
                        }
                        // we hit a wall, turn right (relative)
                        '#' => {
                            if let Some((pos, cell)) = n.get("Left") {
                                match cell {
                                    '.' | 'X' | '^' => {
                                        // Store the values we need before modifying grid
                                        let current_pos = last_position;
                                        let next_pos = *pos;

                                        // Now we can modify grid
                                        grid.insert(current_pos, Direction::Left(next_pos, None));

                                        direction = '<';
                                        
                                        last_position = next_pos;
                                        // Get new neighbors after modification
                                        neighbors = Some(grid.get_neighbors(next_pos));
                                    },
                                    '#' => {
                                        if let Some((pos, cell)) = n.get("Up") {
                                            match cell {
                                                '.' | 'X' | '^' => {
                                                    // Store the values we need before modifying grid
                                                    let current_pos = last_position;
                                                    let next_pos = *pos;
                                                    
                                                    // Now we can modify grid
                                                    grid.insert(current_pos, Direction::Up(next_pos, None));

                                                    direction = '^';
                                                    
                                                    last_position = next_pos;
                                                    // Get new neighbors after modification
                                                    neighbors = Some(grid.get_neighbors(next_pos));
                                                },
                                                '#' => {
                                                    if let Some((pos, cell)) = n.get("Right") {
                                                        match cell {
                                                            '.' | 'X' | '^' => {
                                                                // Store the values we need before modifying grid
                                                                let current_pos = last_position;
                                                                let next_pos = *pos;

                                                                // Now we can modify grid
                                                                grid.insert(current_pos, Direction::Right(next_pos, None));

                                                                direction = '>';
                                                                
                                                                last_position = next_pos;
                                                                // Get new neighbors after modification
                                                                neighbors = Some(grid.get_neighbors(next_pos));
                                                            }
                                                            _ => todo!()
                                                        }
                                                    } else {
                                                        todo!()
                                                    }
                                                }
                                                _ => todo!()
                                            }
                                        }
                                    }
                                    _ => todo!()
                                }
                            }
                        }
                        _ => todo!()
                    }
                } else if let Some((pos, cell)) = n.get("Left") {
                    match cell {
                        '.' | 'X' | '^' => {
                            // Store the values we need before modifying grid
                            let current_pos = last_position;
                            let next_pos = *pos;

                            // Now we can modify grid
                            grid.insert(current_pos, Direction::Left(next_pos, None));

                            direction = '<';
                            
                            last_position = next_pos;
                            // Get new neighbors after modification
                            neighbors = Some(grid.get_neighbors(next_pos));
                        }
                        // we hit a wall, turn right (relative)
                        '#' => {
                            if let Some((pos, cell)) = n.get("Up") {
                                match cell {
                                    '.' | 'X' | '^' => {
                                        // Store the values we need before modifying grid
                                        let current_pos = last_position;
                                        let next_pos = *pos;
                                        
                                        // Now we can modify grid
                                        grid.insert(current_pos, Direction::Up(next_pos, None));

                                        direction = '^';
                                        
                                        last_position = next_pos;
                                        // Get new neighbors after modification
                                        neighbors = Some(grid.get_neighbors(next_pos));
                                    },
                                    '#' => {
                                        if let Some((pos, cell)) = n.get("Right") {
                                            match cell {
                                                '.' | 'X' | '^' => {
                                                    // Store the values we need before modifying grid
                                                    let current_pos = last_position;
                                                    let next_pos = *pos;
                                                    
                                                    // Now we can modify grid
                                                    grid.insert(current_pos, Direction::Right(next_pos, None));

                                                    direction = '>';
                                                    
                                                    last_position = next_pos;
                                                    // Get new neighbors after modification
                                                    neighbors = Some(grid.get_neighbors(next_pos));
                                                },
                                                '#' => {
                                                    if let Some((pos, cell)) = n.get("Down") {
                                                        match cell {
                                                            '.' | 'X' | '^' => {
                                                                // Store the values we need before modifying grid
                                                                let current_pos = last_position;
                                                                let next_pos = *pos;

                                                                // Now we can modify grid
                                                                grid.insert(current_pos, Direction::Down(next_pos, None));

                                                                direction = 'v';
                                                                
                                                                last_position = next_pos;
                                                                // Get new neighbors after modification
                                                                neighbors = Some(grid.get_neighbors(next_pos));
                                                            },
                                                            _ => todo!()
                                                        }
                                                    }
                                                },
                                                _ => todo!()
                                            }
                                        }
                                    },
                                    _ => todo!()
                                }
                            }
                        }
                        '\n' => {
                            dbg!("we are leaving");
                            break;
                        }
                        unknown => {
                            dbg!(unknown);
                            panic!("invalid character")
                        }
                    }
                } else {
                    todo!("break?");
                }
            },
            '<' => {
                // forward
                if let Some((pos, cell)) = n.get("Left") {
                    match cell {
                        '.' | 'X' | '^' => {
                            // Store the values we need before modifying grid
                            let current_pos = last_position;
                            let next_pos = *pos;

                            // Now we can modify grid
                            grid.insert(current_pos, Direction::Left(next_pos, None));

                            direction = '<';
                            
                            last_position = next_pos;
                            // Get new neighbors after modification
                            neighbors = Some(grid.get_neighbors(next_pos));
                        }
                        // we hit a wall, turn right (relative)
                        '#' => {
                            if let Some((pos, cell)) = n.get("Up") {
                                match cell {
                                    '.' | 'X' | '^' => {
                                        // Store the values we need before modifying grid
                                        let current_pos = last_position;
                                        let next_pos = *pos;
                                        
                                        // Now we can modify grid
                                        grid.insert(current_pos, Direction::Up(next_pos, None));

                                        direction = '^';
                                        
                                        last_position = next_pos;
                                        // Get new neighbors after modification
                                        neighbors = Some(grid.get_neighbors(next_pos));
                                    },
                                    '#' => {
                                        if let Some((pos, cell)) = n.get("Right") {
                                            match cell {
                                                '.' | 'X' | '^' => {
                                                    // Store the values we need before modifying grid
                                                    let current_pos = last_position;
                                                    let next_pos = *pos;
                                                    
                                                    // Now we can modify grid
                                                    grid.insert(current_pos, Direction::Right(next_pos, None));

                                                    direction = '>';
                                                    
                                                    last_position = next_pos;
                                                    // Get new neighbors after modification
                                                    neighbors = Some(grid.get_neighbors(next_pos));
                                                },
                                                '#' => {
                                                    if let Some((pos, cell)) = n.get("Down") {
                                                        match cell {
                                                            '.' | 'X' | '^' => {
                                                                // Store the values we need before modifying grid
                                                                let current_pos = last_position;
                                                                let next_pos = *pos;
                                                                
                                                                // Now we can modify grid
                                                                grid.insert(current_pos, Direction::Down(next_pos, None));

                                                                direction = 'v';
                                                                
                                                                last_position = next_pos;
                                                                // Get new neighbors after modification
                                                                neighbors = Some(grid.get_neighbors(next_pos));
                                                            },
                                                            _ => todo!()
                                                        }
                                                    }
                                                },
                                                _ => todo!()
                                            }
                                        }
                                    },
                                    _ => todo!()
                                }
                            }
                        }
                        '\n' => {
                            dbg!("we are leaving");
                            break;
                        }
                        unknown => {
                            dbg!(unknown);
                            panic!("invalid character")
                        }
                    }
                } else {
                    todo!("break?");
                }
            },
            unknown => {
                dbg!(unknown);
                panic!("invalid character")
            }
        }
    }

//     // dbg!(&grid.2);
//     // let _ = &grid.2.lines().inspect(|l| {dbg!(l);}).count();

    // grid.print();
    // let output = grid.2.chars().filter(|c| *c == 'X').count();

    dbg!(&grid.find_cycles());

    // dbg!(&grid.find_crossings());
    dbg!(&grid.find_stuff());

    return Ok(grid.find_stuff().to_string());

    // // let output = grid.3.len();
    // let output = grid.distinct_positions();

    // Ok((output + 1).to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    // use rstest::rstest;

//     #[rstest]
//     #[case("ABC
// ^..
// DEF
// GHI", "100")]
//     fn test_cases(#[case] input: &str, #[case] expected: &str) {
//         assert_eq!(process(input).unwrap(), expected);
//     }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
