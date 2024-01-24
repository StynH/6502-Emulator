#[cfg(test)]
mod dec_test {
    use crate::cpu::cpu::CPU;
    use crate::helpers::bitwise::split_word_into_bytes;

    #[test]
    fn dec_test_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        cpu.memory[0x2233] = 0x20;
        let bytes = [
            0xCE, low_byte, high_byte
        ];

        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x2233], 0x20 - 1);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn dec_test_absolute_negative() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        cpu.memory[0x2233] = 0x00;
        let bytes = [
            0xCE, low_byte, high_byte
        ];

        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x2233], 0xFF);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.negative, true);
    }

    #[test]
    fn dec_test_absolute_zero() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        cpu.memory[0x2233] = 0x01;
        let bytes = [
            0xCE, low_byte, high_byte
        ];

        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x2233], 0x01 - 1);
        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn dec_test_x_indexed_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2230);
        cpu.memory[0x2233] = 0x20;
        let bytes = [
            0xDE, low_byte, high_byte
        ];

        cpu.registers.xr = 3;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x2233], 0x20 - 1);
    }

    #[test]
    fn dec_test_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x33] = 0x20;
        let bytes = [
            0xC6, 0x33
        ];

        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x33], 0x20 - 1);
    }

    #[test]
    fn dec_test_x_indexed_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x33] = 0x20;
        let bytes = [
            0xD6, 0x30
        ];

        cpu.registers.xr = 3;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x33], 0x20 - 1);
    }
}