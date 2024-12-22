use crate::AocError;


fn mult(a: usize) -> usize {
    a * 64
}

/// To mix a value into the secret number, calculate the bitwise XOR of 
/// the given value and the secret number. Then, the secret number becomes 
/// the result of that operation.
/// 
/// luckily, XOR is commutative, so the order of the operands doesn't matter.
fn mix(secret: usize, result: usize) -> usize {
    result ^ secret
}

/// To prune the secret number, calculate the value of the secret number modulo 16777216. 
/// Then, the secret number becomes the result of that operation.
fn prune(secret: usize) -> usize {
    secret % 16777216
}


// // secret * 64
// // secret / 32
// // secret * 2048
// fn secret(input: usize) -> usize {
//     // let a = input * 64;
//     let a = mult(input);
//     // let b = input ^ a;
//     let b = mix(input, a);
//     // let c = b / 
//     let c = prune(b);

//     dbg!(a, b, c);

//     c
// }

fn calculate_next_secret(mut secret: usize) -> (usize, usize) {
    // First operation
    let result = secret * 64;
    secret = mix(secret, result);
    secret = prune(secret);

    // Second operation
    let result = secret / 32;
    secret = mix(secret, result);
    secret = prune(secret);

    // Third operation
    let result = secret * 2048;
    secret = mix(secret, result);
    secret = prune(secret);

    // dbg!(secret, secret % 10);

    (secret, secret % 10)
}


fn repeat(input: usize, count: usize) -> Vec<(usize, usize)> {
    fn inner(input: usize, count: usize, mut acc: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        if count == 0 {
            return acc;
        }
        
        let next = calculate_next_secret(input);
        acc.push(next);
        inner(next.0, count - 1, acc)
    }

    inner(input, count, Vec::with_capacity(count))

    // if count == 0 {
    //     return input;
    // }

    // repeat(calculate_next_secret(input), count - 1)

    // // for _ in 0..count {
    // //     // let a = mult(secret);
    // //     // let b = mix(secret, a);
    // //     // let c = prune(b);

    // //     // secret = c;

    // //     secret(secret)
    // // }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output: usize = input.lines()
        // .flat_map(|line| {
        //     let number: usize = line.parse().unwrap();

        //     repeat(number, 10).iter().unzip().1
        // })
        .map(|line| {
            let number: usize = line.parse().unwrap();

            repeat(number, 10).iter()
                .map(|v| v.1)
                .collect::<Vec<usize>>()
                // differences
                .windows(2)
                .map(|window| {
                    // window[1] is current, window[0] is previous
                    window[1] as i32 - window[0] as i32
                }).collect::<Vec<i32>>()
        })
        // .inspect(|v| {
        //     dbg!(v);
        // })
        // .map(|v| v.0)
        .inspect(|v| {
            dbg!(v);
        })
        .count();
// panic!("at the disco");
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
    fn test_process() -> miette::Result<()> {
        let input = "1
10
100
2024";
        assert_eq!("37327623", process(input)?);
        Ok(())
    }
}
