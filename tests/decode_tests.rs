#[cfg(test)]
mod tests {
    use emulator::cpu::*;
    use emulator::cpu_instructions::instruction_decoding::*;

    #[test]
    fn test_decode_add_immediate() {
        // ADD r0, r1, #10, setting flags (S bit set)
        // Expected encoding: 0xE281000A
        let instruction = u32::from_le_bytes([0x0A, 0x00, 0x81, 0xE2]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::AddImmediate {
                rd: 0,
                rn: 1,
                imm12: 10,
                set_flags: true
            }
        );
    }

    #[test]
    fn test_decode_add_register_lsl() {
        // ADD r0, r1, r2, LSL #4 (no set flags)
        // Expected: opcode for ADD register = 0x4; rm encoded in low nibble = 0x2.
        // Adjusted encoding: [0x02, 0x02, 0x81, 0xE0] gives expected result.
        let instruction = u32::from_le_bytes([0x02, 0x02, 0x81, 0xE0]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::AddRegister {
                rd: 0,
                rn: 1,
                rm: 2,
                shift: ShiftType::LSL,
                shift_amount: 4,
                set_flags: false
            }
        );
    }

    #[test]
    fn test_decode_add_register_lsl_set_flags() {
        // ADDS r0, r1, r2, LSL #4 (set flags)
        let instruction = u32::from_le_bytes([0x02, 0x02, 0x91, 0xE0]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::AddRegister {
                rd: 0,
                rn: 1,
                rm: 2,
                shift: ShiftType::LSL,
                shift_amount: 4,
                set_flags: true
            }
        );
    }

    #[test]
    fn test_decode_add_register_lsr() {
        // ADD r3, r4, r5, LSR #8 (no set flags)
        let instruction = u32::from_le_bytes([0x25, 0x34, 0x84, 0xE0]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::AddRegister {
                rd: 3,
                rn: 4,
                rm: 5,
                shift: ShiftType::LSR,
                shift_amount: 8,
                set_flags: false
            }
        );
    }

    #[test]
    fn test_decode_add_register_asr() {
        // ADDS r6, r7, r8, ASR #12
        let instruction = u32::from_le_bytes([0x48, 0x66, 0x97, 0xE0]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::AddRegister {
                rd: 6,
                rn: 7,
                rm: 8,
                shift: ShiftType::ASR,
                shift_amount: 12,
                set_flags: true
            }
        );
    }

    #[test]
    fn test_decode_add_register_ror() {
        // ADD r9, r10, r11, ROR #3 (no set flags)
        let instruction = u32::from_le_bytes([0xEB, 0x91, 0x8A, 0xE0]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::AddRegister {
                rd: 9,
                rn: 10,
                rm: 11,
                shift: ShiftType::ROR,
                shift_amount: 3,
                set_flags: false
            }
        );
    }

    #[test]
    fn test_decode_mov_immediate() {
        // MOV r0, #0x55, setting flags
        let instruction = u32::from_le_bytes([0x55, 0x00, 0xA0, 0xE3]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::MovImmediate {
                rd: 0,
                imm12: 0x55,
                set_flags: true
            }
        );
    }

    #[test]
    fn test_decode_mov_register_lsl() {
        // MOV r1, r2, LSL #3, not setting flags.
        let instruction = u32::from_le_bytes([0x82, 0x11, 0xA0, 0xE1]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::MovRegister {
                rd: 1,
                rm: 2,
                shift: ShiftType::LSL,
                shift_amount: 3,
                set_flags: false
            }
        );
    }

    #[test]
    fn test_decode_sub_register() {
        // SUB r0, r1, r2, LSL #0
        let instruction = u32::from_le_bytes([0x02, 0x00, 0x41, 0xE0]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::SubRegister {
                rd: 0,
                rn: 1,
                rm: 2,
                shift: ShiftType::LSL,
                shift_amount: 0,
                set_flags: false
            }
        );
    }

    #[test]
    fn test_decode_sub_immediate() {
        // SUB r0, r1, #10, setting flags.
        let instruction = u32::from_le_bytes([0x0A, 0x00, 0x41, 0xE2]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::SubImmediate {
                rd: 0,
                rn: 1,
                imm12: 10,
                set_flags: true
            }
        );
    }

    #[test]
    fn test_decode_and_immediate() {
        // AND r0, r1, #10, setting flags.
        let instruction = u32::from_le_bytes([0x0A, 0x00, 0x01, 0xE2]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::AndImmediate {
                rd: 0,
                rn: 1,
                imm12: 10,
                set_flags: true
            }
        );
    }

    #[test]
    fn test_decode_and_register() {
        // AND r0, r1, r2, LSL #0 (no set flags)
        let instruction = u32::from_le_bytes([0x02, 0x00, 0x01, 0xE0]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::AndRegister {
                rd: 0,
                rn: 1,
                rm: 2,
                shift: ShiftType::LSL,
                shift_amount: 0,
                set_flags: false
            }
        );
    }

    #[test]
    fn test_decode_orr_immediate() {
        // ORR r0, r1, #10, setting flags.
        let instruction = u32::from_le_bytes([0x0A, 0x00, 0x81, 0xE3]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::OrrImmediate {
                rd: 0,
                rn: 1,
                imm12: 10,
                set_flags: true
            }
        );
    }

    #[test]
    fn test_decode_orr_register() {
        // ORR r0, r1, r2, LSL #0 (no set flags)
        let instruction = u32::from_le_bytes([0x02, 0x00, 0x81, 0xE1]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::OrrRegister {
                rd: 0,
                rn: 1,
                rm: 2,
                shift: ShiftType::LSL,
                shift_amount: 0,
                set_flags: false
            }
        );
    }
    #[test]
    fn test_decode_adc_immediate() {
        //ADC R0,R1,#5
        let instruction: u32 = u32::from_le_bytes([0x05, 0x00, 0xA1, 0xE2]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::AdcImmediate {
                rd: 0,
                rn: 1,
                imm12: 5,
                set_flags: true
            }
        )
    }
    #[test]
    fn test_decode_adc_register() {
        // ADC R2, R3, R4
        let instruction: u32 = u32::from_le_bytes([0x04, 0x20, 0xA3, 0xE0]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::AdcRegister {
                rd: 2,
                rn: 3,
                rm: 4,
                shift: ShiftType::LSL,
                shift_amount: 0,
                set_flags: false
            }
        );
    }

    #[test]
    fn test_decode_sbc_immediate() {
        //SBC r0, r1, #10
        let instruction: u32 = u32::from_le_bytes([0x0A, 0x00, 0xC1, 0xE2]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::SbcImmediate {
                rd: 0,
                rn: 1,
                imm12: 10,
                set_flags: true
            }
        );
    }
    #[test]
    fn test_decode_sbc_register() {
        // SBC r0, r1, r2, LSL #0
        let instruction = u32::from_le_bytes([0x02, 0x00, 0xC1, 0xE0]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::SbcRegister {
                rd: 0,
                rn: 1,
                rm: 2,
                shift: ShiftType::LSL,
                shift_amount: 0,
                set_flags: false
            }
        );
    }
    #[test]
    fn test_eor_immediate_without_s() {
        let instruction: u32 = u32::from_le_bytes([0x05, 0x10, 0x22, 0xE2]); // EOR R1, R2, #5
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::EorImmediate {
                rd: 1,
                rn: 2,
                imm12: 5,
                set_flags: true
            }
        );

        let mut cpu = Cpu::new();
        cpu.cpu_state.set_register(2, 0b1010); // R2 = 10
        cpu.eor_immediate(1, 2, 5, false); // R1 = R2 ^ 5

        assert_eq!(cpu.cpu_state.get_register(1), 0b1010 ^ 0b0101); // 10 ^ 5 = 15 (0b1111)
    }
    #[test]
    fn test_eor_register_with_s() {
        let instruction: u32 = u32::from_le_bytes([0x03, 0x10, 0x32, 0xE0]); // EORS R1, R2, R3
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::EorRegister {
                rd: 1,
                rn: 2,
                rm: 3,
                shift: ShiftType::LSL,
                shift_amount: 0,
                set_flags: true
            }
        );

        let mut cpu = Cpu::new();
        cpu.cpu_state.set_register(2, 0b1100); // R2 = 12
        cpu.cpu_state.set_register(3, 0b1010); // R3 = 10
        cpu.eor_register(1, 2, 3, ShiftType::LSL, 0, true); // R1 = R2 ^ R3

        assert_eq!(cpu.cpu_state.get_register(1), 0b1100 ^ 0b1010); // 12 ^ 10 = 6 (0b0110)
                                                                    // You'll also need to assert the flags here based on the result (e.g., Z flag)
        assert_eq!(cpu.cpu_state.CPSR.is_zero(), false); // Result is not zero
        assert_eq!(cpu.cpu_state.CPSR.is_negative(), false); // MSB is 0
                                                             // Carry and Overflow flags are typically 0 for logical operations
        assert_eq!(cpu.cpu_state.CPSR.is_carry(), false);
        assert_eq!(cpu.cpu_state.CPSR.is_overflow(), false);
    }

    #[test]
    fn test_decode_bic_immediate() {
        //BIC R0, R1, #7
        let instruction: u32 = u32::from_le_bytes([0x07, 0x00, 0xC1, 0xE3]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::BicImmediate {
                rd: 0,
                rn: 1,
                imm12: 7,
                set_flags: true
            }
        );
    }
    #[test]
    fn test_decode_bic_register() {
        // BIC R0, R1, R2
        let instruction = u32::from_le_bytes([0x02, 0x00, 0xC1, 0xE1]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::BicRegister {
                rd: 0,
                rn: 1,
                rm: 2,
                shift: ShiftType::LSL,
                shift_amount: 0,
                set_flags: false
            }
        );
    }

    #[test]
    fn test_decode_cmn_immediate() {
        // CMN R0, #5
        let instruction: u32 = u32::from_le_bytes([0x05, 0x00, 0x70, 0xE3]);
        let decoded = decode_arm(instruction);
        assert_eq!(decoded, Instruction::CmnImmediate { rn: 0, imm12: 5 });
    }

    #[test]
    fn test_decode_cmn_register() {
        // CMN R0, R1
        let instruction: u32 = u32::from_le_bytes([0x01, 0x00, 0x70, 0xE1]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::CmnRegister {
                rn: 0,
                rm: 1,
                shift: ShiftType::LSL,
                shift_amount: 0,
            }
        );
    }
    #[test]
    fn test_decode_cmp_immediate() {
        // CMP R0, #5
        let instruction: u32 = u32::from_le_bytes([0x05, 0x00, 0x50, 0xE3]);
        let decoded = decode_arm(instruction);
        assert_eq!(decoded, Instruction::CmpImmediate { rn: 0, imm12: 5 });
    }

    #[test]
    fn test_decode_cmp_register() {
        // CMP R0, R1
        let instruction: u32 = u32::from_le_bytes([0x01, 0x00, 0x50, 0xE1]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::CmpRegister {
                rn: 0,
                rm: 1,
                shift: ShiftType::LSL,
                shift_amount: 0,
            }
        );
    }
    #[test]
    fn test_decode_mvn_immediate() {
        // MVN R0, #5 (setting flags)
        let instruction: u32 = u32::from_le_bytes([0x05, 0x00, 0xE0, 0xE3]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::MvnImmediate {
                rd: 0,
                imm12: 5,
                set_flags: true
            }
        );
    }

    #[test]
    fn test_decode_mvn_register() {
        // MVN R0, R1 (setting flags, LSL #0)
        let instruction: u32 = u32::from_le_bytes([0x01, 0x00, 0xF0, 0xE1]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::MvnRegister {
                rd: 0,
                rm: 1,
                shift: ShiftType::LSL,
                shift_amount: 0,
                set_flags: true
            }
        );
    }
    #[test]
    fn test_rsb_immediate_flags() {
        // RSBS r0, r1, #10, setting flags.
        let instruction = u32::from_le_bytes([0x0A, 0x00, 0x71, 0xE2]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::RsbImmediate {
                rd: 0,
                rn: 1,
                imm12: 10,
                set_flags: true
            }
        );
    }

    #[test]
    fn test_rsb_register_flags() {
        // RSBS r0, r1, r2, LSL #0
        let instruction = u32::from_le_bytes([0x02, 0x00, 0x71, 0xE0]);
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::RsbRegister {
                rd: 0,
                rn: 1,
                rm: 2,
                shift: ShiftType::LSL,
                shift_amount: 0,
                set_flags: true
            }
        );
    }
    #[test]
    fn test_decode_unknown() {
        let instruction = 0xFFFFFFFF; // Invalid instruction
        let decoded = decode_arm(instruction);
        assert_eq!(decoded, Instruction::Unknown(0xFFFFFFFF));
    }
}
