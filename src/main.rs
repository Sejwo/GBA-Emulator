#[allow(unused_imports)]
use emulator::cpu::Cpu;
use emulator::memory::Memory;

fn main() {
    let mut cpu = Cpu::new();
    let memory = Memory::new(1024);

    cpu.cpu_state.set_register(5, 99);
    println!("Value of R5 before MOV: {}", cpu.cpu_state.get_register(5));

    // MOV R5, #123: Little-Endian bytes: [0x7B, 0x50, 0xA0, 0xE3]
    let immediate_move_instruction: u32 = u32::from_le_bytes([0x7B, 0x50, 0xA0, 0xE3]);
    cpu.interpret_instruction(immediate_move_instruction);
    println!("Value of R5 after MOV R5, #123: {}", cpu.cpu_state.get_register(5));

    cpu.cpu_state.set_register(7, 0xABC);
    cpu.cpu_state.set_register(3, 0);
    // MOV R3, R7: Little-Endian bytes: [0x07, 0x30, 0xA0, 0xE1]
    let mov_r3_r7: u32 = u32::from_le_bytes([0x07, 0x30, 0xA0, 0xE1]);
    cpu.interpret_instruction(mov_r3_r7);
    println!("Value of R3 after MOV R3, R7: 0x{:X}", cpu.cpu_state.get_register(3));

    cpu.cpu_state.set_register(12, 0x123);
    cpu.cpu_state.set_register(1, 0);
    // MOV R1, R12: Little-Endian bytes: [0x0C, 0x10, 0xA0, 0xE1]
    let mov_r1_r12: u32 = u32::from_le_bytes([0x0C, 0x10, 0xA0, 0xE1]);
    cpu.interpret_instruction(mov_r1_r12);
    println!("Value of R1 after MOV R1, R12: 0x{:X}", cpu.cpu_state.get_register(1));

    cpu.cpu_state.set_register(0, 0x999);
    cpu.cpu_state.set_register(14, 0);
    // MOV R14, R0: Little-Endian bytes: [0x00, 0xE0, 0xA0, 0xE1]
    let mov_r14_r0: u32 = u32::from_le_bytes([0x00, 0xE0, 0xA0, 0xE1]);
    cpu.interpret_instruction(mov_r14_r0);
    println!("Value of R14 after MOV R14, R0: 0x{:X}", cpu.cpu_state.get_register(14));
}