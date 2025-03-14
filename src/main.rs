#[allow(unused_imports)]
use emulator::cpu::Cpu;
use emulator::memory::Memory;

mod memory;
mod cpu;
mod cpu_instructions;
fn main() {
    // Create a memory instance with 1024 bytes.
    let mut memory = Memory::new(1024);

    // Load sample instructions into memory.
    // 1. MOV R5, #123: Encoded in little-endian as [0x7B, 0x50, 0xA0, 0xE3]
    let mov_imm: [u8; 4] = [0x7B, 0x50, 0xA0, 0xE3];
    memory.write_bytes(0, &mov_imm);

    // 2. MOV R3, R5
    let mov_reg: [u8; 4] = [0x05, 0x30, 0xA0, 0xE1];
    memory.write_bytes(4, &mov_reg);

    // 3. Unknown instruction to halt execution: [0xFF, 0xFF, 0xFF, 0xFF]
    let unknown: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
    memory.write_bytes(8, &unknown);

    // Create a CPU instance.
    let mut cpu = Cpu::new();

    // Initialize the program counter (PC) to the start of the program.
    cpu.cpu_state.PC = 0;

    // Run the program. The CPU will fetch, decode, and execute instructions in a loop.
    cpu.run_program(&memory);
}
