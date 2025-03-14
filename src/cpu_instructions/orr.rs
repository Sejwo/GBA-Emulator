use crate::cpu::Cpu;

impl Cpu{
    pub fn orr_immediate(&mut self, rd:usize, rn:usize, imm12:u32){
        let result = self.cpu_state.get_register(rn) | imm12;
        self.cpu_state.set_register(rd, result);
        self.cpu_state.CPSR.set_zero(result ==0);
        self.cpu_state.CPSR.set_negative((result as i32) < 0);
        self.cpu_state.CPSR.set_carry(false);
        self.cpu_state.CPSR.set_overflow(false);
    }

    pub fn orr_register(&mut self, rd:usize, rn:usize, rm:usize){
        let result = self.cpu_state.get_register(rn) | self.cpu_state.get_register(rm);
        self.cpu_state.set_register(rd, result);
        self.cpu_state.CPSR.set_zero(result ==0);
        self.cpu_state.CPSR.set_negative((result as i32) < 0);
        self.cpu_state.CPSR.set_carry(false);
        self.cpu_state.CPSR.set_overflow(false);
    }
}