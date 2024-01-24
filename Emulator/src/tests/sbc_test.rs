#[cfg(test)]
mod sbc_test {
    use crate::cpu::cpu::CPU;
    use crate::helpers::bitwise::split_word_into_bytes;

    #[test]
    fn sbc_test_immediate() {
        let mut cpu = CPU::new();
        let bytes = [
            0xE9, 0x20
        ];

        cpu.registers.acc = 0x30;
        cpu.flags.carry = true;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0x10);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn sbc_test_immediate_zero() {
        let mut cpu = CPU::new();
        let bytes = [
            0xE9, 0x01
        ];

        cpu.registers.acc = 0x01;
        cpu.flags.carry = true;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0x00);
        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn sbc_test_immediate_negative() {
        let mut cpu = CPU::new();
        let bytes = [
            0xE9, 0x02
        ];

        cpu.registers.acc = 0x01;
        cpu.flags.carry = true;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0xFF);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.negative, true);
    }

    #[test]
    fn sbc_test_immediate_no_carry() {
        let mut cpu = CPU::new();
        let bytes = [
            0xE9, 0x20
        ];

        cpu.registers.acc = 0x30;
        cpu.flags.carry = false;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0x0F);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn sbc_test_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        cpu.memory[0x2233] = 0x20;
        let bytes = [
            0xED, low_byte, high_byte
        ];

        cpu.registers.acc = 0x30;
        cpu.flags.carry = true;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0x10);
    }

    #[test]
    fn sbc_test_x_indexed_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2230);
        cpu.memory[0x2233] = 0x20;
        let bytes = [
            0xFD, low_byte, high_byte
        ];

        cpu.registers.xr = 3;
        cpu.registers.acc = 0x30;
        cpu.flags.carry = true;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0x10);
    }

    #[test]
    fn sbc_test_y_indexed_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2230);
        cpu.memory[0x2233] = 0x20;
        let bytes = [
            0xF9, low_byte, high_byte
        ];

        //Y + ACC
        cpu.registers.yr = 3;
        cpu.registers.acc = 0x30;
        cpu.flags.carry = true;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0x10);
    }

    #[test]
    fn sbc_test_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x33] = 0x20;
        let bytes = [
            0xE5, 0x33
        ];

        cpu.registers.acc = 0x30;
        cpu.flags.carry = true;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0x10);
    }

    #[test]
    fn sbc_test_x_indexed_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x33] = 0x20;
        let bytes = [
            0xF5, 0x30
        ];

        cpu.registers.xr = 3;
        cpu.registers.acc = 0x30;
        cpu.flags.carry = true;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0x10);
    }

    #[test]
    fn sbc_test_x_indexed_zero_page_indirect() {
        let mut cpu = CPU::new();

        let bytes = [
            0xE1, 0x1D
        ];

        cpu.registers.xr = 0x03;
        cpu.registers.acc = 0x30;
        cpu.memory[0x20] = 0x30;
        cpu.memory[0x21] = 0x40;
        cpu.memory[0x4030] = 0x20;
        cpu.flags.carry = true;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0x10);
    }

    #[test]
    fn sbc_test_zero_page_indirect_y_indexed() {
        let mut cpu = CPU::new();

        let bytes = [
            0xF1, 0x22
        ];

        cpu.registers.yr = 0x02;
        cpu.registers.acc = 0x30;
        cpu.memory[0x4032] = 0x20;
        cpu.memory[0x22] = 0x30;
        cpu.memory[0x23] = 0x40;
        cpu.flags.carry = true;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0x10);
    }
}