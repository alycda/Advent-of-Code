use crate::AocError;

use std::collections::HashMap;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    // Split input into parts
    let parts: Vec<Vec<&str>> = input.trim().split("\n\n")
        .map(|section| section.split('\n').collect())
        .collect();

    // Parse initial wire values
    let mut wires: HashMap<String, i32> = parts[0].iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            (parts[0].to_string(), parts[1].parse().unwrap())
        })
        .collect();

    // Parse instructions
    let mut instructions: Vec<Instruction> = parts[1].iter()
        .map(|line| {
            let tokens: Vec<&str> = line.split(' ').collect();
            Instruction {
                a: tokens[0].to_string(),
                b: tokens[2].to_string(),
                c: tokens[4].to_string(),
                operation: tokens[1].to_string(),
                executed: false,
            }
        })
        .collect();

    const BIT_LENGTH: usize = 45;

    // Track incorrect connections
    let mut incorrect: Vec<String> = Vec::new();

    for i in 0..BIT_LENGTH {
        let id = format!("{:02}", i);

        // Find specific instructions for this bit
        let xor1 = instructions.iter()
            .find(|instruction| 
                ((instruction.a == format!("x{}", id) && instruction.b == format!("y{}", id)) ||
                 (instruction.a == format!("y{}", id) && instruction.b == format!("x{}", id))) &&
                instruction.operation == "XOR"
            );

        let and1 = instructions.iter()
            .find(|instruction| 
                ((instruction.a == format!("x{}", id) && instruction.b == format!("y{}", id)) ||
                 (instruction.a == format!("y{}", id) && instruction.b == format!("x{}", id))) &&
                instruction.operation == "AND"
            );

        let z = instructions.iter()
            .find(|instruction| instruction.c == format!("z{}", id));

        // Skip if any of the key instructions are missing
        if xor1.is_none() || and1.is_none() || z.is_none() {
            continue;
        }

        let xor1 = xor1.unwrap();
        let and1 = and1.unwrap();
        let z = z.unwrap();

        // Each z must be connected to an XOR
        if z.operation != "XOR" {
            incorrect.push(z.c.clone());
        }
        
        // Each AND must go to an OR (besides the first case as it starts the carry flag)
        let or = instructions.iter()
            .find(|instruction| 
                instruction.a == and1.c || instruction.b == and1.c
            );
        
        if let Some(or) = or {
            if or.operation != "OR" && i > 0 {
                incorrect.push(and1.c.clone());
            }
        }

        // The first XOR must go to XOR or AND
        let after = instructions.iter()
            .find(|instruction| 
                instruction.a == xor1.c || instruction.b == xor1.c
            );
        
        if let Some(after) = after {
            if after.operation == "OR" {
                incorrect.push(xor1.c.clone());
            }
        }
    }

    // Each XOR must be connected to an x, y, or z
    let additional_incorrect: Vec<String> = instructions.iter()
        .filter(|instruction| 
            !instruction.a.starts_with('x') && 
            !instruction.a.starts_with('y') && 
            !instruction.c.starts_with('z') && 
            instruction.operation == "XOR"
        )
        .map(|instruction| instruction.c.clone())
        .collect();
    
    incorrect.extend(additional_incorrect);
    incorrect.sort();
    
    Ok(incorrect.join(","))
}

// Helper struct to represent an instruction
#[derive(Debug, Clone)]
struct Instruction {
    a: String,
    b: String,
    c: String,
    operation: String,
    executed: bool,
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
