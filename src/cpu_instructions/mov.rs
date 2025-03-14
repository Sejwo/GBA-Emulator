use crate::cpu::Cpu;


impl Cpu{
    pub fn mov_immediete(&mut self, rd: usize, imm16:u16){
        self.cpu_state.set_register(rd, imm16 as i32);
    }
    pub fn mov_register(&mut self, rd: usize, rm: usize){
        let source_value = self.cpu_state.get_register(rm);
        self.cpu_state.set_register(rd, source_value);
    }
}