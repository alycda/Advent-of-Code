use std::collections::BTreeMap;

use crate::AocError;

use nom::{
    branch::alt, bytes::complete::{tag, take_while1}, character::complete::{space0, space1}, combinator::map, multi::separated_list1, sequence::tuple, IResult
};

#[derive(Debug, PartialEq)]
pub enum Operation {
    And,
    Xor,
    Or,
}

#[derive(Debug)]
pub struct LogicGate {
    // input: String,
    left: String,
    right: String,
    op: Operation,
    output: String,
}

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (initial_values, connections) = input.split_once("\n\n").unwrap();

    let mut wires = initial_values.lines().fold(BTreeMap::new(), |mut map, next| {
        let (key, value) = next.split_once(":").unwrap();
        let v = value.trim().parse::<u8>().unwrap();

        map.insert(key.to_string(), v == 1);

        map
    });

    let binding = parse_logic_gates(connections).unwrap();

    let mut pending_gates = binding.1;
    
    // Keep evaluating as long as we make progress
    let mut made_progress = true;
    while made_progress && !pending_gates.is_empty() {
        made_progress = false;
        
        let (ready, still_pending): (Vec<_>, Vec<_>) = pending_gates.into_iter()
            .partition(|gate| {
                wires.contains_key(&gate.left) && 
                wires.contains_key(&gate.right)
            });

        pending_gates = still_pending;

        for gate in ready {
            let left = wires[&gate.left];
            let right = wires[&gate.right];
            
            let result = match gate.op {
                Operation::Or => left | right,
                Operation::And => left & right,
                Operation::Xor => left ^ right,
            };
            
            wires.insert(gate.output, result);
            made_progress = true;
        }
    }

    let mut wire_nums: Vec<(usize, bool)> = wires.iter()
        .filter(|(k, _)| k.starts_with("z"))
        .map(|(k, v)| {
            let num = k.trim_start_matches('z').parse::<usize>().unwrap();
            dbg!(k, num, v);
            (num, *v)
        })
        .collect();

    wire_nums.sort_by_key(|(num, _)| *num);
    // dbg!(&wire_nums); // See sorted order

    // let bits: Vec<u32> = wire_nums.iter()
    //     .rev()
    //     .map(|(_, v)| if *v { 1 } else { 0 })
    //     .collect();

    // dbg!(&bits);

    // dbg!(&bits.clone().iter().collect::<String>()); // complete binary number

    // let binary_string: String = bits.iter()
    //     .map(|&bit| char::from_digit(bit as u32, 10).unwrap())
    //     .collect();
    // dbg!(&binary_string);

    // // convert to decimal
    // let output = bits.into_iter()
    //     .fold(0u32, |acc, bit| {
    //         (acc << 1) | bit
    //     });

    // dbg!(&wires);

    // dbg!(output);

    // Use u128 for the conversion
    let output = wire_nums.iter()
        .rev()
        .map(|(_, v)| if *v { 1u128 } else { 0u128 })
        .fold(0u128, |acc, bit| {
            (acc << 1) | bit
        });

    Ok(output.to_string())
}

// Parse identifiers like x00, y01, z02
fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric())(input)
}

// Parse the operation
fn operation(input: &str) -> IResult<&str, Operation> {
    map(
        alt((tag("AND"), tag("XOR"), tag("OR"))),
        |op: &str| match op {
            "AND" => Operation::And,
            "XOR" => Operation::Xor,
            "OR" => Operation::Or,
            _ => unreachable!(),
        },
    )(input)
}

// Parse a single line
fn logic_line(input: &str) -> IResult<&str, LogicGate> {
    map(
        tuple((
            identifier,
            space1,
            operation,
            space1,
            identifier,
            space0,
            tag("->"),
            space0,
            identifier,
        )),
        |(input1, _, op, _, input2, _, _, _, output)| LogicGate {
            // input: format!("{} {:?} {}", input1, op, input2),
            left: input1.to_string(),
            right: input2.to_string(),
            op,
            output: output.to_string(),
        },
    )(input)
}

// Parse multiple lines
fn parse_logic_gates(input: &str) -> IResult<&str, Vec<LogicGate>> {
    separated_list1(
        tag("\n"),
        logic_line,
    )(input)
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
        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        assert_eq!("4", process(input)?); // 100
        Ok(())
    }

    #[test]
    fn test_process_large() -> miette::Result<()> {
        let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        assert_eq!("2024", process(input)?); // 0011111101000
        Ok(())
    }
}
