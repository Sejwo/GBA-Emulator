use crate::cpu_instructions::instruction_decoding::{decode_arm, Instruction, ShiftType};
use crate::memory::Memory;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Debug, Default, Copy, Clone)]
pub struct CpuState {
    pub regular_registers: [u32; 16], // Regular registers (r0-r15)
    pub PC: u32,                      // Program Counter
    pub SP: u32,                      // Stack Pointer
    pub CPSR: Cpsr,                   // Current Program Status Register
    pub SPSR: Cpsr,                   // Saved Program Status Register
}
#[allow(dead_code)]
impl CpuState {
    pub fn get_register(&self, reg_num: usize) -> u32 {
        if reg_num < 16 {
            self.regular_registers[reg_num]
        } else {
            panic!("Invalid register number requested: {}", reg_num);
        }
    }

    pub fn set_register(&mut self, reg_num: usize, value: u32) {
        if reg_num < 16 {
            self.regular_registers[reg_num] = value;
        } else {
            panic!("Invalid register number: {}", reg_num)
        }
    }

    pub fn fetch_instruction(&mut self, memory: &Memory) -> (u32, bool) {
        let instruction = memory.read_word(self.PC);
        self.PC += 4;
        (instruction, self.CPSR.is_thumb_state()) // Return instruction and T-bit
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn is_thumb_state(&self) -> bool {
        (self.value >> Self::T_BIT & 1 == 1)
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn set_thumb_state(&mut self, set: bool) {
        if set {
            self.value |= 1 << Self::T_BIT;
        } else {
            self.value &= !1 << Self::T_BIT;
        }
    }
    #[inline(always)]
    #[allow(dead_code)]
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

pub struct Cpu {
    pub cpu_state: CpuState,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            cpu_state: CpuState::default(),
        }
    }

    // Helper function to update flags after arithmetic operations.
    pub fn update_arithmetic_flags(&mut self, result: u32, carry: bool, overflow: bool) {
        self.cpu_state.CPSR.set_zero(result == 0);
        self.cpu_state.CPSR.set_negative((result as i32) < 0);
        self.cpu_state.CPSR.set_carry(carry);
        self.cpu_state.CPSR.set_overflow(overflow);
    }

    // Helper function to update flags after logical operations.
    pub fn update_logical_flags(&mut self, result: u32, carry: bool) {
        self.cpu_state.CPSR.set_zero(result == 0);
        self.cpu_state.CPSR.set_negative((result as i32) < 0);
        self.cpu_state.CPSR.set_carry(carry);
        // Overflow is not affected by logical operations.
    }

    // Placeholder for applying shifts (we'll implement this later).
    pub fn apply_shift(&self, value: u32, shift_type: ShiftType, shift_amount: u8) -> (u32, bool) {
        match shift_type {
            ShiftType::LSL => {
                if shift_amount == 0 {
                    (value, self.cpu_state.CPSR.is_carry()) // Special case for LSL #0
                } else if shift_amount < 32 {
                    let carry_out = (value >> (32 - shift_amount)) & 1 == 1;
                    (value << shift_amount, carry_out)
                } else if shift_amount == 32 {
                    (0, (value & 1) == 1)
                } else {
                    (0, false)
                }
            }
            ShiftType::LSR => {
                if shift_amount == 0 {
                    (value, self.cpu_state.CPSR.is_carry()) // Special case for LSR #0 is LSR #32
                } else if shift_amount < 32 {
                    let carry_out = (value >> (shift_amount - 1)) & 1 == 1;
                    (value >> shift_amount, carry_out)
                } else if shift_amount == 32 {
                    (0, (value >> 31) == 1)
                } else {
                    (0, false)
                }
            }
            ShiftType::ASR => {
                if shift_amount == 0 {
                    (value, self.cpu_state.CPSR.is_carry()) // Special case for ASR #0 is ASR #32
                } else if shift_amount < 32 {
                    let carry_out = (value >> (shift_amount - 1)) & 1 == 1;
                    let result = (value as i32 >> shift_amount) as u32; // Arithmetic shift
                    (result, carry_out)
                } else {
                    //ASR #32 or more
                    let result = if (value as i32) < 0 {
                        !0u32 //All bits 1
                    } else {
                        0u32 // All bits 0
                    };
                    (result, (value as i32) < 0)
                }
            }
            ShiftType::ROR => {
                let shift_amount = shift_amount % 32;
                if shift_amount == 0 {
                    (value, self.cpu_state.CPSR.is_carry()) // Special case for ROR #0 is RRX
                } else {
                    let carry_out = (value >> (shift_amount - 1)) & 1 == 1;
                    let result = value.rotate_right(shift_amount as u32);
                    (result, carry_out)
                }
            }
        }
    }
    // Placeholder for interpreting a single instruction.
    fn interpret_instruction(&mut self, instruction: u32) {
        // Perform the condition check *here*
        let condition_passed = match decode_arm(instruction) {
            Instruction::Nop => true, // NOP always passes
            _ => {
                let condition_code = (instruction >> 28) & 0xF;
                match condition_code {
                    0b0000 => self.cpu_state.CPSR.is_zero(),
                    0b0001 => !self.cpu_state.CPSR.is_zero(),
                    0b0010 => self.cpu_state.CPSR.is_carry(),
                    0b0011 => !self.cpu_state.CPSR.is_carry(),
                    0b0100 => self.cpu_state.CPSR.is_negative(),
                    0b0101 => !self.cpu_state.CPSR.is_negative(),
                    0b0110 => self.cpu_state.CPSR.is_overflow(),
                    0b0111 => !self.cpu_state.CPSR.is_overflow(),
                    0b1000 => self.cpu_state.CPSR.is_carry() && !self.cpu_state.CPSR.is_zero(),
                    0b1001 => !self.cpu_state.CPSR.is_carry() || self.cpu_state.CPSR.is_zero(),
                    0b1010 => {
                        self.cpu_state.CPSR.is_negative() == self.cpu_state.CPSR.is_overflow()
                    }
                    0b1011 => {
                        self.cpu_state.CPSR.is_negative() != self.cpu_state.CPSR.is_overflow()
                    }
                    0b1100 => {
                        !self.cpu_state.CPSR.is_zero()
                            && (self.cpu_state.CPSR.is_negative()
                                == self.cpu_state.CPSR.is_overflow())
                    }
                    0b1101 => {
                        self.cpu_state.CPSR.is_zero()
                            || (self.cpu_state.CPSR.is_negative()
                                != self.cpu_state.CPSR.is_overflow())
                    }
                    0b1110 => true,      // AL (Always)
                    0b1111 => false,     // NV (Never)
                    _ => unreachable!(), // Invalid condition code.
                }
            }
        };
        if condition_passed {
            //If condition is met
            let decoded = decode_arm(instruction);
            match decoded {
                Instruction::MovImmediate {
                    rd,
                    imm12,
                    set_flags,
                } => {
                    self.mov_immediete(rd, imm12);
                }
                Instruction::MovRegister {
                    rd,
                    rm,
                    shift,
                    shift_amount,
                    set_flags,
                } => {
                    self.mov_register(rd, rm, shift, shift_amount, set_flags);
                }
                Instruction::AddImmediate {
                    rd,
                    rn,
                    imm12,
                    set_flags,
                } => {
                    self.add_immediate(rd, rn, imm12, set_flags);
                }
                Instruction::AddRegister {
                    rd,
                    rn,
                    rm,
                    shift,
                    shift_amount,
                    set_flags,
                } => {
                    self.add_register(rd, rn, rm, shift, shift_amount, set_flags);
                }
                Instruction::SubImmediate {
                    rd,
                    rn,
                    imm12,
                    set_flags,
                } => {
                    self.sub_immediate(rd, rn, imm12, set_flags);
                }
                Instruction::SubRegister {
                    rd,
                    rn,
                    rm,
                    shift,
                    shift_amount,
                    set_flags,
                } => {
                    self.sub_register(rd, rn, rm, shift, shift_amount, set_flags);
                }
                Instruction::AndImmediate {
                    rd,
                    rn,
                    imm12,
                    set_flags,
                } => {
                    self.and_immediate(rd, rn, imm12, set_flags);
                }
                Instruction::AndRegister {
                    rd,
                    rn,
                    rm,
                    shift,
                    shift_amount,
                    set_flags,
                } => {
                    self.and_register(rd, rn, rm, shift, shift_amount, set_flags);
                }
                Instruction::OrrImmediate {
                    rd,
                    rn,
                    imm12,
                    set_flags,
                } => {
                    self.orr_immediate(rd, rn, imm12, set_flags);
                }
                Instruction::OrrRegister {
                    rd,
                    rn,
                    rm,
                    shift,
                    shift_amount,
                    set_flags,
                } => {
                    self.orr_register(rd, rn, rm, shift, shift_amount, set_flags);
                }
                // Add the new ADC instruction handling here
                Instruction::AdcImmediate {
                    rd,
                    rn,
                    imm12,
                    set_flags,
                } => {
                    self.adc_immediate(rd, rn, imm12, set_flags);
                }
                Instruction::AdcRegister {
                    rd,
                    rn,
                    rm,
                    shift,
                    shift_amount,
                    set_flags,
                } => {
                    self.adc_register(rd, rn, rm, shift, shift_amount, set_flags);
                }
                Instruction::Unknown(instruction) => {
                    // Handle unknown instructions (e.g., raise an exception).
                    panic!("Unknown instruction: 0x{:X}", instruction);
                }
                Instruction::Nop => {} // Do nothing for NOP
            }
        }
    }

    pub fn run_program(&mut self, memory: &Memory) {
        const HALT_INSTRUCTION: u32 = 0xFFFFFFFF; // Or another sentinel value.
        loop {
            let (instruction, is_thumb) = self.cpu_state.fetch_instruction(memory);
            if is_thumb {
                todo!("Implement Thumb mode")
            }
            if instruction == HALT_INSTRUCTION {
                // Print registers and halt.
                for (i, register) in self.cpu_state.regular_registers.iter().enumerate() {
                    println!("R{}: 0x{:X}", i, register);
                }
                println!("End of program (halt instruction encountered).");
                break;
            }
            // Decode and execute the instruction.
            self.interpret_instruction(instruction);
        }
    }
}
