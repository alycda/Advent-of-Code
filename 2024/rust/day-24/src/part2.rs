use std::collections::BTreeMap;

use crate::AocError;

use nom::{
    branch::alt, bytes::complete::{tag, take_while1}, character::complete::{space0, space1}, combinator::map, multi::separated_list1, sequence::tuple, IResult
};

use petgraph::{
    graph::{DiGraph, NodeIndex},
    dot::{Dot, Config},
};
use std::collections::HashMap;

fn visualize_circuit(gates: &[LogicGate], wires: &BTreeMap<String, bool>) -> std::io::Result<()> {
    let mut graph = DiGraph::new();
    let mut node_map = HashMap::new();

    // Add wire nodes
    for (wire, &value) in wires.iter() {
        let node = graph.add_node(format!("{}={}", wire, value));
        node_map.insert(wire.clone(), node);
    }

    // Add gate nodes and edges
    for (i, gate) in gates.iter().enumerate() {
        let gate_node = graph.add_node(format!("Gate{} {:?}", i, gate.op));
        
        // Add edges from inputs to gate
        if let Some(&left) = node_map.get(&gate.left) {
            graph.add_edge(left, gate_node, "");
        }
        if let Some(&right) = node_map.get(&gate.right) {
            graph.add_edge(right, gate_node, "");
        }

        // Add edge from gate to output
        let output_node = if let Some(&node) = node_map.get(&gate.output) {
            node
        } else {
            let node = graph.add_node(format!("{}", gate.output));
            node_map.insert(gate.output.clone(), node);
            node
        };
        graph.add_edge(gate_node, output_node, "");
    }

    // Export to dot format
    let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    std::fs::write("circuit.dot", format!("{:?}", dot))?;
    
    println!("Generated circuit.dot - view with 'dot -Tpng circuit.dot -o circuit.png'");
    Ok(())
}

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

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (initial_values, gate_defs) = input.split_once("\n\n").unwrap();
    let gates = parse_logic_gates(gate_defs).unwrap().1;
    
    // Count how many x inputs we have (could also check y)
    let num_bits = initial_values.lines()
        .filter(|line| line.starts_with("x"))
        .count();

    println!("Found {} bits to process", num_bits);

    let mut swapped = Vec::new();
    let mut c0: Option<String> = None;
    
    for i in 0..num_bits {
        let n = format!("{:02}", i);

        println!("Processing bit {}", n);

        let mut m1 = find_gate(&gates, &format!("x{}", n), &format!("y{}", n), &Operation::Xor)
            .map(|s| s.to_string());
        let mut n1 = find_gate(&gates, &format!("x{}", n), &format!("y{}", n), &Operation::And)
            .map(|s| s.to_string());
        
        println!("Half adder gates found: m1={:?}, n1={:?}", m1, n1);

        if let Some(c0_val) = &c0 {
            if let (Some(m1_val), Some(n1_val)) = (&m1, &n1) {
                let mut r1 = find_gate(&gates, c0_val, m1_val, &Operation::And)
                    .map(|s| s.to_string());
                    
                if r1.is_none() {
                    // Clone values before swap
                    let m1_clone = m1.clone();
                    let n1_clone = n1.clone();
                    std::mem::swap(&mut m1, &mut n1);
                    
                    // Push the swapped values
                    if let (Some(m), Some(n)) = (m1_clone, n1_clone) {
                        swapped.push(m);
                        swapped.push(n);
                    }
                    
                    if let Some(m1_val) = &m1 {
                        r1 = find_gate(&gates, c0_val, m1_val, &Operation::And)
                            .map(|s| s.to_string());
                    }
                }
                
                let mut z1 = find_gate(&gates, c0_val, m1.as_ref().unwrap(), &Operation::Xor)
                    .map(|s| s.to_string());
                
                // Handle m1/z1 swap
                {
                    let needs_swap = m1.as_ref()
                        .map(|m| m.starts_with('z'))
                        .unwrap_or(false);
                    
                    if needs_swap {
                        let m1_clone = m1.clone();
                        std::mem::swap(&mut m1, &mut z1);
                        if let Some(m) = m1_clone {
                            swapped.push(m);
                            if let Some(z) = &z1 {
                                swapped.push(z.clone());
                            }
                        }
                    }
                }
                
                // Handle n1/z1 swap
                {
                    let needs_swap = n1.as_ref()
                        .map(|n| n.starts_with('z'))
                        .unwrap_or(false);
                    
                    if needs_swap {
                        let n1_clone = n1.clone();
                        std::mem::swap(&mut n1, &mut z1);
                        if let Some(n) = n1_clone {
                            swapped.push(n);
                            if let Some(z) = &z1 {
                                swapped.push(z.clone());
                            }
                        }
                    }
                }
                
                // Handle r1/z1 swap
                {
                    let needs_swap = r1.as_ref()
                        .map(|r| r.starts_with('z'))
                        .unwrap_or(false);
                    
                    if needs_swap {
                        let r1_clone = r1.clone();
                        std::mem::swap(&mut r1, &mut z1);
                        if let Some(r) = r1_clone {
                            swapped.push(r);
                            if let Some(z) = &z1 {
                                swapped.push(z.clone());
                            }
                        }
                    }
                }
                
                if let (Some(r1_val), Some(n1_val)) = (&r1, &n1) {
                    let mut c1 = find_gate(&gates, r1_val, n1_val, &Operation::Or)
                        .map(|s| s.to_string());
                    
                    // Handle carry bit swap
                    let needs_swap = c1.as_ref()
                        .map(|c| c.starts_with('z') && c != "z45")
                        .unwrap_or(false);
                    
                    if needs_swap {
                        let c1_clone = c1.clone();
                        std::mem::swap(&mut c1, &mut z1);
                        if let Some(c) = c1_clone {
                            swapped.push(c);
                            if let Some(z) = &z1 {
                                swapped.push(z.clone());
                            }
                        }
                    }
                    
                    c0 = c1;
                }
            }
        } else {
            c0 = n1;
        }
    }

    dbg!(&swapped);
    
    swapped.sort();
    swapped.dedup();
    Ok(swapped.join(","))
}

// #[tracing::instrument]
pub fn _process(input: &str) -> miette::Result<String, AocError> {
    let (initial_values, gate_defs) = input.split_once("\n\n").unwrap();

    // Still need initial wires map for setup
    let wires = initial_values.lines().fold(BTreeMap::new(), |mut map, next| {
        let (key, value) = next.split_once(":").unwrap();
        let v = value.trim().parse::<u8>().unwrap();
        map.insert(key.to_string(), v == 1);
        map
    });

    // Convert to our optimized circuit state
    let _initial_state = CircuitState::from_wires(&wires);
    let gates = parse_logic_gates(gate_defs).unwrap().1;
    
    visualize_circuit(&gates, &wires).unwrap();

    panic!("halt");

    // Get candidate wires for swapping
    let mut outputs: Vec<String> = gates.iter()
        .map(|gate| gate.output.clone())
        .collect();
    outputs.sort();

    let mut swap_candidates = Vec::new();
    for i in 0..outputs.len() {
        for j in (i + 1)..outputs.len() {
            swap_candidates.push(SwapPair {
                wire1: outputs[i].clone(),
                wire2: outputs[j].clone(),
            });
        }
    }

    // Try combinations of 4 pairs using bit vector operations
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

#[derive(Debug, Clone)]
struct CircuitState {
    x_bits: u128,
    y_bits: u128,
    z_bits: u128,
}

impl CircuitState {
    fn from_wires(wires: &BTreeMap<String, bool>) -> Self {
        let x_bits = wires.iter()
            .filter(|(k, _)| k.starts_with("x"))
            .map(|(k, &v)| {
                let pos = k.trim_start_matches('x').parse::<u32>().unwrap();
                if v { 1u128 << pos } else { 0 }
            })
            .sum();

        let y_bits = wires.iter()
            .filter(|(k, _)| k.starts_with("y"))
            .map(|(k, &v)| {
                let pos = k.trim_start_matches('y').parse::<u32>().unwrap();
                if v { 1u128 << pos } else { 0 }
            })
            .sum();

        CircuitState {
            x_bits,
            y_bits,
            z_bits: 0,
        }
    }

    fn evaluate_gate(&mut self, gate: &LogicGate) -> bool {
        // Get bit positions
        let left_pos = match gate.left.chars().next().unwrap() {
            'x' => {
                let pos = gate.left.trim_start_matches('x').parse::<u32>().unwrap();
                (self.x_bits >> pos) & 1
            },
            'y' => {
                let pos = gate.left.trim_start_matches('y').parse::<u32>().unwrap();
                (self.y_bits >> pos) & 1
            },
            _ => return false,
        };

        let right_pos = match gate.right.chars().next().unwrap() {
            'x' => {
                let pos = gate.right.trim_start_matches('x').parse::<u32>().unwrap();
                (self.x_bits >> pos) & 1
            },
            'y' => {
                let pos = gate.right.trim_start_matches('y').parse::<u32>().unwrap();
                (self.y_bits >> pos) & 1
            },
            _ => return false,
        };

        let result = match gate.op {
            Operation::And => left_pos & right_pos,
            Operation::Or => left_pos | right_pos,
            Operation::Xor => left_pos ^ right_pos,
        };

        let z_pos = gate.output.trim_start_matches('z').parse::<u32>().unwrap();
        self.z_bits |= result << z_pos;
        true
    }
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
