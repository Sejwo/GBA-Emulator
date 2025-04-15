use std::fmt::write;

use crate::cpu::Cpu;
use crate::cpu_instructions::instruction_decoding::Instruction;
use crate::cpu_instructions::instruction_decoding::ShiftType; // Import ShiftType
use crate::memory::Memory;

impl Cpu {
    pub fn load_register(
        //LDR
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
        //LDM
        &mut self,
        rn: usize,
        register_list: u16,
        pre_index: bool,
        add: bool,
        write_back: bool,
        memory: &Memory,
    ) {
        let base = self.cpu_state.get_register(rn);
        let mut addr = if pre_index {
            if add {
                base.wrapping_add(4)
            } else {
                base.wrapping_sub(4)
            }
        } else {
            base
        };
        for reg in 0..16 {
            if (register_list >> reg) & 1 == 1 {
                let value = memory.read_word(addr);
                self.cpu_state.set_register(reg, value);
                addr = addr.wrapping_add(4);
            }
        }
        if write_back {
            let num_regs = register_list.count_ones();
            let new_base = if add {
                base.wrapping_add(num_regs * 4)
            } else {
                base.wrapping_sub(num_regs * 4)
            };
            self.cpu_state.set_register(rn, new_base);
        }
    }
    pub fn load_register_byte( //LDRB
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
        let byte_val = memory.read_byte(addr);
        let value = byte_val as u32;
        self.cpu_state.set_register(rt, value);
        if write_back{
            self.cpu_state.set_register(rn, effective_address);
        }
    }
    pub fn load_doubleword( //LDRD
        &mut self, 
        rt: usize,
        rn: usize,
        offset: u32,
        pre_index: bool,
        add: bool,
        write_back: bool,
        memory: &Memory,
    ){
        let base = self.cpu_state.get_register(rn);
        let effective_address = if add{
            base.wrapping_add(offset)
        }else{
            base.wrapping_sub(offset)
        };
        let addr = if pre_index {effective_address} else {base};
        let lower_word = memory.read_word(addr);
        let upper_word = memory.read_word(addr.wrapping_add(4));
        self.cpu_state.set_register(rt, lower_word);
        self.cpu_state.set_register(rt + 1 , upper_word);
        if write_back {
            self.cpu_state.set_register(rn, effective_address);
        }
    }
}

#[allow(dead_code)]
pub fn decode_single_data_transfer(instruction: u32) -> Instruction {
    // We treat it as a load if bit 20 is 1.
    if ((instruction >> 20) & 1) == 1 {
        // Use the lower 12 bits as the offset regardless of I.
        let rt = ((instruction >> 12) & 0xF) as usize;
        let rn = ((instruction >> 16) & 0xF) as usize;
        let offset = instruction & 0xFFF; // immediate offset
        let pre_index = ((instruction >> 24) & 1) == 1;
        let add = ((instruction >> 23) & 1) == 1;
        let write_back = ((instruction >> 21) & 1) == 1;
        // B bit: bit22 indicates a byte transfer.
        let b_bit = ((instruction >> 22) & 1) == 1;
        // Check bits 7 to 4 for a doubleword indicator. (Here we assume 0b1010 means LDRD.)
        let bits_7_to_4 = (instruction >> 4) & 0xF;
        if bits_7_to_4 == 0b1010 {
            return Instruction::Ldrd { rt, rn, offset, pre_index, add, write_back };
        } else if b_bit {
            return Instruction::Ldrb { rt, rn, offset, pre_index, add, write_back };
        } else {
            return Instruction::Ldr { rt, rn, offset, pre_index, add, write_back };
        }
    }
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
