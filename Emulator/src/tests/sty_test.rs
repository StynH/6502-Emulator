#[cfg(test)]
mod sty_test {
    use crate::cpu::cpu::CPU;
    use crate::helpers::bitwise::split_word_into_bytes;

    #[test]
    fn sty_test_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        cpu.memory[0x2233] = 0x00;
        let bytes = [
            0x8C, low_byte, high_byte
        ];

        cpu.registers.yr = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x2233], 0x20);
        assert_eq!(cpu.registers.yr, 0x00);
    }

    #[test]
    fn sty_test_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x33] = 0x00;
        let bytes = [
            0x84, 0x33
        ];

        cpu.registers.yr = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x33], 0x20);
        assert_eq!(cpu.registers.yr, 0x00);
    }

    #[test]
    fn sty_test_x_indexed_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x33] = 0x00;
        let bytes = [
            0x94, 0x30
        ];

        //X + Zero Page (0x30 + 3 = 0x33)
        cpu.registers.xr = 0x03;
        cpu.registers.yr = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x33], 0x20);
        assert_eq!(cpu.registers.yr, 0x00);
    }
}