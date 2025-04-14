use std::thread::current;

use crate::cpu_instructions::instruction_decoding::Instruction;
use crate::cpu::Cpu;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BranchType {
    B,
    BL,
}

#[allow(dead_code)]
pub struct BranchInstruction {
    pub branch_type: BranchType,
    pub imm24: u32,
}

#[allow(dead_code)]
pub trait BranchOps {
    fn execute_branch(&mut self, branch_type: BranchType, imm24: u32, raw_instr: u32);
    fn branch_exchange(&mut self, rm: usize);
    fn branch_link_exchange(&mut self, rm:usize);
}
#[allow(dead_code)]
impl BranchOps for Cpu {
    fn execute_branch(&mut self, branch_type: BranchType, imm24: u32, raw_instr: u32) {
        // Compute the branch offset: shift the 24-bit immediate left by 2.
        let mut offset = (imm24 << 2) as i32;
        // Sign-extend the 26-bit result to 32 bits.
        if (offset & (1 << 25)) != 0 {
            offset |= !0x03FF_FFFF; // set upper 6 bits to 1.
        }
        // Get the current PC, which is already advanced by 4 from fetch_instruction.
        let current_pc = self.cpu_state.get_register(15);
        // Compute target_pc as current_pc + offset (without adding extra 4).
        let target_pc = current_pc.wrapping_add(offset as u32);
        
        // For Branch with Link, set LR (r14) to the current PC.
        if branch_type == BranchType::BL {
            self.cpu_state.set_register(14, current_pc);
        }
        
        // Set the PC to the computed target.
        self.cpu_state.set_register(15, target_pc);
        println!(
            "Branch executed: raw 0x{:08X}, type: {:?}, imm24: 0x{:06X}, offset: {}, target_pc set to: 0x{:08X}",
            raw_instr, branch_type, imm24, offset, target_pc
        );  
    }
    fn branch_exchange(&mut self, rm:usize){
        let target = self.cpu_state.get_register(rm);
        if (target & 1) == 1{
            self.cpu_state.CPSR.set_thumb_state(true);
        }else{
            self.cpu_state.CPSR.set_thumb_state(false);
        }
        self.cpu_state.set_register(15, target&!1);
        println!("BX executed: Branching to 0x{:09X}", (target &!1))
    }
    fn branch_link_exchange(&mut self, rm:usize) {
        let return_address = self.cpu_state.get_register(15);
        self.cpu_state.set_register(14, return_address);
        println!("BLX executed: Saving return address 0x{:08X} in R14", return_address);
        self.branch_exchange(rm);
    }
}



pub fn decode_branch(instruction:u32) -> Instruction{
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
    }else{
        return Instruction::Unknown(instruction);
    }
}