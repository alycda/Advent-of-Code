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

fn find_gate<'a>(gates: &'a [LogicGate], a: &str, b: &str, op: &Operation) -> Option<&'a String> {
    let result = gates.iter()
        .find(|gate| {
            let match_found = (gate.left == a && gate.right == b && gate.op == *op) ||
                (gate.left == b && gate.right == a && gate.op == *op);
            if match_found {
                println!("Found gate: {} {} {:?} {} -> {}", 
                    gate.left, gate.right, gate.op, a, gate.output);
            }
            match_found
        })
        .map(|gate| &gate.output);
    
    if result.is_none() {
        println!("No gate found for {} {:?} {}", a, op, b);
    }
    
    result
}

#[derive(Debug, Clone)]
struct FullAdder {
    sum: String,    // z wire output
    carry: String,  // carry output
    temps: Vec<String>, // intermediate wires
}

fn analyze_adder(gates: &[LogicGate]) -> Vec<String> {
    let mut incorrect_wires = Vec::new();
    let mut prev_carry = None;
    
    for i in 0..45 {
        let n = format!("{:02}", i);
        let stage = identify_adder_stage(gates, &n, prev_carry.as_deref());
        
        // For first stage (half adder)
        if i == 0 {
            if stage.sum != "z00" {
                incorrect_wires.push(stage.sum);
            }
            prev_carry = Some(stage.carry);
            continue;
        }
        
        // Check XOR gates in full adder stages
        if let Some(current_z) = find_z_wire(gates, &n) {
            if !is_valid_z_output(gates, &current_z) {
                incorrect_wires.push(current_z);
            }
        }

        prev_carry = Some(stage.carry);
    }
    
    incorrect_wires.sort();
    incorrect_wires.dedup();
    incorrect_wires
}

fn identify_adder_stage(gates: &[LogicGate], n: &str, prev_carry: Option<&str>) -> FullAdder {
    let x = format!("x{}", n);
    let y = format!("y{}", n);
    
    // Find initial XOR and AND between x and y
    let xor_out = find_gate(gates, &x, &y, &Operation::Xor)
        .map(|s| s.to_string())
        .unwrap_or_default();
        
    let and_out = find_gate(gates, &x, &y, &Operation::And)
        .map(|s| s.to_string())
        .unwrap_or_default();

    // For full adder, find carry chain
    let (sum, carry) = if let Some(c) = prev_carry {
        let carry_and = find_gate(gates, &xor_out, c, &Operation::And)
            .map(|s| s.to_string())
            .unwrap_or_default();
            
        let sum = find_gate(gates, &xor_out, c, &Operation::Xor)
            .map(|s| s.to_string())
            .unwrap_or_default();
            
        let carry = find_gate(gates, &carry_and, &and_out, &Operation::Or)
            .map(|s| s.to_string())
            .unwrap_or_default();
            
        (sum, carry)
    } else {
        (xor_out.clone(), and_out.clone())
    };

    FullAdder {
        sum,
        carry,
        temps: vec![xor_out, and_out],
    }
}

fn is_valid_z_output(gates: &[LogicGate], wire: &str) -> bool {
    if !wire.starts_with('z') {
        return true; // Not a z-wire, so no validation needed
    }
    
    // Find the gate that produces this z-wire
    if let Some(gate) = gates.iter().find(|g| g.output == wire) {
        // All z-wires except z45 should come from XOR gates
        if wire == "z45" {
            gate.op == Operation::Or
        } else {
            gate.op == Operation::Xor
        }
    } else {
        false
    }
}

fn find_z_wire(gates: &[LogicGate], n: &str) -> Option<String> {
    gates.iter()
        .find(|g| g.output == format!("z{}", n))
        .map(|g| g.output.clone())
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (initial_values, gate_defs) = input.split_once("\n\n").unwrap();
    let gates = parse_logic_gates(gate_defs).unwrap().1;
    
    let incorrect_wires = analyze_adder(&gates);
    Ok(incorrect_wires.join(","))
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
