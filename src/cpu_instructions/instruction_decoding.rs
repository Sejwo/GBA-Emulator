// src/cpu_instructions/instruction_decoding.rs

use std::sync::Arc;

use crate::cpu_instructions::branch_ops::decode_branch;
use crate::cpu_instructions::data_proc_instructions::decode_data_processing;
use crate::cpu_instructions::load_store_instructions::{decode_single_data_transfer, decode_block_data_transfer};
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ShiftType {
    LSL,
    LSR,
    ASR,
    ROR,
}
#[allow(dead_code)]
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
    Ldr {
        rt: usize,
        rn: usize,
        offset: u32,
        pre_index: bool,
        add: bool,
        write_back: bool,
    },
    Ldm {
        rn: usize,
        register_list: u16,
        pre_index: bool,
        add: bool,
        write_back: bool,
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
#[allow(dead_code)]
pub fn decode_arm(instruction: u32) -> Instruction {
    // First, special-case branch exchanges.
    if (instruction & 0x0FFFFFF0) == 0x012FFF10 {
        return Instruction::BranchExchange { rm: (instruction & 0xF) as usize };
    } else if (instruction & 0x0FFFFFF0) == 0x012FFF30 {
        return Instruction::BranchLinkExchange { rm: (instruction & 0xF) as usize };
    }

    // First-tier: uses bits 27:26.
    let group26 = (instruction >> 26) & 0b11;
    match group26 {
        0b00 => {
            // Data processing instructions (both immediate and register)
            decode_data_processing(instruction)
        },
        0b01 => {
            // Single data transfer instructions (e.g. LDR/STR)
            decode_single_data_transfer(instruction)
        },
        0b10 => {
            // This group contains both block data transfer AND branch instructions.
            // Uses bits 27:25 (three bits) to differentiate.
            let group25 = (instruction >> 25) & 0b111;
            match group25 {
                0b100 => decode_block_data_transfer(instruction),  // LDM/STM
                0b101 => decode_branch(instruction),               // Branch
                _ => Instruction::Unknown(instruction),
            }
        },
        0b11 => {
            // Other instructions (coprocessor, etc.) not implemented yet.
            Instruction::Unknown(instruction)
        },
        _ => Instruction::Unknown(instruction),
    }
}
