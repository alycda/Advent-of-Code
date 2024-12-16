use nom::{
    bytes::complete::tag, character::complete::{i32 as parse_i32, space0}, sequence::{preceded, separated_pair, tuple}, IResult
};
use ornaments::{PhantomGrid, Position, Velocity, AocError};

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

#[derive(Debug, Clone)]
pub struct Robot {
    position: Position,
    velocity: Velocity,
}

pub struct Robots(Vec<Robot>);

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
}

#[tracing::instrument]
pub fn process(input: &str, dimensions: Position) -> miette::Result<String, AocError> {
    let mut robots = input.lines()
        .map(|line| {
            let (_, robot) = parse_robot(line).unwrap();

            // dbg!(robot)
            robot
        })
        .collect::<Vec<_>>();
    assert!(robots.len() > 0);

    // dbg!(robots);
    // println!("Initial state:");
    // println!("{}", visualize_robots(&robots, dimensions.x, dimensions.y));

    let mut grid = Day14(PhantomGrid::new(dimensions.x, dimensions.y));

    for i in 0..100 {
        grid.step(&mut robots);
        // println!("Next state:");
        // println!("{}", visualize_robots(&robots, dimensions.x, dimensions.y));
    }

    // println!("Final state:");
    // println!("{}", visualize_robots(&robots, dimensions.x, dimensions.y));

    let q0 = grid.get_quadrant_robots(&robots, 0).len(); // Top Left
    let q1 = grid.get_quadrant_robots(&robots, 1).len(); // Top Right  
    let q2 = grid.get_quadrant_robots(&robots, 2).len(); // Bottom Left
    let q3 = grid.get_quadrant_robots(&robots, 3).len(); // Bottom Right

    // The safety factor is the product of all quadrants
    let safety_factor = q0 * q1 * q2 * q3;

    // println!("Quadrant counts: {} {} {} {}", q0, q1, q2, q3);
    // println!("Safety factor: {}", safety_factor);

    Ok(safety_factor.to_string())
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

    // #[test]
    // fn test_process() -> miette::Result<()> {
    //     let input = "p=2,4 v=2,-3";
    //     let grid_size = IVec2::new(11, 7);

    //     assert_eq!("12", process(input, grid_size)?);
    //     Ok(())
    // }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let grid_size = Position::new(11, 7);

        assert_eq!("12", process(input, grid_size)?);
        Ok(())
    }
}

// a method on PhantomGrid?
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

impl std::fmt::Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "p={},{} v={},{}", 
            self.position.x, self.position.y,
            self.velocity.x, self.velocity.y)
    }
}

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