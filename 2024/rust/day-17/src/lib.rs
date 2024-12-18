use ornaments::{AocError, Solution};

pub use crate::MachineState as Day;

#[derive(Debug)]
pub struct MachineState {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    program: Vec<u8>,
}

impl MachineState {
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
        let mut ip = 0;  // instruction pointer

        while ip < instructions.len() {
            let instruction = &instructions[ip];
            
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

        outputs
    }
}

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


impl Solution for MachineState {
    type Output = String;
    type Item = u8;

    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        
        // Parse registers (assuming format "Register X: value")
        let register_a = lines[0].split(": ").nth(1)
            .ok_or("Invalid register A format").unwrap()
            .parse().unwrap();
        let register_b = lines[1].split(": ").nth(1)
            .ok_or("Invalid register B format").unwrap()
            .parse().unwrap();
        let register_c = lines[2].split(": ").nth(1)
            .ok_or("Invalid register C format").unwrap()
            .parse().unwrap();
            
        // Parse program (assuming format "Program: 0,1,5,4,3,0")
        let program = lines[4].split(": ").nth(1)
            .ok_or("Invalid program format").unwrap()
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<u8>, _>>().unwrap();
            
        Self {
            register_a,
            register_b,
            register_c,
            program,
        }
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        let outputs = self.execute();

        Ok(outputs.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(","))
    }

    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        Ok(find_initial_a(&self.program, 0, (self.program.len() - 1) as i32).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", MachineState::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_reverse_engineer() -> miette::Result<()> {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        let machine = MachineState::parse(input);

        assert_eq!(117_440, find_initial_a(&machine.program, 0, (machine.program.len() - 1) as i32));
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!("117440", MachineState::parse(input).solve(Part::Two)?);
        Ok(())
    }
}