use crate::memory::Memory;
use crate::cpu_instructions::instruction_decoding::{Instruction, decode_instruction};

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Debug, Default, Copy, Clone)]
pub struct CpuState {
    pub regular_registers: [u32; 16], //regular registers
    pub PC: u32,                      //Program Counter
    pub SP: u32,                      //Stack Pointer
    pub CPSR: Cpsr, //Current Program Status Register uses custom struct for clearer flag management
}

impl CpuState {
    pub fn get_register(&self, reg_num: usize) -> u32 {
        if reg_num < 16 {
            self.regular_registers[reg_num] as u32
        } else {
            panic!("Invalid register number requested: {}", reg_num);
        }
    }
    pub fn set_register(&mut self, reg_num: usize, value: u32) -> () {
        if reg_num < 16 {
            self.regular_registers[reg_num] = value;
        } else {
            panic!("Invalid register number: {}", reg_num)
        }
    }
    pub fn fetch_instruction(&mut self, memory: &Memory) -> u32 {
        let instruction = memory.read_word(self.PC);
        self.PC += 4;
        instruction
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Cpsr {
    value: u32, //Stores the raw CPSR
}
impl Cpsr {
    const NEGATIVE_BIT: u32 = 31;
    const ZERO_BIT: u32 = 30;
    const CARRY_BIT: u32 = 29;
    const OVERFLOW_BIT: u32 = 28;
    const I_BIT: u32 = 7; //IRQ disable
    const F_BIT: u32 = 8; //FIQ disable
    const T_BIT: u32 = 5; //Thumb state bit
                          //Later implementation of mode bits (0-4)

    //------flag access methods getters and setters------

    //  ---Negative Flag (N)---

    #[inline(always)]
    #[allow(unused_parens)]
    pub fn is_negative(&self) -> bool {
        (self.value >> Self::NEGATIVE_BIT & 1 == 1)
    }

    #[inline(always)]

    pub fn set_negative(&mut self, set: bool) {
        if set {
            self.value |= 1 << Self::NEGATIVE_BIT;
        } else {
            self.value &= !1 << Self::NEGATIVE_BIT;
        }
    }

    //  ---Zero flag(Z)---

    #[inline(always)]
    #[allow(unused_parens)]
    pub fn is_zero(&self) -> bool {
        (self.value >> Self::ZERO_BIT & 1 == 1)
    }

    #[inline(always)]
    pub fn set_zero(&mut self, set: bool) {
        if set {
            self.value |= 1 << Self::ZERO_BIT;
        } else {
            self.value &= 1 << Self::ZERO_BIT;
        }
    }

    //  ---Carry flag(C)---

    #[inline(always)]
    #[allow(unused_parens)]
    pub fn is_carry(&self) -> bool {
        (self.value >> Self::CARRY_BIT & 1 == 1)
    }

    #[inline(always)]
    pub fn set_carry(&mut self, set: bool) {
        if set {
            self.value |= 1 << Self::CARRY_BIT;
        } else {
            self.value &= 1 << Self::CARRY_BIT;
        }
    }

    //  ---Overflow flag(V)---

    #[inline(always)]
    #[allow(unused_parens)]
    pub fn is_overflow(&self) -> bool {
        (self.value >> Self::OVERFLOW_BIT & 1 == 1)
    }

    #[inline(always)]
    pub fn set_overflow(&mut self, set: bool) {
        if set {
            self.value |= 1 << Self::OVERFLOW_BIT;
        } else {
            self.value &= 1 << Self::OVERFLOW_BIT;
        }
    }

    // --- I (IRQ Disable) Flag ---

    #[inline(always)]
    #[allow(unused_parens)]
    pub fn is_irq_disabled(&self) -> bool {
        (self.value >> Self::I_BIT & 1 == 1)
    }

    #[inline(always)]
    pub fn set_irq_disabled(&mut self, set: bool) {
        if set {
            self.value |= 1 << Self::I_BIT;
        } else {
            self.value &= !1 << Self::I_BIT;
        }
    }

    // --- F (FIQ Disable) Flag ---

    #[inline(always)]
    #[allow(unused_parens)]
    pub fn is_fiq_disabled(&self) -> bool {
        (self.value >> Self::F_BIT & 1 == 1)
    }

    #[inline(always)]
    pub fn set_fiq_disabled(&mut self, set: bool) {
        if set {
            self.value |= 1 << Self::F_BIT;
        } else {
            self.value &= !1 << Self::F_BIT;
        }
    }

    // --- T (Thumb State) Flag ---

    #[inline(always)]
    #[allow(unused_parens)]
    pub fn is_thumb_state(&self) -> bool {
        (self.value >> Self::T_BIT & 1 == 1)
    }

    #[inline(always)]
    pub fn set_thumb_state(&mut self, set: bool) {
        if set {
            self.value |= 1 << Self::T_BIT;
        } else {
            self.value &= !1 << Self::T_BIT;
        }
    }
    #[inline(always)]
    pub fn display_all_flags(&self) -> () {
        println!("Is negative: {}\nIs zero: {}\nIs carry: {}\nIs overflow: {} \nIs IRQ disabled: {} \nIs FIQ disabled: {} \nIs Thumb state: {} ",
                self.is_negative(),
                self.is_zero(),
                self.is_carry(),
                self.is_overflow(),
                self.is_irq_disabled(),
                self.is_fiq_disabled(),
                self.is_thumb_state()
                )
    }
}

pub struct Cpu{
    pub cpu_state: CpuState,
}
impl Cpu {
    pub fn new() -> Self {
        Cpu {
            cpu_state: CpuState::default(),
        }
    }

    pub fn interpret_instruction(&mut self, instruction: u32) {
        let decoded = decode_instruction(instruction);
        match decoded {
            Instruction::MovImmediate { rd, imm16 } => {
                self.mov_immediete(rd, imm16);
            }
            Instruction::MovRegister { rd, rm } => {
                self.mov_register(rd, rm);
            }
            Instruction::AddImmediate { rd, rn, imm12 } => {
                self.add_immediate(rd, rn, imm12);
            }
            Instruction::AddRegister { rd, rn, rm } => {
                self.add_register(rd, rn, rm);
            }
            Instruction::SubImmediate { rd, rn, imm12 } => {
                self.sub_immediate(rd, rn, imm12);
            }
            Instruction::SubRegister { rd, rn, rm } => {
                self.sub_register(rd, rn, rm);
            }
            Instruction::AndImmediate { rd, rn, imm12 } => {
                self.and_immediate(rd, rn, imm12);
            }
            Instruction::AndRegister { rd, rn, rm } => {
                self.and_register(rd, rn, rm);
            }
            Instruction::OrrImmediate { rd, rn, imm12 } => {
                self.orr_immediate(rd, rn, imm12);
            }
            Instruction::OrrRegister { rd, rn, rm } => {
                self.orr_register(rd, rn, rm);
            }
            Instruction::Unknown(u32) => {
                todo!("Add instruction for 0x{:X}", u32)
            }
        }
    }
    pub fn run_program(&mut self, memory: &Memory) {
        const HALT_INSTRUCTION: u32 = 0xFFFFFFFF;
        loop {
            // Fetch the next instruction using the PC in CpuState.
            let instruction = self.cpu_state.fetch_instruction(memory);
            if instruction == HALT_INSTRUCTION{
                for (i, register) in self.cpu_state.regular_registers.iter().enumerate(){
                    println!("R{}: 0x{:X} \n", i, register);
                }
                println!("End of program");
                break;
            }
            let decoded = decode_instruction(instruction);
            match decoded {
                Instruction::Unknown(val) => {
                    println!("Encountered unknown instruction: 0x{:08X}. Halting execution.", val);
                    break;
                }
                _ => {
                    // Execute the instruction.
                    self.interpret_instruction(instruction);
                }
            }
        }
    }
}