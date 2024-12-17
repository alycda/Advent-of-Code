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
}

fn find_initial_a(program: &[u8], next_val: i64, index: i32) -> i64 {
    if index < 0 {
        return next_val;
    }
    
    // Try 8 possible values for current position
    for a_val in (next_val * 8)..((next_val * 8) + 8) {
        let mut state = MachineState {
            register_a: a_val,
            register_b: 0,
            register_c: 0,
            program: program.to_vec(),
        };
        
        let output = state.execute();
        if !output.is_empty() && output[0] == program[index as usize] {
            let final_val = find_initial_a(program, a_val, index - 1);
            if final_val >= 0 {
                return final_val;
            }
        }
    }
    
    -1
}

fn calculate_initial_a() -> i64 {    
    let program = vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0];
    find_initial_a(&program, 0, (program.len() - 1) as i32)
}

// fn find_a(a: i64, p: &[u8], depth: usize) -> i64 {
//     if p.is_empty() {
//         return a;
//     }
    
//     // Collect all possible values first
//     let mut possibilities = Vec::new();
    
//     for bc_init in 0..8 {
//         let mut bc = bc_init as i64;
//         let ta = (a << 3) | bc_init as i64;
//         bc ^= 1_i64;
//         let c = (ta >> bc) as i64;
//         bc ^= 5_i64;
//         bc ^= c;
        
//         if (bc & 7) as u8 == p[0] {
//             possibilities.push((bc_init, ta));
//         }
//     }
    
//     // Try all possibilities
//     for (_, ta) in possibilities {
//         let aa = find_a(ta, &p[1..], depth + 1);
//         if aa > 0 {
//             return aa;
//         }
//     }
    
//     -1
// }

// fn calculate_initial_a() -> i64 {    
//     let program = vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0];
//     let mut reversed = program.clone();
//     reversed.reverse();
    
//     find_a(0, &reversed, 0)
// }

// fn find_a(a: i64, p: &[u8], depth: usize) -> i64 {
//     if p.is_empty() {
//         println!("Found complete solution: {}", a);
//         return a;
//     }
    
//     if depth < 5 {  // Increased debug depth
//         println!("Depth {}: target={}, current_a={:#x}", depth, p[0], a);
//     }
    
//     for bc_init in 0..8 {
//         let mut bc = bc_init as i64;
//         let ta = (a << 3) | bc;
        
//         // Debug the transformation steps
//         if depth < 5 {
//             println!("  try bc={}: ta={:#x}", bc_init, ta);
//         }
        
//         bc ^= 1_i64;
//         let c = (ta >> bc) as i64;
//         bc ^= 5_i64;
//         bc ^= c;
        
//         if (bc & 7) as u8 == p[0] {
//             if depth < 5 {
//                 println!("  Found potential: bc={}, next_a={:#x}", bc_init, ta);
//             }
//             let aa = find_a(ta, &p[1..], depth + 1);
//             if aa > 0 {
//                 return aa;
//             }
//         }
//     }
    
//     -1
// }

// fn calculate_initial_a() -> i64 {    
//     let program = vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0];
//     let mut reversed = program.clone();
//     reversed.reverse();
    
//     // Let's try with a non-zero starting value
//     for start in 0..8 {
//         println!("\nTrying with start value: {}", start);
//         let result = find_a(start, &reversed, 0);
//         if result > 0 {
//             return result;
//         }
//     }
    
//     -1
// }

// fn find_a(a: i64, p: &[u8], depth: usize) -> i64 {
//     if p.is_empty() {
//         println!("Found leaf solution: {}", a);
//         return a;
//     }
    
//     if depth < 3 {  // Only print first few levels to avoid spam
//         println!("Depth {}: trying to generate {:?} with a={}", depth, p[0], a);
//     }
    
//     // Try all possible values 0-7 that could have generated this output
//     for bc_init in 0..8 {
//         let mut bc = bc_init as i64;
//         let ta = (a << 3) | bc;
//         bc ^= 1_i64;
//         let c = (ta >> bc) as i64;
//         bc ^= 5_i64;
//         bc ^= c;
        
//         // Check if this would generate our target output
//         if (bc & 7) as u8 == p[0] {
//             if depth < 3 {
//                 println!("Depth {}: Found match with bc_init={}", depth, bc_init);
//             }
//             // Recursively try the rest of the program
//             let aa = find_a(ta, &p[1..], depth + 1);
//             if aa > 0 {
//                 return aa;
//             }
//         }
//     }
    
//     if depth < 3 {
//         println!("Depth {}: No solution found", depth);
//     }
//     -1
// }

// fn calculate_initial_a() -> i64 {    
//     let program = vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0];
//     let mut reversed = program.clone();
//     reversed.reverse();
    
//     println!("Starting search with reversed program: {:?}", reversed);
//     let result = find_a(0, &reversed, 0);
//     println!("Search finished with result: {}", result);
//     result
// }

// fn find_a(a: i64, p: &[u8]) -> i64 {
//     if p.is_empty() {
//         return a;
//     }
    
//     // Try all possible values 0-7 that could have generated this output
//     for bc_init in 0..8 {
//         let mut bc = bc_init as i64;
//         let ta = (a << 3) | bc;
//         bc ^= 1_i64;
//         let c = (ta >> bc) as i64;
//         bc ^= 5_i64;
//         bc ^= c;
        
//         // Check if this would generate our target output
//         if (bc & 7) as u8 == p[0] {
//             // Recursively try the rest of the program
//             let aa = find_a(ta, &p[1..]);
//             if aa > 0 {
//                 return aa;
//             }
//         }
//     }
    
//     -1
// }

// fn calculate_initial_a() -> i64 {    
//     let program = vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0];
//     let mut reversed = program.clone();
//     reversed.reverse();
    
//     find_a(0, &reversed)
// }

// fn find_a(a: i64, p: &[u8]) -> i64 {
//     if p.is_empty() {
//         return a;
//     }
    
//     // Try all possible values 0-7 that could have generated this output
//     for bc in 0..8 {
//         let mut bc = bc;
//         let ta = (a << 3) | bc;
//         bc ^= 1;
//         let c = (ta >> bc) as u8;
//         bc ^= 5;
//         bc ^= c;
        
//         // Check if this would generate our target output
//         if bc & 7 == p[0] {
//             // Recursively try the rest of the program
//             let aa = find_a(ta, &p[1..]);
//             if aa > 0 {
//                 return aa;
//             }
//         }
//     }
    
//     -1
// }

// fn calculate_initial_a() -> i64 {    
//     let program = vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0];
//     let mut reversed = program.clone();
//     reversed.reverse();
    
//     find_a(0, &reversed)
// }

// fn calculate_initial_a() -> i64 {    
//     // Program we need to output
//     let program = vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0];
    
//     // Convert to base-8 since we're working with mod 8 operations
//     let octal = format!("{:o}", 89057473207128_i64);
//     println!("Starting octal: {}", octal);
    
//     let increment = 2097152; // 2^21
//     let mut a = 89057473207128_i64;
//     // Align to increment
//     a += increment - (a % increment);
    
//     // Only test values that align with our output pattern
//     while a < 974823479571216_i64 {
//         if a % 1_000_000_000_000 == 0 {
//             println!("Testing: {}", a);
//         }
        
//         let mut state = MachineState {
//             register_a: a,
//             register_b: 0,
//             register_c: 0,
//             program: program.clone(),
//         };
        
//         let output = state.execute();
        
//         // Check first few values as a quick filter
//         if output.len() >= 3 && output[0] == 2 && output[1] == 4 && output[2] == 1 {
//             if output == program {
//                 return a;
//             }
//         }
        
//         a += increment;
//     }
    
//     panic!("No solution found");
// }

// fn calculate_initial_a() -> i64 {    
//     let program = vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0];
    
//     // Start with value that will output last number (0)
//     let mut a = 3i64;  // This will output 0 as last value
    
//     // Work backwards through the program, each time multiplying by 8
//     // and adding the next required output
//     for &target in program.iter().rev().skip(1) {
//         a = a * 8 + target as i64;
//     }
    
//     // One final multiply by 8 
//     a * 8
// }

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

    // let machine = MachineState::from_str(input).unwrap();

    // dbg!(&machine);

    // let instructions = dbg!(machine.decode_instructions());

    // let outputs = machine.execute();

    // dbg!(&outputs.join(","));

    // Ok(format!("{outputs:?}"))
    // Ok(outputs.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(","))

    // Ok(MachineState::find_lowest_a(&machine.program).to_string())

    // Ok(calculate_initial_a(&machine.program).to_string())

    // Ok(MachineState::find_lowest_a(&machine.program).to_string())

    // Ok(calculate_initial_a().to_string())

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
