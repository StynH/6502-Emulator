#[cfg(test)]
mod asl_test {
    use crate::cpu::cpu::CPU;
    use crate::helpers::bitwise::split_word_into_bytes;

    #[test]
    fn asl_test_accumulator() {
        let mut cpu = CPU::new();
        let bytes = [
            0x0A
        ];

        cpu.registers.acc = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0x20 << 1);
    }

    #[test]
    fn asl_test_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        cpu.memory[0x2233] = 0x20;
        let bytes = [
            0x0E, low_byte, high_byte
        ];

        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x2233], 0x20 << 1);
    }

    #[test]
    fn asl_test_x_indexed_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        cpu.memory[0x2236] = 0x20;
        let bytes = [
            0x1E, low_byte, high_byte
        ];

        cpu.registers.xr = 0x03;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x2236], 0x20 << 1);
    }

    #[test]
    fn asl_test_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x33] = 0x20;
        let bytes = [
            0x06, 0x33
        ];

        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x33], 0x20 << 1);
    }

    #[test]
    fn asl_test_x_indexed_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x36] = 0x20;
        let bytes = [
            0x16, 0x33
        ];

        cpu.registers.xr = 0x03;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x36], 0x20 << 1);
    }
}