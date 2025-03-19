use crate::cpu::Cpu;
use crate::cpu_instructions::instruction_decoding::ShiftType; // Import ShiftType

impl Cpu {
    pub fn add_immediate(&mut self, rd: usize, rn: usize, imm12: u32, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let operand_2 = imm12;
        let (result, overflow) = operand_1.overflowing_add(operand_2);
        let carry = result < operand_1; // Correct carry calculation
        self.cpu_state.set_register(rd, result);
        if set_flags {
            self.update_arithmetic_flags(result, carry, overflow);
        }
    }

    pub fn add_register(&mut self, rd: usize, rn: usize, rm: usize, shift: ShiftType, shift_amount: u8, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let (operand_2, carry_out) = self.apply_shift(self.cpu_state.get_register(rm), shift, shift_amount);
        let (result, overflow) = operand_1.overflowing_add(operand_2);
        self.cpu_state.set_register(rd, result);
        if set_flags {
            self.update_arithmetic_flags(result, carry_out, overflow); // Use carry_out from shift
        }
    }

    pub fn sub_immediate(&mut self, rd: usize, rn: usize, imm12: u32, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let operand_2 = imm12;
        let (result, overflow) = operand_1.overflowing_sub(operand_2);
        let carry = operand_1 >= operand_2;  // Carry flag is set if *no* borrow occurred.
        self.cpu_state.set_register(rd, result);
        if set_flags {
            self.update_arithmetic_flags(result, carry, overflow);
        }
    }

    pub fn sub_register(&mut self, rd: usize, rn: usize, rm: usize, shift: ShiftType, shift_amount: u8, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let (operand_2, carry_out) = self.apply_shift(self.cpu_state.get_register(rm), shift, shift_amount);
        let (result, overflow) = operand_1.overflowing_sub(operand_2);
        let carry = operand_1 >= operand_2; // Carry flag is set if *no* borrow occurred
        self.cpu_state.set_register(rd, result);
        if set_flags {
            self.update_arithmetic_flags(result, carry, overflow);
       }
    }

    pub fn and_immediate(&mut self, rd: usize, rn: usize, imm12: u32, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let result = operand_1 & imm12;
        self.cpu_state.set_register(rd, result);
        if set_flags {
            self.update_logical_flags(result, false); // Carry flag is unchanged (in most cases)
        }
    }

    pub fn and_register(&mut self, rd: usize, rn: usize, rm: usize, shift: ShiftType, shift_amount: u8, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let (operand_2, carry_out) = self.apply_shift(self.cpu_state.get_register(rm), shift, shift_amount);
        let result = operand_1 & operand_2;
        self.cpu_state.set_register(rd, result);
        if set_flags {
            self.update_logical_flags(result, carry_out);
        }
    }

    pub fn orr_immediate(&mut self, rd: usize, rn: usize, imm12: u32, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let result = operand_1 | imm12;
        self.cpu_state.set_register(rd, result);
        if set_flags {
            self.update_logical_flags(result, false); //Carry flag unchanged
        }
    }

    pub fn orr_register(&mut self, rd: usize, rn: usize, rm: usize, shift: ShiftType, shift_amount: u8, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let (operand_2, carry_out) = self.apply_shift(self.cpu_state.get_register(rm), shift, shift_amount);
        let result = operand_1 | operand_2;
        self.cpu_state.set_register(rd, result);
        if set_flags {
            self.update_logical_flags(result, carry_out);
        }
    }
        pub fn mov_immediete(&mut self, rd: usize, imm12: u32) {
            let rotated_imm = imm12; // Placeholder.  Implement rotation!
            self.cpu_state.set_register(rd, rotated_imm);
            self.update_logical_flags(rotated_imm, false);
    }
        pub fn mov_register(&mut self, rd: usize, rm: usize, shift: ShiftType, shift_amount: u8, set_flags: bool) {
            let (result, carry) = self.apply_shift(self.cpu_state.get_register(rm), shift, shift_amount);
            self.cpu_state.set_register(rd, result);
            if set_flags{
                self.update_logical_flags(result, carry);
            }

    }
}