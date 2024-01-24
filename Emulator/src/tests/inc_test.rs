#[cfg(test)]
mod inc_test {
    use crate::cpu::cpu::CPU;
    use crate::helpers::bitwise::split_word_into_bytes;

    #[test]
    fn inc_test_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        cpu.memory[0x2233] = 0x20;
        let bytes = [
            0xEE, low_byte, high_byte
        ];

        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x2233], 0x20 + 1);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn inc_test_absolute_zero_negative() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        cpu.memory[0x2233] = 0xFF;
        let bytes = [
            0xEE, low_byte, high_byte
        ];

        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x2233], 0x00);
        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.negative, true);
    }

    #[test]
    fn inc_test_x_indexed_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2230);
        cpu.memory[0x2233] = 0x20;
        let bytes = [
            0xFE, low_byte, high_byte
        ];

        cpu.registers.xr = 3;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x2233], 0x20 + 1);
    }

    #[test]
    fn inc_test_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x33] = 0x20;
        let bytes = [
            0xE6, 0x33
        ];

        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x33], 0x20 + 1);
    }

    #[test]
    fn inc_test_x_indexed_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x33] = 0x20;
        let bytes = [
            0xF6, 0x30
        ];

        cpu.registers.xr = 3;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x33], 0x20 + 1);
    }
}