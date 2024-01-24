#[cfg(test)]
mod stx_test {
    use crate::cpu::cpu::CPU;
    use crate::helpers::bitwise::split_word_into_bytes;

    #[test]
    fn stx_test_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        cpu.memory[0x2233] = 0x00;
        let bytes = [
            0x8E, low_byte, high_byte
        ];

        cpu.registers.xr = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x2233], 0x20);
        assert_eq!(cpu.registers.xr, 0x00);
    }

    #[test]
    fn stx_test_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x33] = 0x00;
        let bytes = [
            0x86, 0x33
        ];

        cpu.registers.xr = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x33], 0x20);
        assert_eq!(cpu.registers.xr, 0x00);
    }

    #[test]
    fn stx_test_y_indexed_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x33] = 0x00;
        let bytes = [
            0x96, 0x30
        ];

        //Y + Zero Page (0x30 + 3 = 0x33)
        cpu.registers.yr = 0x03;
        cpu.registers.xr = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x33], 0x20);
        assert_eq!(cpu.registers.xr, 0x00);
    }
}