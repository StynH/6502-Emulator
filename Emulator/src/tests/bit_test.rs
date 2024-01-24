#[cfg(test)]
mod bit_test {
    use crate::cpu::cpu::CPU;
    use crate::helpers::bitwise::split_word_into_bytes;

    #[test]
    fn bit_test_absolute() {
        let mut cpu = CPU::new();

        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        let bytes = [
            0x2C, low_byte, high_byte
        ];

        cpu.memory[0x2233] = 0xC0;
        cpu.registers.acc = 0xFF;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.zero, cpu.registers.acc & cpu.memory[0x2233] == 0);
        assert_eq!(cpu.flags.overflow, (cpu.memory[0x2233] & 0x40) != 0);
        assert_eq!(cpu.flags.negative, (cpu.memory[0x2233] & 0x80) != 0);
    }

    #[test]
    fn bit_test_zero_page() {
        let mut cpu = CPU::new();

        let bytes = [
            0x24, 0x33
        ];

        cpu.memory[0x33] = 0xC0;
        cpu.registers.acc = 0xFF;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.zero, cpu.registers.acc & cpu.memory[0x33] == 0);
        assert_eq!(cpu.flags.overflow, (cpu.memory[0x33] & 0x40) != 0);
        assert_eq!(cpu.flags.negative, (cpu.memory[0x33] & 0x80) != 0);
    }
}