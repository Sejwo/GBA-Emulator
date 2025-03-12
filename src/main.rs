#[allow(unused_imports)]
use emulator::cpu::CpuState;
fn main(){
    let mut cpu = CpuState::default();
    cpu.regular_registers[0] = 0b01011;
    cpu.regular_registers[5] = 0xABCDE;
    cpu.PC = 0x08000000;
    cpu.SP = 0x03007F00;
    cpu.CPSR.set_zero(true);
    for (i, register_val) in cpu.regular_registers.iter().enumerate(){
        println!("R{}: 0x{:X} \n", i, register_val);
    }
    println!("PC value: 0x{:X}",cpu.PC);
    println!("SP value: 0x{:X}",cpu.SP);
    cpu.CPSR.display_all_flags();

}