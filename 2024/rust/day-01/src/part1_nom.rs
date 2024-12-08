use miette::miette;
use nom::IResult;

fn parse(input: &str) -> IResult<&str, (Vec<i32>, Vec<i32>)> {
    todo!()
}

pub fn process(input: &str) -> miette::Result<String> {
    let (_, (mut left, mut right)) = parse(input).map_err(|e| miette!("parse failed {}", e))?;

    left.sort();
    right.sort();

    let output: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}