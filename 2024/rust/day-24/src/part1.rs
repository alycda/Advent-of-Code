use std::collections::{BTreeMap, HashMap};

use crate::AocError;

use nom::{
    branch::alt, bytes::complete::{tag, take_while1}, character::complete::{space0, space1}, combinator::map, multi::separated_list1, sequence::{separated_pair, tuple}, IResult
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
    let (a, b) = input.split_once("\n\n").unwrap();

    let mut wires = a.lines().fold(BTreeMap::new(), |mut map, next| {
        let (key, value) = next.split_once(":").unwrap();
        let v = value.trim().parse::<u8>().unwrap();
        let v = if v == 1 { true } else { false };

        map.insert(key, v);

        map
    });

    // b.lines()
    //     .map(|line| {

    //     })

    //     .filter(|line| {
    //         true
    //     })
    //     .collect::<String>();

    // let mut op_map: HashMap<String, Operation> = HashMap::new();
    // let mut output_map: HashMap<String, String> = HashMap::new();

    // match parse_logic_gates(b) {
    //     Ok((_, gates)) => {
            
    //         for gate in gates {
    //             op_map.insert(gate.input.clone(), gate.op);
    //             output_map.insert(gate.input, gate.output);
    //         }
            
    //         println!("Operation map: {:?}", op_map);
    //         println!("Output map: {:?}", output_map);
    //     }
    //     Err(e) => println!("Parsing error: {:?}", e),
    // }

    let binding = parse_logic_gates(b).unwrap();
    binding.1.iter()
        .inspect(|gates| {
            // dbg!(gates);

            match gates.op {
                Operation::Or => {
                    let left = wires.get(gates.left.as_str()).unwrap();
                    let right = wires.get(gates.right.as_str()).unwrap();

                    wires.insert(&gates.output, left | right);
                }
                Operation::And => {
                    let left = wires.get(gates.left.as_str()).unwrap();
                    let right = wires.get(gates.right.as_str()).unwrap();

                    wires.insert(&gates.output, left & right);
                }
                Operation::Xor => {
                    let left = wires.get(gates.left.as_str()).unwrap();
                    let right = wires.get(gates.right.as_str()).unwrap();

                    wires.insert(&gates.output, left ^ right);
                }
                
            }

        })
        // .rev()
        .count();
        // .collect::<String>();

    let binary = wires.iter()
        .filter(|(k, _)| k.starts_with("z"))
        .map(|(k, v)| {
            // println!("{}: {}", k, v);
            dbg!(k, v);

            if v == &true {
                1
            } else {
                0
            }
        })
        .rev()
        // .collect::<String>();
        // .fold(0u32, |acc, (_, &bit)| {
        //     (acc << 1) | (bit as u32)
        // });
        .fold(0u32, |acc, bit| {  // Now we just have the bit, not a tuple
            (acc << 1) | bit
        });

    // dbg!(wires, op_map, output_map);
    // dbg!(&wires);

    dbg!(binary);

    Ok(binary.to_string())
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
