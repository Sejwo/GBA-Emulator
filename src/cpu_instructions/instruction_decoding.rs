#[derive(Debug)]
pub enum Instruction {
    MovImmediate { rd: usize, imm16: u16 },
    MovRegister { rd: usize, rm: usize },
    AddImmediate { rd: usize, rn: usize, imm12: u32 },
    AddRegister { rd: usize, rn: usize, rm: usize },
    SubImmediate { rd: usize, rn: usize, imm12: u32 },
    SubRegister { rd: usize, rn: usize, rm: usize },
    AndImmediate { rd: usize, rn: usize, imm12: u32 },
    AndRegister { rd: usize, rn: usize, rm: usize },
    OrrImmediate { rd: usize, rn: usize, imm12: u32 },
    OrrRegister { rd: usize, rn: usize, rm: usize },
    Unknown(u32),
}

pub fn decode_instruction(instruction: u32) -> Instruction {
    // MOV Immediate: pattern E3A0xxxx
    if (instruction >> 16) & 0xFFFF == 0xE3A0 {
        let rd = ((instruction >> 12) & 0xF) as usize;
        let imm8 = (instruction & 0xFF) as u8;
        let rotate = ((instruction >> 8) & 0xF) as u8;
        let imm16 = (imm8 as u32).rotate_right((rotate * 2) as u32) as u16;
        return Instruction::MovImmediate { rd, imm16 };
    }
    
    // Data Processing instructions:
    // Check that bits 27-26 are 00
    if (instruction >> 26) & 0x3 == 0 {
        // Extract opcode (bits 24-21) and I-bit (bit 25)
        let opcode = (instruction >> 21) & 0xF;
        let immediate_operand = (instruction >> 25) & 0x1;
        match opcode {
            0b1101 => { // MOV Register (opcode for MOV register is 1101)
                if immediate_operand == 0 {
                    let rd = ((instruction >> 12) & 0xF) as usize;
                    let rm = (instruction & 0xF) as usize;
                    return Instruction::MovRegister { rd, rm };
                }
            }
            0b0100 => { // ADD
                if immediate_operand == 1 {
                    let rd = ((instruction >> 12) & 0xF) as usize;
                    let rn = ((instruction >> 16) & 0xF) as usize;
                    let imm12 = instruction & 0xFFF;
                    return Instruction::AddImmediate { rd, rn, imm12 };
                } else {
                    let rd = ((instruction >> 12) & 0xF) as usize;
                    let rn = ((instruction >> 16) & 0xF) as usize;
                    let rm = (instruction & 0xF) as usize;
                    return Instruction::AddRegister { rd, rn, rm };
                }
            }
            0b0010 => { // SUB
                if immediate_operand == 1 {
                    let rd = ((instruction >> 12) & 0xF) as usize;
                    let rn = ((instruction >> 16) & 0xF) as usize;
                    let imm12 = instruction & 0xFFF;
                    return Instruction::SubImmediate { rd, rn, imm12 };
                } else {
                    let rd = ((instruction >> 12) & 0xF) as usize;
                    let rn = ((instruction >> 16) & 0xF) as usize;
                    let rm = (instruction & 0xF) as usize;
                    return Instruction::SubRegister { rd, rn, rm };
                }
            }
            0b0000 => { // AND
                if immediate_operand == 1 {
                    let rd = ((instruction >> 12) & 0xF) as usize;
                    let rn = ((instruction >> 16) & 0xF) as usize;
                    let imm12 = instruction & 0xFFF;
                    return Instruction::AndImmediate { rd, rn, imm12 };
                } else {
                    let rd = ((instruction >> 12) & 0xF) as usize;
                    let rn = ((instruction >> 16) & 0xF) as usize;
                    let rm = (instruction & 0xF) as usize;
                    return Instruction::AndRegister { rd, rn, rm };
                }
            }
            0b1100 => { // ORR
                if immediate_operand == 1 {
                    let rd = ((instruction >> 12) & 0xF) as usize;
                    let rn = ((instruction >> 16) & 0xF) as usize;
                    let imm12 = instruction & 0xFFF;
                    return Instruction::OrrImmediate { rd, rn, imm12 };
                } else {
                    let rd = ((instruction >> 12) & 0xF) as usize;
                    let rn = ((instruction >> 16) & 0xF) as usize;
                    let rm = (instruction & 0xF) as usize;
                    return Instruction::OrrRegister { rd, rn, rm };
                }
            }
            _ => {}
        }
    }
    Instruction::Unknown(instruction)
}
