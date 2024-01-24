#[cfg(test)]
mod cmp_test {
    use crate::cpu::cpu::CPU;
    use crate::helpers::bitwise::split_word_into_bytes;

    #[test]
    fn cmp_test_immediate() {
        let mut cpu = CPU::new();
        let bytes = [
            0xC9, 0x20
        ];

        cpu.registers.acc = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn cmp_test_immediate_false() {
        let mut cpu = CPU::new();
        let bytes = [
            0xC9, 0x40
        ];

        cpu.registers.acc = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.negative, true);
    }

    #[test]
    fn cmp_test_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        cpu.memory[0x2233] = 0x20;
        let bytes = [
            0xCD, low_byte, high_byte
        ];

        cpu.registers.acc = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn cmp_test_x_indexed_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2230);
        cpu.memory[0x2233] = 0x20;
        let bytes = [
            0xDD, low_byte, high_byte
        ];

        cpu.registers.xr = 3;
        cpu.registers.acc = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn cmp_test_y_indexed_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2230);
        cpu.memory[0x2233] = 0x20;
        let bytes = [
            0xD9, low_byte, high_byte
        ];

        cpu.registers.yr = 3;
        cpu.registers.acc = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn cmp_test_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x33] = 0x20;
        let bytes = [
            0xC5, 0x33
        ];

        cpu.registers.acc = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn cmp_test_x_indexed_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x33] = 0x20;
        let bytes = [
            0xD5, 0x30
        ];

        cpu.registers.xr = 3;
        cpu.registers.acc = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn cmp_test_x_indexed_zero_page_indirect() {
        let mut cpu = CPU::new();
        cpu.memory[0x4030] = 0x20;
        //0x20 (0x30) + 0x21 (0x40) = 0x4030
        cpu.memory[0x20] = 0x30;
        cpu.memory[0x21] = 0x40;

        let bytes = [
            0xC1, 0x1D
        ];

        cpu.registers.xr = 3;
        cpu.registers.acc = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn cmp_test_zero_page_indirect_y_indexed() {
        let mut cpu = CPU::new();
        cpu.memory[0x4032] = 0x20;
        //0x20 (0x30) + 0x21 (0x40) = 0x4030
        cpu.memory[0x22] = 0x30;
        cpu.memory[0x23] = 0x40;

        let bytes = [
            0xD1, 0x22
        ];

        cpu.registers.yr = 2;
        cpu.registers.acc = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.negative, false);
    }
}