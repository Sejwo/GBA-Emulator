use crate::cpu::Cpu;
use crate::cpu_instructions::instruction_decoding::ShiftType; // Import ShiftType

#[allow(unused)]
impl Cpu {
    #[inline(always)]
    fn _copy_cpsr_to_spsr(&mut self) {
        self.cpu_state.SPSR = self.cpu_state.CPSR;
    }
    fn _copy_spsr_to_cpsr(&mut self) {
        self.cpu_state.CPSR = self.cpu_state.SPSR;
    }

    #[inline(always)]
    pub fn add_immediate(&mut self, rd: usize, rn: usize, imm12: u32, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let operand_2 = imm12;
        let (result, overflow) = operand_1.overflowing_add(operand_2);
        let carry = result < operand_1; // Correct carry calculation
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            self.update_arithmetic_flags(result, carry, overflow);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    #[inline(always)]
    pub fn add_register(
        &mut self,
        rd: usize,
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    ) {
        let operand_1 = self.cpu_state.get_register(rn);
        let (operand_2, carry_out) =
            self.apply_shift(self.cpu_state.get_register(rm), shift, shift_amount);
        let (result, overflow) = operand_1.overflowing_add(operand_2);
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            self.update_arithmetic_flags(result, carry_out, overflow);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    #[inline(always)]
    pub fn sub_immediate(&mut self, rd: usize, rn: usize, imm12: u32, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let operand_2 = imm12;
        let (result, overflow) = operand_1.overflowing_sub(operand_2);
        let carry = operand_1 >= operand_2; // Carry flag is set if *no* borrow occurred.
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            self.update_arithmetic_flags(result, carry, overflow);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    #[inline(always)]
    pub fn sub_register(
        &mut self,
        rd: usize,
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    ) {
        let operand_1 = self.cpu_state.get_register(rn);
        let (operand_2, carry_out) =
            self.apply_shift(self.cpu_state.get_register(rm), shift, shift_amount);
        let (result, overflow) = operand_1.overflowing_sub(operand_2);
        let carry = operand_1 >= operand_2; // Carry flag is set if *no* borrow occurred
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            self.update_arithmetic_flags(result, carry, overflow);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    #[inline(always)]
    pub fn and_immediate(&mut self, rd: usize, rn: usize, imm12: u32, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let result = operand_1 & imm12;
        self.cpu_state.set_register(rd, result);
        if set_flags {
            self.update_logical_flags(result, false); // Carry flag is unchanged (in most cases)
        }
    }
    #[inline(always)]
    pub fn and_register(
        &mut self,
        rd: usize,
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    ) {
        let operand_1 = self.cpu_state.get_register(rn);
        let (operand_2, carry_out) =
            self.apply_shift(self.cpu_state.get_register(rm), shift, shift_amount);
        let result = operand_1 & operand_2;
        self.cpu_state.set_register(rd, result);
        if set_flags {
            self.update_logical_flags(result, carry_out);
        }
    }
    #[inline(always)]
    pub fn orr_immediate(&mut self, rd: usize, rn: usize, imm12: u32, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let result = operand_1 | imm12;
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            self.update_logical_flags(result, false); //Carry flag unchanged
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    #[inline(always)]
    pub fn orr_register(
        &mut self,
        rd: usize,
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    ) {
        let operand_1 = self.cpu_state.get_register(rn);
        let (operand_2, carry_out) =
            self.apply_shift(self.cpu_state.get_register(rm), shift, shift_amount);
        let result = operand_1 | operand_2;
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            self.update_logical_flags(result, false); //Carry flag unchanged
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    #[inline(always)]
    pub fn mov_immediete(&mut self, rd: usize, imm12: u32) {
        let rotated_imm = imm12; // Placeholder.  Implement rotation!
        self.cpu_state.set_register(rd, rotated_imm);
        if rd != 15 {
            self.update_logical_flags(rotated_imm, false);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    #[inline(always)]
    pub fn mov_register(
        &mut self,
        rd: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    ) {
        let (result, carry) =
            self.apply_shift(self.cpu_state.get_register(rm), shift, shift_amount);
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            self.update_logical_flags(result, carry);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    #[inline(always)]
    pub fn adc_immediate(&mut self, rd: usize, rn: usize, imm12: u32, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let operand_2 = imm12;
        let carry_in = if self.cpu_state.CPSR.is_carry() { 1 } else { 0 };
        let (intermediate_result, overflow_1) = operand_1.overflowing_add(operand_2);
        let (result, overflow_2) = intermediate_result.overflowing_add(carry_in);
        let carry_out = overflow_1 | overflow_2;

        // Determine the overflow flag (for signed arithmetic)
        // Get the sign bits by right-shifting by 31 and masking with 1
        let operand_1_sign_bit = (operand_1 as i32) >> 31 & 1;
        let operand_2_sign_bit = (operand_2 as i32) >> 31 & 1;
        let result_sign_bit = (result as i32) >> 31 & 1;

        let overflow =
            (operand_1_sign_bit == operand_2_sign_bit) && (operand_1_sign_bit != result_sign_bit);

        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            self.update_arithmetic_flags(result, carry_out, overflow);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    #[inline(always)]
    pub fn adc_register(
        &mut self,
        rd: usize,
        rm: usize,
        rn: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    ) {
        let operand_1 = self.cpu_state.get_register(rn);
        let operand_2 = self.cpu_state.get_register(rm);
        let carry_in = if self.cpu_state.CPSR.is_carry() { 1 } else { 0 };

        let (shifted_operand_2, shift_carry_out) = self.apply_shift(operand_2, shift, shift_amount);

        let (intermediate_result, overflow_1) = operand_1.overflowing_add(shifted_operand_2);
        let (result, overflow_2) = intermediate_result.overflowing_add(carry_in as u32);
        let carry_out = overflow_1 | overflow_2;

        // Determine the overflow flag (for signed arithmetic)
        let operand_1_sign_bit = (operand_1 as i32) >> 31 & 1;
        let shifted_operand_2_sign_bit = (shifted_operand_2 as i32) >> 31 & 1;
        let result_sign_bit = (result as i32) >> 31 & 1;
        let overflow = (operand_1_sign_bit == shifted_operand_2_sign_bit)
            && (operand_1_sign_bit != result_sign_bit);

        self.cpu_state.set_register(rd, result);

        if set_flags & (rd != 15) {
            self.update_arithmetic_flags(result, carry_out, overflow);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    pub fn sbc_immediate(&mut self, rd: usize, rn: usize, imm12: u32, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let operand_2 = imm12;
        let carry_in = if self.cpu_state.CPSR.is_carry() { 1 } else { 0 };
        let borrow = 1 - carry_in;
        let (intermediate_result, borrow_1) = operand_1.overflowing_sub(operand_2);
        let (result, borrow_2) = intermediate_result.overflowing_sub(borrow as u32);
        let carry_out = !(borrow_1 | borrow_2);

        let operand_1_sign_bit = (operand_1 as i32) >> 31 & 1;
        let operand_2_sign_bit = (operand_2 as i32) >> 31 & 1;
        let result_sign_bit = (result as i32) >> 31 & 1;

        let overflow =
            (operand_1_sign_bit == operand_2_sign_bit) && (operand_1_sign_bit != result_sign_bit);
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            self.update_arithmetic_flags(result, carry_out, overflow);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    pub fn sbc_register(
        &mut self,
        rd: usize,
        rm: usize,
        rn: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    ) {
        let operand_1 = self.cpu_state.get_register(rn);
        let operand_2 = self.cpu_state.get_register(rm);
        let carry_in = if self.cpu_state.CPSR.is_carry() { 1 } else { 0 };
        let borrow = 1 - carry_in;

        let (shifted_operand_2, _) = self.apply_shift(operand_2, shift, shift_amount);

        let (intermediate_result, borrow1) = operand_1.overflowing_sub(shifted_operand_2);
        let (result, borrow2) = intermediate_result.overflowing_sub(borrow as u32);

        let carry_out = !(borrow1 | borrow2);

        // Signed overflow detection
        let operand_1_sign_bit = (operand_1 as i32) >> 31 & 1;
        let operand_2_sign_bit = (operand_2 as i32) >> 31 & 1;
        let result_sign_bit = (result as i32) >> 31 & 1;

        let overflow =
            (operand_1_sign_bit == operand_2_sign_bit) && (operand_1_sign_bit != result_sign_bit);
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            self.update_arithmetic_flags(result, carry_out, overflow);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    pub fn eor_immediate(&mut self, rd: usize, rn: usize, imm12: u32, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let operand_2 = imm12;
        let result = operand_1 ^ operand_2;
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            self.update_logical_flags(result, false);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    pub fn eor_register(
        &mut self,
        rd: usize,
        rm: usize,
        rn: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    ) {
        let operand_1 = self.cpu_state.get_register(rn);
        let (operand_2, carry_out) =
            self.apply_shift(self.cpu_state.get_register(rm), shift, shift_amount);
        let result = operand_1 ^ operand_2;
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            self.update_logical_flags(result, carry_out);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    pub fn bic_immediate(&mut self, rd: usize, rn: usize, imm12: u32, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let operand_2 = imm12;
        let result = operand_1 & (!operand_2);
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            self.update_logical_flags(result, false);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    pub fn bic_register(
        &mut self,
        rd: usize,
        rn: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    ) {
        let operand_1 = self.cpu_state.get_register(rn);
        let (operand_2, carry_out) =
            self.apply_shift(self.cpu_state.get_register(rm), shift, shift_amount);
        let result = operand_1 & (!operand_2);
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            self.update_logical_flags(result, carry_out);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    pub fn cmn_immediate(&mut self, rn: usize, imm12: u32) {
        let operand_1 = self.cpu_state.get_register(rn);
        let operand_2 = imm12;
        let (result, overflow) = operand_1.overflowing_add(operand_2);
        let carry = result < operand_1;
        self.update_arithmetic_flags(result, carry, overflow);
    }
    pub fn cmn_register(&mut self, rn: usize, rm: usize, shift: ShiftType, shift_amount: u8) {
        let operand_1 = self.cpu_state.get_register(rn);
        let (operand_2, carry_out) =
            self.apply_shift(self.cpu_state.get_register(rm), shift, shift_amount);
        let (result, overflow) = operand_1.overflowing_add(operand_2);
        self.update_arithmetic_flags(result, carry_out, overflow);
    }
    pub fn cmp_immediate(&mut self, rn: usize, imm12: u32) {
        let operand_1 = self.cpu_state.get_register(rn);
        let operand_2 = imm12;
        let (result, overflow) = operand_1.overflowing_sub(operand_2);
        let carry = operand_1 >= operand_2;
        self.update_arithmetic_flags(result, carry, overflow);
    }
    pub fn cmp_register(&mut self, rn: usize, rm: usize, shift: ShiftType, shift_amount: u8) {
        let operand_1 = self.cpu_state.get_register(rn);
        let (operand_2, carry_out) =
            self.apply_shift(self.cpu_state.get_register(rm), shift, shift_amount);
        let (result, overflow) = operand_1.overflowing_sub(operand_2);
        let carry = operand_1 >= operand_2;
        self.update_arithmetic_flags(result, carry, overflow);
    }
    #[inline(always)]
    pub fn mvn_immediate(&mut self, rd: usize, imm12: u32, set_flags: bool) {
        let rotate = (imm12 >> 8) & 0xF;
        let imm8 = imm12 & 0xFF;
        let rotated_imm = imm8.rotate_right(rotate * 2);
        let result = !rotated_imm;
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            let carry: bool = if rotate == 0 {
                !self.cpu_state.CPSR.is_carry()
            } else {
                (imm8 >> ((rotate * 2) - 1) % 32) & 1 != 0
            };
            self.update_logical_flags(result, carry);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    pub fn mvn_register(
        &mut self,
        rd: usize,
        rm: usize,
        shift: ShiftType,
        shift_amount: u8,
        set_flags: bool,
    ) {
        let (operand, carry_out) =
            self.apply_shift(self.cpu_state.get_register(rm), shift, shift_amount);
        let result = !operand;
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            self.update_logical_flags(result, carry_out);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    pub fn rsb_immediate(&mut self, rd: usize, rn: usize, imm12: u32, set_flags: bool) {
        let operand_1 = self.cpu_state.get_register(rn);
        let operand_2 = imm12;
        let (result, overflow) = operand_2.overflowing_sub(operand_1);
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15) {
            self.update_arithmetic_flags(result, (operand_1 >= operand_2), overflow);
        } else {
            self._copy_spsr_to_cpsr();
        }
    }
    pub fn rsb_register(&mut self, rd: usize, rn: usize, rm: usize, shift: ShiftType, shift_amount: u8, set_flags: bool){
        let operand_1 = self.cpu_state.get_register(rn);
        let (operand_2, _) = self.apply_shift(self.cpu_state.get_register(rm), shift, shift_amount);
        let (result, overflow) = operand_2.overflowing_sub(operand_1);
        self.cpu_state.set_register(rd, result);
        if set_flags & (rd != 15){
            self.update_arithmetic_flags(result, (operand_1 >= operand_2), overflow);
        }else{
            self._copy_spsr_to_cpsr();
        }
    }
}
