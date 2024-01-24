#[cfg(test)]
mod jmp_test {
    use crate::cpu::cpu::CPU;
    use crate::helpers::bitwise::split_word_into_bytes;

    #[test]
    fn jmp_test_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x0005);
        let bytes = [
            0x4C, low_byte, high_byte, 0xE8, 0xE8, 0xC8
        ];

        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.xr, 0x00); //INX (0xE8) should be jumped over.
        assert_eq!(cpu.registers.yr, 0x01);
    }

    #[test]
    fn jmp_test_absolute_indirect() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        cpu.memory[0x2233] = 0x0005;
        let bytes = [
            0x6C, low_byte, high_byte, 0xE8, 0xE8, 0xC8
        ];

        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.xr, 0x00); //INX (0xE8) should be jumped over.
        assert_eq!(cpu.registers.yr, 0x01);
    }
}