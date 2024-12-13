use nom::{
    bytes::complete::tag, character::complete::{alpha1, char, digit1}, combinator::{map, map_res, opt}, sequence::{preceded, tuple}, IResult
};
use glam::IVec2;

use crate::AocError;

// A = 3 tokens
// B = 1 token

#[derive(Debug)]
struct Button(IVec2);

impl std::ops::Deref for Button {
    type Target = IVec2;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(
        tuple((
            opt(char('+')),
            digit1
        )),
        |(_sign, num): (Option<char>, &str)| num.parse::<i32>()
    )(input)
}

fn parse_coordinates(input: &str) -> IResult<&str, IVec2> {
    map(
        tuple((
            preceded(tag("X"), parse_number),
            preceded(tag(", Y"), parse_number)
        )),
        |(x, y)| IVec2::new(x, y)
    )(input)
}

fn parse_button(input: &str) -> IResult<&str, IVec2> {
    preceded(
        tuple((
            tag("Button "),
            alpha1,
            tag(": ")
        )),
        parse_coordinates
    )(input)
}

fn parse_prize_coordinates(input: &str) -> IResult<&str, IVec2> {
    map(
        tuple((
            preceded(tag("X="), parse_number),
            preceded(tag(", Y="), parse_number)
        )),
        |(x, y)| IVec2::new(x, y)
    )(input)
}

fn parse_prize(input: &str) -> IResult<&str, IVec2> {
    preceded(
        tag("Prize: "),
        parse_prize_coordinates
    )(input)
}

fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        return (a, 1, 0);
    }
    
    let (gcd, x1, y1) = extended_gcd(b, a % b);
    let x = y1;
    let y = x1 - (a / b) * y1;
    
    (gcd, x, y)
}

fn find_button_presses(button_a: &Button, button_b: &Button, target: IVec2) -> Option<(i32, i32)> {
    // Find base solutions for x coordinate
    let (gcd_x, x0, y0) = extended_gcd(button_a.x, button_b.x);
    if target.x % gcd_x != 0 {
        return None;
    }
    
    // Find base solutions for y coordinate
    let (gcd_y, x1, y1) = extended_gcd(button_a.y, button_b.y);
    if target.y % gcd_y != 0 {
        return None;
    }
    
    // Compute base solutions scaled by target
    let t_x = target.x / gcd_x;
    let t_y = target.y / gcd_y;
    let base_x = (x0 * t_x, y0 * t_x);
    let base_y = (x1 * t_y, y1 * t_y);
    
    // Generate possible k values for each coordinate
    // For x: base_x.0 + k * (button_b.x/gcd_x) >= 0 and base_x.1 - k * (button_a.x/gcd_x) >= 0
    // For y: base_y.0 + k * (button_b.y/gcd_y) >= 0 and base_y.1 - k * (button_a.y/gcd_y) >= 0
    
    // Find k values that work for both x and y
    let mut min_cost = i32::MAX;
    let mut best_solution = None;
    
    // Try reasonable k values (-1000 to 1000 should cover our problem space)
    for k in -50000..=50000 {
        let ax = base_x.0 + k * (button_b.x/gcd_x);
        let bx = base_x.1 - k * (button_a.x/gcd_x);
        let ay = base_y.0 + k * (button_b.y/gcd_y);
        let by = base_y.1 - k * (button_a.y/gcd_y);
        
        // Check if this solution is valid for both coordinates
        if ax >= 0 && bx >= 0 && ay >= 0 && by >= 0 {
            let cost = 80 * ax + 40 * bx;
            if cost < min_cost {
                min_cost = cost;
                best_solution = dbg!(Some((ax, bx)));
            }
        }
    }
    
    dbg!(best_solution)
}

fn solve_button_presses(button_a: Button, button_b: Button, target: IVec2) -> Option<(i32, i32)> {
    let denominator = button_a.x * button_b.y - button_a.y * button_b.x;
    
    // Using Cramer's rule
    let a = (button_b.y * target.x - button_b.x * target.y) / denominator;
    let b = (-button_a.y * target.x + button_a.x * target.y) / denominator;
    
    // Check if we got integer solutions
    if a * denominator == (button_b.y * target.x - button_b.x * target.y) && 
       b * denominator == (-button_a.y * target.x + button_a.x * target.y) {
        Some((a, b))
    } else {
        None
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output: i32 = input.split("\n\n")
        .map(|mini_game| {
            // dbg!(mini_game);

            let mut game = mini_game.lines();

            // 3 tokens
            let (_, a) = parse_button(game.next().unwrap()).unwrap();
            let a = Button(a);

            // 1 token
            let (_, b) = parse_button(game.next().unwrap()).unwrap();
            let b = Button(b);

            // prize
            let (_, prize) = parse_prize(game.next().unwrap()).unwrap();

            (a, b, prize)
        })
        .filter_map(|(button_a, button_b, prize_location)| {
            // let (gcd_x, x0, y0) = extended_gcd(button_a.x, button_b.x);
            // let (gcd_y, x1, y1) = extended_gcd(button_a.y, button_b.y);

            // dbg!(gcd_x, x0, y0);
            // dbg!(gcd_y, x1, y1);

            // find_button_presses(button_a, button_b, *prize_location);
            dbg!(solve_button_presses(button_a, button_b, prize_location))
        }).map(|(a, b)| {
            (a * 3) + b
        }).sum();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_button_solver() {
    //     let button_a = Button(IVec2::new(94, 34));
    //     let button_b = Button(IVec2::new(22, 67));
    //     let target = IVec2::new(8400, 5400);
        
    //     let (a, b) = find_button_presses(&button_a, &button_b, target).unwrap();
    //     assert_eq!(*button_a * a + *button_b * b, target);
    // }

    // use rstest::rstest;

    // #[rstest]
    // #[case("Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400", "280")]
    // #[case("Button A: X+26, Y+66
// Button B: X+67, Y+21
// Prize: X=12748, Y=12176", "0")]
    // #[case("Button A: X+17, Y+86
// Button B: X+84, Y+37
// Prize: X=7870, Y=6450", "200")]
    // #[case("Button A: X+69, Y+23
// Button B: X+27, Y+71
// Prize: X=18641, Y=10279", "0")]
    // #[case("", "")]
    // #[case("", "")]
    // fn test_cases(#[case] input: &str, #[case] expected: &str) {
    //     assert_eq!(process(input).unwrap(), expected);
    // }

//     #[test]
//     fn test_process() -> miette::Result<()> {
//         let input = "Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400";
//         assert_eq!("280", process(input)?);
//         Ok(())
//     }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
