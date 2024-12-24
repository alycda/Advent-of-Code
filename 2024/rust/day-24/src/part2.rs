use std::collections::BTreeMap;

use crate::AocError;

use nom::{
    branch::alt, bytes::complete::{tag, take_while1}, character::complete::{space0, space1}, combinator::map, multi::separated_list1, sequence::tuple, IResult
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operation {
    And,
    Xor,
    Or,
}

#[derive(Debug, Clone)]
pub struct LogicGate {
    // input: String,
    left: String,
    right: String,
    op: Operation,
    output: String,
}

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (initial_values, gate_defs) = input.split_once("\n\n").unwrap();

    let wires = initial_values.lines().fold(BTreeMap::new(), |mut map, next| {
        let (key, value) = next.split_once(":").unwrap();
        let v = value.trim().parse::<u8>().unwrap();
        map.insert(key.to_string(), v == 1);
        map
    });

    let gates = parse_logic_gates(gate_defs).unwrap().1;
    
    // Get all possible pairs for swapping
    let mut outputs: Vec<String> = gates.iter()
        .map(|gate| gate.output.clone())
        .collect();
    outputs.sort(); // Ensure consistent ordering

    let mut swap_candidates = Vec::new();
    for i in 0..outputs.len() {
        for j in (i + 1)..outputs.len() {
            swap_candidates.push(SwapPair {
                wire1: outputs[i].clone(),
                wire2: outputs[j].clone(),
            });
        }
    }

    // Try combinations of 4 pairs
    // needs optimization
    for i in 0..swap_candidates.len() {
        for j in (i + 1)..swap_candidates.len() {
            for k in (j + 1)..swap_candidates.len() {
                for l in (k + 1)..swap_candidates.len() {
                    let swaps = vec![
                        swap_candidates[i].clone(),
                        swap_candidates[j].clone(),
                        swap_candidates[k].clone(),
                        swap_candidates[l].clone(),
                    ];
                    
                    if test_addition_with_swaps(&gates, &wires, &swaps) {
                        // Found correct swaps - return sorted wire list
                        let mut wire_list = swaps.iter()
                            .flat_map(|swap| vec![swap.wire1.clone(), swap.wire2.clone()])
                            .collect::<Vec<_>>();
                        wire_list.sort();
                        return Ok(wire_list.join(","));
                    }
                }
            }
        }
    }

    todo!()
}

// Represent a swap pair
#[derive(Debug, Clone)]
struct SwapPair {
    wire1: String,
    wire2: String,
}

fn test_addition_with_swaps(
    original_gates: &[LogicGate],
    initial_values: &BTreeMap<String, bool>,
    swaps: &[SwapPair],
) -> bool {
    // Clone gates and apply swaps
    let mut modified_gates = original_gates.to_vec();
    for swap in swaps {
        // Apply each swap to the gates' outputs
        for gate in &mut modified_gates {
            if gate.output == swap.wire1 {
                gate.output = swap.wire2.clone();
            } else if gate.output == swap.wire2 {
                gate.output = swap.wire1.clone();
            }
        }
    }

    // Get x and y input values
    let x_bits = initial_values.iter()
        .filter(|(k, _)| k.starts_with("x"))
        .map(|(k, &v)| (k.trim_start_matches('x').parse::<usize>().unwrap(), v))
        .collect::<BTreeMap<_, _>>();

    let y_bits = initial_values.iter()
        .filter(|(k, _)| k.starts_with("y"))
        .map(|(k, &v)| (k.trim_start_matches('y').parse::<usize>().unwrap(), v))
        .collect::<BTreeMap<_, _>>();

    // Calculate expected sum
    let x_val = x_bits.iter()
        .fold(0u128, |acc, (_, &bit)| (acc << 1) | (bit as u128));
    let y_val = y_bits.iter()
        .fold(0u128, |acc, (_, &bit)| (acc << 1) | (bit as u128));
    let expected_sum = x_val + y_val;

    // Evaluate gates with swaps and compare to expected sum
    let mut wires = initial_values.clone();
    let mut made_progress = true;
    while made_progress {
        made_progress = false;
        for gate in &modified_gates {
            if !wires.contains_key(&gate.output) && 
               wires.contains_key(&gate.left) && 
               wires.contains_key(&gate.right) {
                let result = match gate.op {
                    Operation::And => wires[&gate.left] & wires[&gate.right],
                    Operation::Or => wires[&gate.left] | wires[&gate.right],
                    Operation::Xor => wires[&gate.left] != wires[&gate.right],
                };
                wires.insert(gate.output.clone(), result);
                made_progress = true;
            }
        }
    }

    // Calculate actual sum from z wires
    let actual_sum = wires.iter()
        .filter(|(k, _)| k.starts_with("z"))
        .map(|(k, &v)| (k.trim_start_matches('z').parse::<usize>().unwrap(), v))
        .collect::<BTreeMap<_, _>>()
        .iter()
        .fold(0u128, |acc, (_, &bit)| (acc << 1) | (bit as u128));

    actual_sum == expected_sum
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
    fn test_process() -> miette::Result<()> {
        let input = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";
        assert_eq!("z00,z01,z02,z05", process(input)?);
        Ok(())
    }
}
