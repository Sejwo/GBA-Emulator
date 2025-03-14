pub fn decode_instruction(instruction: u32) -> Instruction {
    // Check for MOV Rd, #immediate (Immediate MOV)
    if ((instruction >> 25) & 0b111) == 0b001 && ((instruction >> 21) & 0b1111) == 0b1101 {
        let rd = ((instruction >> 12) & 0b1111) as usize;
        let imm8 = (instruction & 0xFF) as u8;
        let rotate = ((instruction >> 8) & 0xF) as u8;
        let imm = (imm8 as u32).rotate_right((rotate * 2) as u32) as u16;
        return Instruction::MovImmediate { rd, imm16: imm };
    }

    // Check for MOV Rd, Rm (Register MOV)
    if ((instruction >> 26) & 0b11) == 0b00 && ((instruction >> 21) & 0b1111) == 0b1101 {
        let rd = ((instruction >> 12) & 0b1111) as usize;
        let rm = (instruction & 0b1111) as usize;
        return Instruction::MovRegister { rd, rm };
    }

    Instruction::Unknown(instruction)
}

#[derive(Debug)]
pub enum Instruction {
    MovImmediate { rd: usize, imm16: u16 },
    MovRegister { rd: usize, rm: usize },
    Unknown(u32),
}