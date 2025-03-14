use crate::cpu::Cpu;


impl Cpu{
    pub fn mov_immediete(&mut self, rd: usize, imm16: u16){
        self.cpu_state.CPSR.set_zero(imm16 as u32 == 0);
        self.cpu_state.CPSR.set_negative((imm16 as i32) < 0);
        self.cpu_state.set_register(rd, imm16 as u32);
    }
    pub fn mov_register(&mut self, rd: usize, rm:usize){
        let val_m = self.cpu_state.get_register(rm);
        self.cpu_state.CPSR.set_zero(val_m == 0);
        self.cpu_state.CPSR.set_negative((val_m as i32) < 0);
        self.cpu_state.set_register(rd, val_m);

    }
}
