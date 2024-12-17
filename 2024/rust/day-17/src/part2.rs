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
        let mut low = 1;
        // Using example value as a reasonable starting upper bound
        let mut high = 1_000_000; 
        
        while low < high {
            let mid = low + (high - low) / 2;
            
            if MachineState::verify_output(mid, program) {
                // Found a working value, but might not be lowest
                // Keep searching lower half
                high = mid;
            } else {
                // This value doesn't work, eliminate lower half
                low = mid + 1;
            }
        }
        
        // Verify we actually found a working value
        if MachineState::verify_output(low, program) {
            low
        } else {
            panic!("No solution found in range");
        }
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

    Ok(MachineState::find_lowest_a(&machine.program).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_engineer() -> miette::Result<()> {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        let machine = MachineState::from_str(input).unwrap();

        assert_eq!(117440, MachineState::find_lowest_a(&machine.program));
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!("117440", process(input)?);
        Ok(())
    }
}
