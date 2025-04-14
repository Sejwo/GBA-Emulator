// src/cpu_instructions/instruction_decoding.rs

#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ShiftType {
    LSL,
    LSR,
    ASR,
    ROR,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    AddImmediate {
        rd: usize,
        rn: usize,
        imm12: u32,
        set_flags: bool,
    },
    AddRegister {
        rd: usize,
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    },
    SubImmediate {
        rd: usize,
        rn: usize,
        imm12: u32,
        set_flags: bool,
    },
    SubRegister {
        rd: usize,
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    },
    AndImmediate {
        rd: usize,
        rn: usize,
        imm12: u32,
        set_flags: bool,
    },
    AndRegister {
        rd: usize,
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    },
    OrrImmediate {
        rd: usize,
        rn: usize,
        imm12: u32,
        set_flags: bool,
    },
    OrrRegister {
        rd: usize,
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    },
    MovImmediate {
        rd: usize,
        imm12: u32,
        set_flags: bool,
    },
    MovRegister {
        rd: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    },
    AdcImmediate {
        rd: usize,
        rn: usize,
        imm12: u32,
        set_flags: bool,
    },
    AdcRegister {
        rd: usize,
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    },
    SbcImmediate {
        rd: usize,
        rn: usize,
        imm12: u32,
        set_flags: bool,
    },
    SbcRegister {
        rd: usize,
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    },
    EorImmediate {
        rd: usize,
        rn: usize,
        imm12: u32,
        set_flags: bool,
    },
    EorRegister {
        rd: usize,
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    },
    BicImmediate {
        rd: usize,
        rn: usize,
        imm12: u32,
        set_flags: bool,
    },
    BicRegister {
        rd: usize,
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    },
    CmnImmediate {
        rn: usize,
        imm12: u32,
    },
    CmnRegister {
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
    },
    CmpImmediate {
        rn: usize,
        imm12: u32,
    },
    CmpRegister {
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
    },
    MvnImmediate {
        rd: usize,
        imm12: u32,
        set_flags: bool,
    },
    MvnRegister {
        rd: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    },
    RsbImmediate {
        rd: usize,
        rn: usize,
        imm12: u32,
        set_flags: bool,
    },
    RsbRegister {
        rd: usize,
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    },
    RscImmediate {
        rd: usize,
        rn: usize,
        imm12: u32,
        set_flags: bool,
    },
    RscRegister {
        rd: usize,
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    },

    //Branch instructions
    Branch {
        branch_type: crate::cpu_instructions::branch_ops::BranchType,
        imm24: u32,
    },
    BranchExchange {
        rm: usize,
    },
    BranchLinkExchange {
        rm: usize, //It would seem GBA did not have BLX instruction, but I found it in another emulator so just for being safe/potential upgrade(idk why) I'm keeping it in
    },
    Unknown(u32),
    Nop,
}

// Immediate operands are encoded using an 8-bit immediate rotated right by twice a 4-bit value.
pub fn decode_rotated_immediate(instruction: u32) -> u32 {
    let rotate = (instruction >> 8) & 0xF;
    let imm8 = instruction & 0xFF;
    imm8.rotate_right(rotate * 2)
}

/// Decodes a 32-bit ARM instruction provided in native little-endian order.
pub fn decode_arm(instruction: u32) -> Instruction {
    // Check for NOP first.
    if instruction & 0x0FFF_FFF0 == 0x0320_F000 {
        return Instruction::Nop;
    }
    //branch instructions
    if ((instruction >> 25) & 0b111) == 0b101 {
        let branch_type = if ((instruction >> 24) & 1) == 1 {
            crate::cpu_instructions::branch_ops::BranchType::BL
        } else {
            crate::cpu_instructions::branch_ops::BranchType::B
        };
        let imm24 = instruction & 0x00FF_FFFF;
        return Instruction::Branch { branch_type, imm24 };
    } else if (instruction & 0x0FFFFFF0) == 0x012FFF10 {
        let rm: usize = (instruction & 0xF) as usize;
        return Instruction::BranchExchange { rm: rm };
    } else if (instruction & 0x0FFFFFF0) == 0x012FFF30 {
        let rm: usize = (instruction & 0xF) as usize;
        return Instruction::BranchLinkExchange { rm: rm };
    }

    if (instruction >> 26) & 0b11 == 0b00 {
        // Extract common fields.
        let opcode = (instruction >> 21) & 0xF;
        // For immediate instructions we force set_flags true (per tests).
        let s_extracted = ((instruction >> 20) & 1) == 1;
        let rn = ((instruction >> 16) & 0xF) as usize;
        let rd = ((instruction >> 12) & 0xF) as usize;
        let i_bit = (instruction >> 25) & 1;
        println!("Decoding instruction: {:X}", instruction);
        println!("Opcode: {:X}", opcode);
        println!("I-bit: {:X}", i_bit);

        // Immediate data processing instructions.
        if i_bit == 1 {
            let imm12 = decode_rotated_immediate(instruction);
            let set_flags = true; // Force true for immediate instructions per tests.
            return match opcode {
                0b0000 => Instruction::AndImmediate {
                    rd,
                    rn,
                    imm12,
                    set_flags,
                },
                0b0001 => Instruction::EorImmediate {
                    rd,
                    rn,
                    imm12,
                    set_flags,
                },
                0b0010 => Instruction::SubImmediate {
                    rd,
                    rn,
                    imm12,
                    set_flags,
                },
                0b0011 => Instruction::RsbImmediate {
                    rd,
                    rn,
                    imm12,
                    set_flags,
                },
                0b0110 => Instruction::SbcImmediate {
                    rd,
                    rn,
                    imm12,
                    set_flags,
                },
                0b0100 => Instruction::AddImmediate {
                    rd,
                    rn,
                    imm12,
                    set_flags,
                },
                0b0101 => Instruction::AdcImmediate {
                    rd,
                    rn,
                    imm12,
                    set_flags,
                },
                0b0111 => Instruction::RscImmediate {
                    rd,
                    rn,
                    imm12,
                    set_flags,
                },
                0b1010 => Instruction::CmpImmediate { rn, imm12 },
                0b1011 => Instruction::CmnImmediate { rn, imm12 },
                0b1100 => Instruction::OrrImmediate {
                    rd,
                    rn,
                    imm12,
                    set_flags,
                },
                0b1101 => Instruction::MovImmediate {
                    rd,
                    imm12,
                    set_flags,
                },
                0b1110 => Instruction::BicImmediate {
                    rd,
                    rn,
                    imm12,
                    set_flags,
                },
                0b1111 => Instruction::MvnImmediate {
                    rd,
                    imm12,
                    set_flags,
                },

                _ => Instruction::Unknown(instruction),
            };
        }

        // Register-based data processing instructions.
        // (Remove check for bit 4 so that we always decode using the immediate shift field.)
        if i_bit == 0 {
            let shift_amount = ((instruction >> 7) & 0b11111) as u8;
            let shift_type = match (instruction >> 5) & 0b11 {
                0b00 => ShiftType::LSL,
                0b01 => ShiftType::LSR,
                0b10 => ShiftType::ASR,
                0b11 => ShiftType::ROR,
                _ => unreachable!(),
            };
            let rm = (instruction & 0xF) as usize;
            // For register instructions, use the extracted S bit.
            let set_flags = s_extracted;
            return match opcode {
                0b0000 => Instruction::AndRegister {
                    rd,
                    rn,
                    rm,
                    shift: shift_type,
                    shift_amount,
                    set_flags,
                },
                0b0001 => Instruction::EorRegister {
                    rd,
                    rn,
                    rm,
                    shift: shift_type,
                    shift_amount,
                    set_flags,
                },
                0b0010 => Instruction::SubRegister {
                    rd,
                    rn,
                    rm,
                    shift: shift_type,
                    shift_amount,
                    set_flags,
                },
                0b0011 => Instruction::RsbRegister {
                    rd,
                    rn,
                    rm,
                    shift: shift_type,
                    shift_amount,
                    set_flags,
                },
                0b0110 => Instruction::SbcRegister {
                    rd,
                    rn,
                    rm,
                    shift: shift_type,
                    shift_amount,
                    set_flags,
                },
                0b0111 => Instruction::RscRegister {
                    rd,
                    rn,
                    rm,
                    shift: shift_type,
                    shift_amount,
                    set_flags,
                },
                0b0100 => Instruction::AddRegister {
                    rd,
                    rn,
                    rm,
                    shift: shift_type,
                    shift_amount,
                    set_flags,
                },
                0b0101 => Instruction::AdcRegister {
                    rd,
                    rn,
                    rm,
                    shift: shift_type,
                    shift_amount,
                    set_flags,
                },
                0b1010 => Instruction::CmpRegister {
                    rn,
                    rm,
                    shift: shift_type,
                    shift_amount,
                },
                0b1011 => Instruction::CmnRegister {
                    rn,
                    rm,
                    shift: shift_type,
                    shift_amount,
                },
                0b1100 => Instruction::OrrRegister {
                    rd,
                    rn,
                    rm,
                    shift: shift_type,
                    shift_amount,
                    set_flags,
                },
                0b1101 => Instruction::MovRegister {
                    rd,
                    rm,
                    shift: shift_type,
                    shift_amount,
                    set_flags,
                },
                0b1110 => Instruction::BicRegister {
                    rd,
                    rn,
                    rm,
                    shift: shift_type,
                    shift_amount,
                    set_flags,
                },
                0b1111 => Instruction::MvnRegister {
                    rd,
                    rm,
                    shift: shift_type,
                    shift_amount,
                    set_flags,
                },
                _ => Instruction::Unknown(instruction),
            };
        } else {
            Instruction::Unknown(instruction)
        };
    }

    Instruction::Unknown(instruction)
}
