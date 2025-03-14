use crate::cpu::Cpu;

impl Cpu {
    pub fn add_immediate(&mut self, rd: usize, rn: usize, imm12: u32) {
        let operand_1 = self.cpu_state.get_register(rn);
        let operand_2 = imm12;
        let (result, overflow) = operand_1.overflowing_add(operand_2);
        self.cpu_state.set_register(rd, result);
        self.cpu_state.CPSR.set_zero(result == 0);
        self.cpu_state.CPSR.set_negative((result as i32) < 0);
        self.cpu_state.CPSR.set_carry(result < operand_1);
        self.cpu_state.CPSR.set_overflow(overflow);
    }
    pub fn add_register(&mut self, rd: usize, rn: usize, rm: usize) {
        let operand_1 = self.cpu_state.get_register(rn);
        let operand_2 = self.cpu_state.get_register(rm);
        let (result, overflow) = operand_1.overflowing_add(operand_2);
        self.cpu_state.set_register(rd, result);
        self.cpu_state.CPSR.set_zero(result == 0);
        self.cpu_state.CPSR.set_negative((result as i32) < 0);
        self.cpu_state.CPSR.set_carry(result < operand_1);
        self.cpu_state.CPSR.set_overflow(overflow);
    }
}
