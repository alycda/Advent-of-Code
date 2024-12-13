use glam::IVec2;

use crate::{parse_button, parse_prize, AocError, Button};

fn solve_button_presses(button_a: Button, button_b: Button, target: IVec2, offset: i64) -> Option<(i64, i64)> {
    // Convert to i64 for large number calculations
    let a_x = button_a.x as i64;
    let a_y = button_a.y as i64;
    let b_x = button_b.x as i64;
    let b_y = button_b.y as i64;
    let t_x = target.x as i64 + offset;
    let t_y = target.y as i64 + offset;
    
    let denominator = a_x * b_y - a_y * b_x;
    
    // Using Cramer's rule with i64
    let a = (b_y * t_x - b_x * t_y) / denominator;
    let b = (-a_y * t_x + a_x * t_y) / denominator;
    
    // Check if we got integer solutions
    if a * denominator == (b_y * t_x - b_x * t_y) && 
       b * denominator == (-a_y * t_x + a_x * t_y) {
        Some((a, b))
    } else {
        None
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output: i64 = input.split("\n\n")
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
            solve_button_presses(button_a, button_b, prize_location, 10_000_000_000_000)
        }).map(|(a, b)| {
            (a * 3) + b
        }).sum();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!("875318608908", process(input)?);
        Ok(())
    }
}
