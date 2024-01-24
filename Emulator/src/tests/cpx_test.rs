#[cfg(test)]
mod cpx_test {
    use crate::cpu::cpu::CPU;
    use crate::helpers::bitwise::split_word_into_bytes;

    #[test]
    fn cpx_test_immediate() {
        let mut cpu = CPU::new();
        let bytes = [
            0xE0, 0x20
        ];

        cpu.registers.xr = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn cpx_test_immediate_false() {
        let mut cpu = CPU::new();
        let bytes = [
            0xE0, 0x40
        ];

        cpu.registers.xr = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.negative, true);
    }

    #[test]
    fn cpx_test_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        cpu.memory[0x2233] = 0x20;
        let bytes = [
            0xEC, low_byte, high_byte
        ];

        cpu.registers.xr = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn cpx_test_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x33] = 0x20;
        let bytes = [
            0xE4, 0x33
        ];

        cpu.registers.xr = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.negative, false);
    }
}