#[cfg(test)]
mod tests {
    const HALT: u32 = 0xFFFFFFFF;
    use emulator::cpu::*;
    use emulator::cpu_instructions::branch_ops::*;
    use emulator::cpu_instructions::instruction_decoding::*;
    use emulator::memory::Memory;

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
    fn test_rsc_register_without_s() {
        let instruction: u32 = u32::from_le_bytes([0x03, 0x10, 0xE2, 0xE0]); // RSC R1, R2, R3
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::RscRegister {
                rd: 1,
                rn: 2,
                rm: 3,
                shift: ShiftType::LSL,
                shift_amount: 0,
                set_flags: false,
            }
        );

        let mut cpu = Cpu::new();
        cpu.cpu_state.CPSR.set_carry(true);
        cpu.cpu_state.set_register(2, 3); // R2 = 3
        cpu.cpu_state.set_register(3, 10); // R3 = 10
        cpu.rsc_register(1, 2, 3, ShiftType::LSL, 0, false); // R1 = R3 - R2 - !C = 10 - 3 - 0 = 7

        assert_eq!(cpu.cpu_state.get_register(1), 7);
    }

    #[test]
    fn test_rsc_register_with_s() {
        let instruction: u32 = u32::from_le_bytes([0x03, 0x10, 0xF2, 0xE0]); // RSCS R1, R2, R3
        let decoded = decode_arm(instruction);
        assert_eq!(
            decoded,
            Instruction::RscRegister {
                rd: 1,
                rn: 2,
                rm: 3,
                shift: ShiftType::LSL,
                shift_amount: 0,
                set_flags: true,
            }
        );

        let mut cpu = Cpu::new();
        cpu.cpu_state.CPSR.set_carry(false);
        cpu.cpu_state.set_register(2, 10); // R2 = 10
        cpu.cpu_state.set_register(3, 1); // R3 = 1
        cpu.rsc_register(1, 2, 3, ShiftType::LSL, 0, true); // R1 = R3 - R2 - !C = 1 - 10 - 1 = -10
        cpu.cpu_state.CPSR.display_all_flags();
        assert_eq!(cpu.cpu_state.get_register(1) as i32, -10);
        assert_eq!(cpu.cpu_state.CPSR.is_zero(), false);
        assert_eq!(cpu.cpu_state.CPSR.is_negative(), true);
        assert_eq!(cpu.cpu_state.CPSR.is_carry(), false);
        assert_eq!(cpu.cpu_state.CPSR.is_overflow(), false);
    }
    #[test]
    fn test_branch_instruction_from_raw() {
        // Create memory with a size large enough (here, 1024 bytes)
        let mut memory = Memory::new(1024);

        // Write the branch (B) instruction at address 0.
        // Encoding: 0xEA000006
        // Breakdown:
        //  - Condition: 0xE (AL)
        //  - Bits [27:25]: 101 (branch)
        //  - Bit 24: 0 (B, not BL)
        //  - imm24: 6 (i.e., 0x000006)
        // In little-endian, the bytes are: [0x06, 0x00, 0x00, 0xEA].
        let branch_instruction: [u8; 4] = [0x06, 0x00, 0x00, 0xEA];
        memory.write_bytes(0, &branch_instruction);

        // Write the halt instruction (0xFFFFFFFF) at the branch target address.
        // As calculated below, for imm24 = 6:
        //   offset = (6 << 2) = 24.
        // Assume fetch_instruction adds 4 when reading,
        // so current PC becomes 4 when executing the branch.
        // Then target_pc = 4 + 24 + 4 = 32.
        // Write halt (0xFFFFFFFF) starting at address 32.
        let halt_instruction: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
        memory.write_bytes(32, &halt_instruction);

        // Create a new CPU instance.
        let mut cpu = Cpu::new();

        // Initialize the PC (register 15) to 0.
        cpu.cpu_state.set_register(15, 0);

        // Run the program.
        cpu.run_program(&mut memory);

        // When fetching from memory, fetch_instruction increments PC by 4.
        // So when the branch is executed, current PC = 4.
        // For imm24 = 6:
        //   Offset = 6 << 2 = 24.
        // Adding a pipeline offset of 4 gives target PC = 4 + 24 + 4 = 32.
        // Since there is a halt instruction at address 32, the CPU should stop.
        // We then assert that PC is 32.
        let expected_pc = 32;
        assert_eq!(
            cpu.cpu_state.get_register(15),
            expected_pc,
            "Branch target PC should be {}",
            expected_pc
        );
    }

    // Test for a Branch with Link (BL) instruction.
    #[test]
    fn test_branch_with_link_instruction_from_raw() {
        let mut memory = Memory::new(1024);

        // Write the BL instruction at address 0.
        // BL is encoded as 0xEB000006.
        // In little-endian, the bytes are: [0x06, 0x00, 0x00, 0xEB].
        let branch_link_instruction: [u8; 4] = [0x06, 0x00, 0x00, 0xEB];
        memory.write_bytes(0, &branch_link_instruction);

        // Write the halt instruction at the expected branch target.
        // Using the same calculation: target PC = 4 (after fetch) + 24 + 4 = 32.
        let halt_instruction: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
        memory.write_bytes(32, &halt_instruction);

        let mut cpu = Cpu::new();
        cpu.cpu_state.set_register(15, 0);

        cpu.run_program(&mut memory);

        // For BL, in addition to updating PC to 32, the link register (r14) should be set.
        // Convention: r14 receives the PC value before the branch (which is 4).
        let expected_pc = 32;
        let expected_lr = 4;
        assert_eq!(
            cpu.cpu_state.get_register(15),
            expected_pc,
            "Branch with link target PC should be {}",
            expected_pc
        );
        assert_eq!(
            cpu.cpu_state.get_register(14),
            expected_lr,
            "Link register (r14) should be {} for BL",
            expected_lr
        );
    }
    #[test]
    fn test_decode_bx() {
        // 0xE12FFF10 is the standard encoding for BX R0.
        let bx_instr: u32 = 0xE12FFF10;
        let decoded = decode_arm(bx_instr);
        assert_eq!(decoded, Instruction::BranchExchange { rm: 0 });
    }

    // Test the decoding of a BLX instruction.
    #[test]
    fn test_decode_blx() {
        // 0xE12FFF30 is the encoding for BLX R0.
        let blx_instr: u32 = 0xE12FFF30;
        let decoded = decode_arm(blx_instr);
        assert_eq!(decoded, Instruction::BranchLinkExchange { rm: 0 });
    }

    // Test the execution of BX, verifying the mode switch and PC update.
    #[test]
    fn test_execute_bx_switch_thumb() {
        let mut cpu = Cpu::new();
        // Set R0 to a target address with the LSB set (indicating Thumb mode).
        // For example, 0x08000001 means the branch target should be 0x08000000 with Thumb mode.
        cpu.cpu_state.set_register(0, 0x08000001);
        // Initialize the PC to a dummy starting address.
        cpu.cpu_state.set_register(15, 0x08000000);
        // Execute BX using register 0.
        cpu.branch_exchange(0);
        // After executing BX, the PC should be set to 0x08000000 (with the LSB cleared)
        // and the CPSR Thumb flag should be set.
        assert_eq!(cpu.cpu_state.get_register(15), 0x08000000);
        assert!(cpu.cpu_state.CPSR.is_thumb_state());
    }

    // Test the execution of BLX, verifying both the link and the branch.
    #[test]
    fn test_execute_blx_with_link() {
        let mut cpu = Cpu::new();
        // Set R1 to a target address with LSB set (to indicate Thumb mode).
        // This means the actual branch target should be 0x08001000.
        cpu.cpu_state.set_register(1, 0x08001001);
        // Initialize the PC to a known value.
        cpu.cpu_state.set_register(15, 0x08000000);
        // Execute BLX using register 1.
        cpu.branch_link_exchange(1);
        // Verify that the Link Register (R14) is set to the old PC value.
        assert_eq!(cpu.cpu_state.get_register(14), 0x08000000);
        // Verify that the PC is updated to the target address with LSB cleared (0x08001000).
        assert_eq!(cpu.cpu_state.get_register(15), 0x08001000);
        // Since the LSB was set, the Thumb mode flag should be on.
        assert!(cpu.cpu_state.CPSR.is_thumb_state());
    }
    // LDR Pre-Indexed, add mode with write-back.
    #[test]
    fn test_cpu_ldr_pre_index_add_writeback() {
        let mut memory = Memory::new(1024);
        let mut cpu = Cpu::new();

        // Set base register R1 = 100.
        cpu.cpu_state.set_register(1, 100);

        // Encode LDR: 0xE5B12008.
        // Instruction at address 0.
        memory.write_word(0, 0xE5B12008);
        // Place HALT at address 4.
        memory.write_word( 4, HALT);

        // At effective address: 100+8 = 108, store 0xDEADBEEF.
        memory.write_word( 108, 0xDEADBEEF);

        // Run program.
        cpu.run_program(&mut memory);

        // Expect R2 = 0xDEADBEEF, and R1 updated to 108.
        assert_eq!(cpu.cpu_state.get_register(2), 0xDEADBEEF);
        assert_eq!(cpu.cpu_state.get_register(1), 108);
    }

    // LDR Post-Indexed, add mode with no write-back.
    #[test]
    fn test_cpu_ldr_post_index_no_writeback() {
        let mut memory = Memory::new(1024);
        let mut cpu = Cpu::new();

        // Set base register R1 = 200.
        cpu.cpu_state.set_register(1, 200);

        // Encode LDR: 0xE4912010 (P=0, W=0, offset = 16).
        memory.write_word(0, 0xE4912010);
        memory.write_word(4, HALT);

        // In post-index, effective address = base (200). Place 0xCAFEBABE at address 200.
        memory.write_word( 200, 0xCAFEBABE);

        cpu.run_program(&mut memory);

        // Expect R2 = 0xCAFEBABE and R1 remains 200.
        assert_eq!(cpu.cpu_state.get_register(2), 0xCAFEBABE);
        assert_eq!(cpu.cpu_state.get_register(1), 200);
    }

    // LDM Pre-Indexed, add mode with write-back.
    #[test]
    fn test_cpu_ldm_pre_index_add_writeback() {
        let mut memory = Memory::new(1024);
        let mut cpu = Cpu::new();

        // Set base register R5 = 300.
        cpu.cpu_state.set_register(5, 300);

        // Encode LDM: 0xE9B50015.
        // This loads registers as indicated by bits in register_list = 0x0015 (R0, R2, R4).
        memory.write_word( 0, 0xE9B50015);
        memory.write_word(4, HALT);

        // Pre-index: effective starting address = 300 + 4 = 304.
        // Write values at sequential addresses:
        memory.write_word(304, 0x11111111); // for R0
        memory.write_word(308, 0x22222222); // for R2
        memory.write_word(312, 0x33333333); // for R4

        cpu.run_program(&mut memory);

        // Verify loaded registers.
        assert_eq!(cpu.cpu_state.get_register(0), 0x11111111);
        assert_eq!(cpu.cpu_state.get_register(2), 0x22222222);
        assert_eq!(cpu.cpu_state.get_register(4), 0x33333333);
        // Write-back updates base R5: 300 + (3 * 4) = 312.
        assert_eq!(cpu.cpu_state.get_register(5), 312);
    }

    //LDM Post-Indexed, subtract mode with no write-back.
    #[test]
    fn test_cpu_ldm_post_index_subtract_no_writeback() {
        let mut memory = Memory::new(1024);
        let mut cpu = Cpu::new();

        // Set base register R6 = 500.
        cpu.cpu_state.set_register(6, 500);

        // Encode LDM (load multiple): 0xE816002A.
        // This chooses post-index (P=0), subtract mode (U=0), no write-back (W=0),
        // and loads registers from register_list = 0x002A (R1, R3, R5).
        memory.write_word( 0, 0xE816002A);
        memory.write_word( 4, HALT);

        // In post-index mode, effective address is the original base (500).
        // Write memory for registers sequentially:
        memory.write_word( 500, 0xAAAAAAAA); // for R1
        memory.write_word(504, 0xBBBBBBBB); // for R3
        memory.write_word( 508, 0xCCCCCCCC); // for R5

        cpu.run_program(&mut memory);

        // Verify registers loaded.
        assert_eq!(cpu.cpu_state.get_register(1), 0xAAAAAAAA);
        assert_eq!(cpu.cpu_state.get_register(3), 0xBBBBBBBB);
        assert_eq!(cpu.cpu_state.get_register(5), 0xCCCCCCCC);
        // Base register R6 remains unchanged.
        assert_eq!(cpu.cpu_state.get_register(6), 500);
    }

    // LDRB Pre-Indexed, Add mode with write-back.
    #[test]
    fn test_ldrb_pre_index() {
        let mut memory = Memory::new(1024);
        let mut cpu = Cpu::new();

        // Set the base register R1 = 100.
        cpu.cpu_state.set_register(1, 100);

        // The LDRB instruction: 0xED703020 encodes:
        //   - Condition: 0xE,
        //   - Bits[27:26] = 01 (Single data transfer)
        //   - I = 0 (immediate offset)
        //   - P = 1 (pre-indexed)
        //   - U = 1 (add offset)
        //   - B = 1 (load byte)
        //   - W = 1 (write-back)
        //   - L = 1 (load)
        //   - Rn = 1 and Rt = 2, offset = 0x20.
        // Effective address = 100 + 0x20 = 132.
        memory.write_word(0, 0xED703020);
        memory.write_word(4, HALT);

        // At effective address 132, place a byte value (e.g., 0xAB).
        memory.write_bytes(132, &[0xAB]);

        cpu.run_program(&mut memory);

        // Verify: R2 should hold 0x000000AB (zero-extended)
        // and write-back updates R1 to 132.
        assert_eq!(cpu.cpu_state.get_register(2), 0xAB);
        assert_eq!(cpu.cpu_state.get_register(1), 132);
    }

    // LDRB Post-Indexed, Add mode with no write-back.
    #[test]
    fn test_ldrb_post_index() {
        let mut memory = Memory::new(1024);
        let mut cpu = Cpu::new();

        // Set the base register R1 = 200.
        cpu.cpu_state.set_register(1, 200);

        // LDRB post-index: same fields as before, but with:
        //   P = 0 (post-index) and W = 0 (no write-back).
        // Encoded instruction: 0xED512020.
        // Effective address = base = 200.
        memory.write_word(0, 0xED512020);
        memory.write_word(4, HALT);

        // Write a byte (e.g., 0xCD) at address 200.
        memory.write_bytes(200, &[0xCD]);

        cpu.run_program(&mut memory);

        // Verify: R2 should hold 0x000000CD and R1 remains 200.
        assert_eq!(cpu.cpu_state.get_register(2), 0xCD);
        assert_eq!(cpu.cpu_state.get_register(1), 200);
    }

    // LDRD Pre-Indexed, Add mode with write-back.
    #[test]
    fn test_ldrd_pre_index() {
        let mut memory = Memory::new(1024);
        let mut cpu = Cpu::new();

        // Set the base register R3 = 200.
        cpu.cpu_state.set_register(3, 200);

        // LDRD instruction: 0xED3340A0 encodes:
        //   - Condition: 0xE,
        //   - Bits[27:26] = 01,
        //   - I = 0, P = 1, U = 1, B = 0 (normal word load),
        //   - W = 1 (write-back), L = 1 (load),
        //   - Rn = 3, Rt = 4, offset = 0xA0.
        // Effective address = 200 + 0xA0 (160) = 360.
        memory.write_word(0, 0xED3340A0);
        memory.write_word(4, HALT);

        // Write the lower word at address 360 and the upper word at address 364.
        memory.write_word(360, 0xDEADBEEF);
        memory.write_word(364, 0xFEEDFACE);

        cpu.run_program(&mut memory);

        // Verify: R4 should be 0xDEADBEEF, R5 should be 0xFEEDFACE,
        // and write-back updates R3 to 360.
        assert_eq!(cpu.cpu_state.get_register(4), 0xDEADBEEF);
        assert_eq!(cpu.cpu_state.get_register(5), 0xFEEDFACE);
        assert_eq!(cpu.cpu_state.get_register(3), 360);
    }

    // Test 4: LDRD Post-Indexed, Subtract mode with no write-back.
    #[test]
    fn test_ldrd_post_index() {
        let mut memory = Memory::new(1024);
        let mut cpu = Cpu::new();

        // Set the base register R3 = 400.
        cpu.cpu_state.set_register(3, 400);

        // LDRD post-index: based on our previous LDRD encoding (0xED3340A0),
        // if we clear P (pre-index) and W (write-back) bits we get:
        //   0xED3340A0 - 0x01000000 (P) - 0x00200000 (W) = 0xEC1340A0.
        // Effective address = base = 400.
        memory.write_word(0, 0xEC1340A0);
        memory.write_word(4, HALT);

        // At address 400, write lower word and at 404, write upper word.
        memory.write_word(400, 0xAAAAAAAA);
        memory.write_word(404, 0xBBBBBBBB);

        cpu.run_program(&mut memory);

        // Verify: R4 should be 0xAAAAAAAA, R5 = 0xBBBBBBBB,
        // and since write-back is disabled, R3 remains 400.
        assert_eq!(cpu.cpu_state.get_register(4), 0xAAAAAAAA);
        assert_eq!(cpu.cpu_state.get_register(5), 0xBBBBBBBB);
        assert_eq!(cpu.cpu_state.get_register(3), 400);
    }

    #[test]
    fn test_decode_unknown() {
        let instruction = 0xFFFFFFFF; // Invalid instruction
        let decoded = decode_arm(instruction);
        assert_eq!(decoded, Instruction::Unknown(0xFFFFFFFF));
    }
}
