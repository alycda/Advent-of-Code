//! Day 24: Crossed Wires

use std::collections::BTreeMap;

use nom::{branch::alt, bytes::complete::{tag, take_while1}, character::complete::{space0, space1}, combinator::map, multi::separated_list1, sequence::tuple, IResult};
use ornaments::{AocError, Solution};

pub use crate::Day24 as Day;

#[derive(Debug, Clone)]
struct Gate {
    left: String,
    right: String,
    operation: Operation,
    out: String,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operation {
    And,
    Xor,
    Or,
}

#[derive(Debug)]
pub struct Day24(BTreeMap<String, bool>, Vec<Gate>);

impl Day {}

impl Solution for Day {
    type Output = String;
    type Item = ();

    fn parse(input: &str) -> Self {
        let (initial_values, connections) = input.split_once("\n\n").unwrap();

        let wires = initial_values
            .lines()
            .fold(BTreeMap::new(), |mut map, next| {
                let (key, value) = next.split_once(":").unwrap();
                let v = value.trim().parse::<u8>().unwrap();
        
                map.insert(key.to_string(), v == 1);
        
                map
            });

        let (_, pending_gates) = parse_logic_gates(connections).unwrap();  

        Self(wires, pending_gates)
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        // let (wires, pending_gates) = (&mut self.0, &mut self.1);
        let mut wires = self.0.clone();
        let mut pending_gates = self.1.clone();

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
                
                let result = match gate.operation {
                    Operation::Or => left | right,
                    Operation::And => left & right,
                    Operation::Xor => left ^ right,
                };
                
                wires.insert(gate.out, result);
                made_progress = true;
            }
        }

        let mut wire_nums: Vec<(usize, bool)> = wires.iter()
            .filter(|(k, _)| k.starts_with("z"))
            .map(|(k, v)| {
                let num = k.trim_start_matches('z').parse::<usize>().unwrap();
                // dbg!(k, num, v);
                (num, *v)
            })
            .collect();

        wire_nums.sort_by_key(|(num, _)| *num);

        let output = wire_nums.iter()
            .rev()
            .map(|(_, v)| if *v { 1u64 } else { 0u64 })
            .fold(0u64, |acc, bit| {
                (acc << 1) | bit
            });

        // self.0 = wires;
        // self.1 = pending_gates;

        Ok(output.to_string())
    }

    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        let instructions = self.1.clone();
        const BIT_LENGTH: usize = 45;

        // dbg!(&self.0);
        // dbg!(&self.1);

        // Track incorrect connections
        let mut incorrect: Vec<String> = Vec::new();
        for i in 0..BIT_LENGTH {
            let id = format!("{:02}", i);

            // dbg!(&id);
    
            // Find specific instructions for this bit
            let xor1 = instructions.iter()
                .find(|instruction| 
                    ((instruction.left == format!("x{}", id) && instruction.right == format!("y{}", id)) ||
                     (instruction.left == format!("y{}", id) && instruction.right == format!("x{}", id))) &&
                    instruction.operation == Operation::Xor
                );
    
            let and1 = instructions.iter()
                .find(|instruction| 
                    ((instruction.left == format!("x{}", id) && instruction.right == format!("y{}", id)) ||
                     (instruction.left == format!("y{}", id) && instruction.right == format!("x{}", id))) &&
                    instruction.operation == Operation::And
                );
    
            let z = instructions.iter()
                .find(|instruction| instruction.out == format!("z{}", id));
    
            // dbg!(&z);

            // Skip if any of the key instructions are missing
            if xor1.is_none() || and1.is_none() || z.is_none() {
                continue;
            }
    
            let xor1 = xor1.unwrap();
            let and1 = and1.unwrap();
            let z = z.unwrap();
    
            // Each z must be connected to an XOR
            if z.operation != Operation::Xor {
                incorrect.push(z.out.clone());
            }
            
            // Each AND must go to an OR (besides the first case as it starts the carry flag)
            let or = instructions.iter()
                .find(|instruction| 
                    instruction.left == and1.out || instruction.right == and1.out
                );
            
            if let Some(or) = or {
                if or.operation != Operation::Or && i > 0 {
                    incorrect.push(and1.out.clone());
                }
            }
    
            // The first XOR must go to XOR or AND
            let after = instructions.iter()
                .find(|instruction| 
                    instruction.left == xor1.out || instruction.right == xor1.out
                );
            
            if let Some(after) = after {
                if after.operation == Operation::Or {
                    incorrect.push(xor1.out.clone());
                }
            }
        }
    
        // Each XOR must be connected to an x, y, or z
        let additional_incorrect: Vec<String> = instructions.iter()
            .filter(|instruction| 
                !instruction.left.starts_with('x') && 
                !instruction.left.starts_with('y') && 
                !instruction.out.starts_with('z') && 
                instruction.operation == Operation::Xor
            )
            .map(|instruction| instruction.out.clone())
            .collect();
        
        
        incorrect.extend(additional_incorrect);
        incorrect.sort();
        
        Ok(incorrect.join(","))
    }
}

// Parse multiple lines
fn parse_logic_gates(input: &str) -> IResult<&str, Vec<Gate>> {
    separated_list1(
        tag("\n"),
        logic_line,
    )(input)
}

// Parse a single line
fn logic_line(input: &str) -> IResult<&str, Gate> {
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
        |(input1, _, op, _, input2, _, _, _, output)| Gate {
            left: input1.to_string(),
            right: input2.to_string(),
            operation: op,
            out: output.to_string(),
        },
    )(input)
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


#[cfg(test)]
mod tests {
    use super::*;

    use ornaments::Part;

    #[test]
    fn test_part1_small() -> miette::Result<()> {
        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        assert_eq!("4", Day::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part1_large() -> miette::Result<()> {
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
        assert_eq!("2024", Day::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "solution is not optimized for the test input, only the full puzzle input")]
    fn test_part2() {
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
        assert_eq!("z00,z01,z02,z05", Day::parse(input).solve(Part::Two).unwrap());
    }
}