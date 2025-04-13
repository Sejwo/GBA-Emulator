use std::thread::current;

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
}
