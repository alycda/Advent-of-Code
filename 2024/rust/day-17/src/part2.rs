use std::i64;

use crate::AocError;

#[derive(Debug)]
struct MachineState {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    program: Vec<u8>,
}

impl MachineState {
    fn from_str(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let lines: Vec<&str> = input.lines().collect();
        
        // Parse registers (assuming format "Register X: value")
        let reg_a = lines[0].split(": ").nth(1)
            .ok_or("Invalid register A format")?
            .parse()?;
        let reg_b = lines[1].split(": ").nth(1)
            .ok_or("Invalid register B format")?
            .parse()?;
        let reg_c = lines[2].split(": ").nth(1)
            .ok_or("Invalid register C format")?
            .parse()?;
            
        // Parse program (assuming format "Program: 0,1,5,4,3,0")
        let program = lines[4].split(": ").nth(1)
            .ok_or("Invalid program format")?
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<u8>, _>>()?;
            
        Ok(MachineState {
            register_a: reg_a,
            register_b: reg_b,
            register_c: reg_c,
            program,
        })
    }

    fn decode_instructions(&self) -> Vec<Instruction> {
        self.program
            .chunks(2)
            .filter_map(|chunk| {
                if chunk.len() == 2 {
                    Some(Instruction {
                        opcode: Opcode::try_from(chunk[0]).ok()?,
                        operand: chunk[1],
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn evaluate_combo(&self, operand: u8) -> i64 {
        match operand {
            0..=3 => operand as i64,  // Literal values 0-3
            4 => self.register_a,      // Value of register A
            5 => self.register_b,      // Value of register B
            6 => self.register_c,      // Value of register C
            7 => panic!("Invalid combo operand 7"), // Reserved
            _ => panic!("Invalid combo operand > 7"),
        }
    }

    fn execute(&mut self) -> Vec<u8> {
        let instructions = self.decode_instructions();
        let mut outputs = Vec::new();
        let mut ip = 0;
    
        // println!("\nTracing execution with A={}", self.register_a);
        
        while ip < instructions.len() {
            let instruction = &instructions[ip];
            // println!("IP: {}, Op: {:?}, Operand: {}, Before: A={}, B={}, C={}", 
            //         ip, instruction.opcode, instruction.operand,
            //         self.register_a, self.register_b, self.register_c);
            
            match instruction.opcode {
                Opcode::Adv => {
                    let power = self.evaluate_combo(instruction.operand);
                    self.register_a = self.register_a / (2_i64.pow(power as u32));
                    ip += 1;
                }
                Opcode::Bxl => {
                    // literal operand for XOR
                    self.register_b ^= instruction.operand as i64;
                    ip += 1;
                }
                Opcode::Bst => {
                    let value = self.evaluate_combo(instruction.operand);
                    self.register_b = value % 8;
                    ip += 1;
                }
                Opcode::Jnz => {
                    if self.register_a != 0 {
                        ip = instruction.operand as usize;
                        continue; // Skip normal ip increment
                    }
                    ip += 1;
                }
                Opcode::Bxc => {
                    self.register_b ^= self.register_c;
                    ip += 1;
                }
                Opcode::Out => {
                    let value = self.evaluate_combo(instruction.operand);
                    outputs.push((value % 8) as u8);
                    ip += 1;
                }
                Opcode::Bdv => {
                    let power = self.evaluate_combo(instruction.operand);
                    self.register_b = self.register_a / (2_i64.pow(power as u32));
                    ip += 1;
                }
                Opcode::Cdv => {
                    let power = self.evaluate_combo(instruction.operand);
                    self.register_c = self.register_a / (2_i64.pow(power as u32));
                    ip += 1;
                }
            }
        }

        // println!("After: A={}, Output={:?}", self.register_a, outputs);

        outputs
    }

    fn verify_output(initial_a: i64, program: &[u8]) -> bool {
        let mut state = MachineState {
            register_a: initial_a,
            register_b: 0,
            register_c: 0,
            program: program.to_vec(),
        };
        
        let output = state.execute();
        output == program
    }

    fn find_lowest_a(program: &[u8]) -> i64 {
        // Start with a reasonable increment to speed up search
        let mut a = 1;
        let increment = 2097152; // Same as Python version
        
        while !Self::verify_output(a, program) {
            a += increment;
        }
        a
    }

    // fn verify_output(initial_a: i64, program: &[u8]) -> bool {
    //     let mut state = MachineState {
    //         register_a: initial_a,
    //         register_b: 0,
    //         register_c: 0,
    //         program: program.to_vec(),
    //     };
        
    //     let output = state.execute();
    //     // if initial_a % 1_000_000 == 0 {
    //     //     println!("Testing A={}, output={:?}, expected={:?}", initial_a, output, program);
    //     // }
        
    //     output == program
    // }

    // fn find_lowest_a(program: &[u8]) -> i64 {
    //     let mut a = 1;
    //     while !Self::verify_output(a, program) {
    //         if a % 1_000_000 == 0 {
    //             // println!("Trying {}", a);
    //         }
    //         a += 1;
    //     }
    //     a
    // }
}

fn calculate_initial_a() -> i64 {    
    let program = vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0];
    
    // Start with value that will output last number (0)
    let mut a = 3i64;  // This will output 0 as last value
    
    // Work backwards through the program, each time multiplying by 8
    // and adding the next required output
    for &target in program.iter().rev().skip(1) {
        a = a * 8 + target as i64;
    }
    
    // One final multiply by 8 
    a * 8
}

// fn calculate_initial_a() -> i64 {    
//     let start = 89057473207128_i64;
//     let end = 974823479571216_i64;
//     let increment = 2097152; // 2^21
//     let program = vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0];
    
//     let mut a = start + (increment - (start % increment));
    
//     while a <= end {
//         // // Print progress every ~billion iterations
//         // if a % 1_000_000_000_000 == 0 {
//         //     println!("Testing {}", a);
//         // }
        
//         let mut state = MachineState {
//             register_a: a,
//             register_b: 0,
//             register_c: 0,
//             program: program.clone(),
//         };
        
//         if state.execute() == program {
//             return a;
//         }
        
//         a += increment;
//     }
    
//     panic!("No solution found in range");
// }

// fn test_value(a: i64, program: &[u8]) -> std::cmp::Ordering {
//     let mut state = MachineState {
//         register_a: a,
//         register_b: 0,
//         register_c: 0,
//         program: program.to_vec(),
//     };
    
//     let output = state.execute();
//     // Debug print every millionth try
//     if a % 1_000_000_000 == 0 {
//         println!("Testing a={}, output={:?}", a, output);
//     }
//     match output.len().cmp(&program.len()) {
//         std::cmp::Ordering::Equal => output.as_slice().cmp(program),
//         other => other
//     }
// }

// fn calculate_initial_a() -> i64 {    
//     let increment = 2097152; // 2^21
//     let mut low = 89057473207128_i64;
//     // Ensure low is aligned
//     low += increment - (low % increment);
    
//     let mut high = 974823479571216_i64;
//     // Ensure high is aligned
//     high -= high % increment;
    
//     let program = vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0];
    
//     println!("Searching from {} to {}", low, high);
    
//     while low <= high {
//         let mid = low + ((high - low) / increment / 2) * increment;
        
//         match test_value(mid, &program) {
//             std::cmp::Ordering::Equal => return mid,
//             std::cmp::Ordering::Less => {
//                 if low == mid {
//                     low = mid + increment;
//                 } else {
//                     low = mid;
//                 }
//             },
//             std::cmp::Ordering::Greater => {
//                 if high == mid {
//                     high = mid - increment;
//                 } else {
//                     high = mid;
//                 }
//             },
//         }
//     }
    
//     panic!("No solution found in range");
// }

// fn test_value(a: i64, program: &[u8]) -> std::cmp::Ordering {
//     let mut state = MachineState {
//         register_a: a,
//         register_b: 0,
//         register_c: 0,
//         program: program.to_vec(),
//     };
    
//     let output = state.execute();
//     // Convert output and program to slices for comparison
//     match output.len().cmp(&program.len()) {
//         std::cmp::Ordering::Equal => output.as_slice().cmp(program),
//         other => other
//     }
// }

// fn calculate_initial_a() -> i64 {    
//     let mut low = 89057473207128_i64;
//     let mut high = 974823479571216_i64;
//     let program = vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0];
    
//     while low <= high {
//         let mid = low + (high - low) / 2;
        
//         match test_value(mid, &program) {
//             std::cmp::Ordering::Equal => return mid,
//             std::cmp::Ordering::Less => low = mid + 2097152,
//             std::cmp::Ordering::Greater => high = mid - 2097152,
//         }
//     }
    
//     panic!("No solution found in range");
// }

// fn test_value(a: i64, program: &[u8]) -> std::cmp::Ordering {
//     let mut state = MachineState {
//         register_a: a,
//         register_b: 0,
//         register_c: 0,
//         program: program.to_vec(),
//     };
    
//     let output = state.execute();
//     // Compare output with program bytes
//     match output.len().cmp(&program.len()) {
//         std::cmp::Ordering::Equal => output.cmp(program),
//         other => other
//     }
// }

// fn calculate_initial_a() -> i64 {    
//     let mut low = 89057473207128_i64;
//     let mut high = 974823479571216_i64;
//     let program = vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0];
    
//     while low <= high {
//         let mid = low + (high - low) / 2;
        
//         match test_value(mid, &program) {
//             std::cmp::Ordering::Equal => return mid,
//             std::cmp::Ordering::Less => low = mid + 2097152, // Maintain alignment with increment
//             std::cmp::Ordering::Greater => high = mid - 2097152,
//         }
//     }
    
//     panic!("No solution found in range");
// }

// fn calculate_initial_a() -> i64 {    
//     let start_value = 35282534841844_i64; // Based on VM analysis
//     let increment = 2097152; // 2^21
//     let program = vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0];
    
//     let mut a = start_value;
    
//     loop {
//         let mut state = MachineState {
//             register_a: a,
//             register_b: 0,
//             register_c: 0,
//             program: program.clone(),
//         };
        
//         if state.execute() == program {
//             return a;
//         }
        
//         a += increment;
//     }
// }

// fn calculate_initial_a() -> i128 {    
//     // Each output needs to generate sequence elements when divided by appropriate powers
//     // Since program ends with ,3,0 we know final operations must produce 3,0
    
//     // Load your exact program bytes for testing
//     let program = vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0];
    
//     // Start with a value that will generate final 3,0 sequence
//     let mut a = 3i128;
    
//     // Work backwards through desired outputs, adjusting value
//     for &target in program.iter().rev().skip(1) {
//         a = a * 8 + target as i128;
//     }
    
//     // Final *8 to get initial value
//     a *= 8;
    
//     a
// }

// fn calculate_initial_a() -> i128 {    
//     // Work with i128 to handle larger numbers
//     let power: i128 = 8_i128.pow(9); // 8^9 for program length 16/2
//     let mut ast: i128 = 1;
    
//     loop {
//         // The program numbers themselves form an octal number
//         let octal_part = i128::from_str_radix("241775170341553", 8).unwrap();
//         let a = ast * power + octal_part;
        
//         // Test if this value works
//         let mut state = MachineState {
//             register_a: a as i64,
//             register_b: 0,
//             register_c: 0,
//             program: vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0],
//         };
        
//         if state.execute() == state.program {
//             return a;
//         }
        
//         ast += 1;
//     }
// }

// fn calculate_initial_a(program: &[u8]) -> i64 {    
//     // Work backwards from known constraints
//     let base = 8_i64.pow(program.len() as u32 / 2);  // Based on program length
//     let mut attempt = 0;
    
//     // Convert program to octal equivalent since we're working with mod 8
//     let target_value = i64::from_str_radix(
//         &program.iter()
//             .map(|x| x.to_string())
//             .collect::<Vec<_>>()
//             .join(""), 
//         8
//     ).unwrap();
    
//     while attempt * base + target_value < 0 {
//         attempt += 1;
//     }
    
//     attempt * base + target_value
// }

// fn calculate_initial_a() -> i128 {    
//     let power: i128 = 8_i128.pow(9); // 8^9
//     let mut ast: i128 = 1;
    
//     loop {
//         // Convert from octal 66752888 to decimal
//         let octal_part = i128::from_str_radix("66752888", 8).unwrap();
//         let a = ast * power + octal_part;
        
//         // Test if this value works
//         let mut state = MachineState {
//             register_a: a as i64,
//             register_b: 0,
//             register_c: 0,
//             program: vec![0,3,5,4,3,0],
//         };
        
//         if state.execute() == state.program {
//             return a.try_into().unwrap();
//         }
        
//         ast += 1;
//     }
//  }

// fn calculate_initial_a(desired_outputs: &[u8]) -> i64 {
//     // Start with 3 since we need the sequence to end with a 3 that becomes 0
//     let mut a = 3;
//     println!("Starting with a=3 (needed to end sequence)");

//     // Each previous value needs to:
//     // 1. Generate correct remainder when divided by 8
//     // 2. Divide down to our next target
//     for &desired in desired_outputs.iter().rev().skip(1) {
//         // Multiply current by 8 and add whatever remainder we need
//         // to get our desired output for this step
//         let next_a = (a * 8) + desired as i64;
//         println!("Need {} to get output {} and divide to {}", next_a, desired, a);
//         a = next_a;
//     }
    
//     // Final multiply by 8 for initial value
//     let final_a = a * 8;
//     println!("Final value: {}", final_a);
    
//     final_a
// }

// fn calculate_initial_a(desired_outputs: &[u8]) -> i64 {
//     // Work backwards from the end
//     let mut a = 0_i64;
    
//     // We need the last non-zero value to be 3 
//     // (since it needs to output 0 and then stop)
//     a = 3;
    
//     // Work backwards through our desired outputs
//     for &output in desired_outputs.iter().rev().skip(1) {
//         a = a * 8 + output as i64;
//         println!("Building up A: {}", a);
//     }
    
//     // Final multiply by 8 to get our initial value
//     a = a * 8;
    
//     println!("Final A: {}", a);
//     a
//  }
 
 // Test function
//  fn verify_sequence() {
//     let desired = [0, 3, 5, 4, 3, 0];
//     let a = calculate_initial_a(&desired);
    
//     // Verify this generates our sequence
//     let mut state = MachineState {
//         register_a: a,
//         register_b: 0,
//         register_c: 0,
//         program: desired.to_vec(),
//     };
    
//     let output = state.execute();
//     assert_eq!(output, desired);
//     println!("Verified: {} generates {:?}", a, output);
//  }

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Adv = 0, // Division with A register
    Bxl = 1, // XOR with B register
    Bst = 2, // Store to B register
    Jnz = 3, // Conditional jump
    Bxc = 4, // XOR B and C registers
    Out = 5, // Output value
    Bdv = 6, // Division to B register
    Cdv = 7, // Division to C register
}

impl TryFrom<u8> for Opcode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::Adv),
            1 => Ok(Opcode::Bxl),
            2 => Ok(Opcode::Bst),
            3 => Ok(Opcode::Jnz),
            4 => Ok(Opcode::Bxc),
            5 => Ok(Opcode::Out),
            6 => Ok(Opcode::Bdv),
            7 => Ok(Opcode::Cdv),
            _ => Err(format!("Invalid opcode: {}", value)),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    operand: u8,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // let instructions = vec![];

    let machine = MachineState::from_str(input).unwrap();

    // dbg!(&machine);

    // let instructions = dbg!(machine.decode_instructions());

    // let outputs = machine.execute();

    // dbg!(&outputs.join(","));

    // Ok(format!("{outputs:?}"))
    // Ok(outputs.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(","))

    // Ok(MachineState::find_lowest_a(&machine.program).to_string())

    // Ok(calculate_initial_a(&machine.program).to_string())

    // Ok(MachineState::find_lowest_a(&machine.program).to_string())

    Ok(calculate_initial_a().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

//     #[test]
//     fn test_reverse_engineer() -> miette::Result<()> {
//         let input = "Register A: 2024
// Register B: 0
// Register C: 0

// Program: 0,3,5,4,3,0";
//         let machine = MachineState::from_str(input).unwrap();

//         assert_eq!(117_440, MachineState::find_lowest_a(&machine.program));
//         Ok(())
//     }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!("11744", process(input)?);
        Ok(())
    }
}
