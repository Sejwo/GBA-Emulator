use std::fmt::write;

use crate::cpu::Cpu;
use crate::cpu_instructions::instruction_decoding::Instruction;
use crate::cpu_instructions::instruction_decoding::ShiftType; // Import ShiftType
use crate::memory::Memory;

impl Cpu {
    pub fn load_register(
        &mut self,
        rt: usize,
        rn: usize,
        offset: u32,
        pre_index: bool,
        add: bool,
        write_back: bool,
        memory: &Memory,
    ) {
        let base = self.cpu_state.get_register(rn);
        let effective_address = if add {
            base.wrapping_add(offset)
        } else {
            base.wrapping_sub(offset)
        };
        let addr = if pre_index { effective_address } else { base };

        let value = memory.read_word(addr);
        self.cpu_state.set_register(rt, value);
        if write_back {
            self.cpu_state.set_register(rn, effective_address);
        }
    }
    pub fn load_multiple(
        &mut self,
        rn: usize,
        register_list: u16,
        pre_index: bool,
        add: bool,
        write_back: bool,
        memory: &Memory,
    ) {
        let base = self.cpu_state.get_register(rn);
        let mut addr = if pre_index{
            if add{
                base.wrapping_add(4)
            }else{
                base.wrapping_sub(4)
            }
        }else{
            base
        };
        for reg in 0..16{
            if (register_list >> reg) &1 == 1{
                let value = memory.read_word(addr);
                self.cpu_state.set_register(reg, value);
                addr = addr.wrapping_add(4);
            }
        }
        if write_back{
            let num_regs = register_list.count_ones();
            let new_base = if add{
                base.wrapping_add(num_regs*4)
            }else{
                base.wrapping_sub(num_regs*4)
            };
            self.cpu_state.set_register(rn, new_base);

        }
    }
}

#[allow(dead_code)]
pub fn decode_single_data_transfer(instruction: u32) -> Instruction {
    // We know here that bits 27:26 are 01 (load/store).
    // A key bit: the L bit (bit 20) tells whether it’s a load (1) or store (0).
    if ((instruction >> 20) & 1) == 1 {
        // For now, support only immediate offset LDR.
        if ((instruction >> 25) & 1) == 0 {
            // I bit == 0
            return Instruction::Ldr {
                rt: ((instruction >> 12) & 0xF) as usize,
                rn: ((instruction >> 16) & 0xF) as usize,
                offset: instruction & 0xFFF,
                pre_index: ((instruction >> 24) & 1) == 1,
                add: ((instruction >> 23) & 1) == 1,
                write_back: ((instruction >> 21) & 1) == 1,
            };
        } else {
            // Later: Implement register offset mode.
            return Instruction::Unknown(instruction);
        }
    }
    // If it's not a load, this branch handles store instructions,
    // which you might not be implementing yet.
    Instruction::Unknown(instruction)
}
#[allow(dead_code)]
pub fn decode_block_data_transfer(instruction: u32) -> Instruction {
    // Bits 27:25 are 0b100.
    let rn = ((instruction >> 16) & 0xF) as usize;
    let register_list = (instruction & 0xFFFF) as u16;
    let pre_index = ((instruction >> 24) & 1) == 1;
    let add = ((instruction >> 23) & 1) == 1;
    let write_back = ((instruction >> 21) & 1) == 1;
    let is_load = ((instruction >> 20) & 1) == 1; // L bit: load if 1.
    if is_load {
        Instruction::Ldm {
            rn,
            register_list,
            pre_index,
            add,
            write_back,
        }
    } else {
        // For now, if it’s a store multiple (STM) we return Unknown.
        Instruction::Unknown(instruction)
    }
}
