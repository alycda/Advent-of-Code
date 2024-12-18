use nom::{
    bytes::complete::tag, character::complete::{i32 as parse_i32, space0}, sequence::{preceded, separated_pair, tuple}, IResult
};

// use miette::Diagnostic;
use ornaments::{PhantomGrid, Position, Velocity};
// use thiserror::Error;

pub use crate::Day14 as Day;

pub mod part1;
pub mod part2;

pub struct Day14(PhantomGrid);

impl std::ops::Deref for Day14 {
    type Target = PhantomGrid;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Day14 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Day14 {
    // Simulates one step for all robots
    pub fn step(&mut self, robots: &mut Vec<Robot>) {
        // Update all positions
        for robot in robots.iter_mut() {
            let new_pos = self.wrap_position(robot.position + robot.velocity);
            robot.position = new_pos;
        }

        // Update grid state
        self.clear();
        for robot in robots.iter() {
            self.insert(robot.position);
        }
    }

    pub fn wrap_position(&self, pos: Position) -> Position {
        // Adding width/height + 1 to handle negative positions
        let width = self.1.1.x + 1;
        let height = self.1.1.y + 1;
        
        Position::new(
            (pos.x % width + width) % width,
            (pos.y % height + height) % height
        )
    }

    fn get_quadrant_robots<'a>(&self, robots: &'a [Robot], quadrant: usize) -> Vec<&'a Robot> {
        let mid_x = self.1.1.x / 2;
        let mid_y = self.1.1.y / 2;
        
        robots.iter()
            .filter(|robot| {
                let pos = robot.position;
                // Skip middle lines
                if pos.x == mid_x || pos.y == mid_y {
                    return false;
                }
                
                match quadrant {
                    0 => pos.x < mid_x && pos.y < mid_y,    // Top Left
                    1 => pos.x > mid_x && pos.y < mid_y,    // Top Right
                    2 => pos.x < mid_x && pos.y > mid_y,    // Bottom Left
                    3 => pos.x > mid_x && pos.y > mid_y,    // Bottom Right
                    _ => false
                }
            })
            .collect()
    }

    fn visualize_robots(robots: &[Robot], width: i32, height: i32) -> String {
        let mut output = String::new();
        for y in 0..height {
            for x in 0..width {
                let count = robots.iter()
                    .filter(|r| r.position.x == x && r.position.y == y)
                    .count();
                
                output.push(if count == 0 {
                    '.'
                } else if count < 10 {
                    // Convert count to char ('1' through '9')
                    (b'0' + count as u8) as char
                } else {
                    '#' // For counts >= 10
                });
            }
            output.push('\n');
        }
        output
    }
}


#[derive(Debug, Clone)]
pub struct Robot {
    position: Position,
    velocity: Velocity,
}

// impl std::fmt::Display for Robot {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "p={},{} v={},{}", 
//             self.position.x, self.position.y,
//             self.velocity.x, self.velocity.y)
//     }
// }

pub struct Robots(Vec<Robot>);

impl std::ops::Deref for Robots {
    type Target = Vec<Robot>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Robots {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn parse_position(input: &str) -> IResult<&str, Position> {
    let (input, coords) = preceded(
        tag("p="),
        separated_pair(parse_i32, tag(","), parse_i32)
    )(input)?;
    
    Ok((input, Position::new(coords.0, coords.1)))
}

fn parse_velocity(input: &str) -> IResult<&str, Velocity> {
    let (input, coords) = preceded(
        tag("v="),
        separated_pair(parse_i32, tag(","), parse_i32)
    )(input)?;
    
    Ok((input, Velocity::new(coords.0, coords.1)))
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (input, (position, _, velocity)) = tuple((
        parse_position,
        space0,  // This handles the space between position and velocity
        parse_velocity
    ))(input)?;

    Ok((input, Robot { position, velocity }))
}
