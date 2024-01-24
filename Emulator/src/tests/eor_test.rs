#[cfg(test)]
mod eor_test {
    use crate::cpu::cpu::CPU;
    use crate::helpers::bitwise::split_word_into_bytes;

    #[test]
    fn eor_test_immediate() {
        let mut cpu = CPU::new();

        let bytes = [
            0x49, 0x55
        ];

        cpu.registers.acc = 0xFF;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0xFF ^ 0x55);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.negative, true);
    }

    #[test]
    fn eor_test_immediate_zero() {
        let mut cpu = CPU::new();

        let bytes = [
            0x49, 0xFF
        ];

        cpu.registers.acc = 0xFF;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0xFF ^ 0xFF);
        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn eor_test_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        cpu.memory[0x2233] = 0x55;

        let bytes = [
            0x4D, low_byte, high_byte
        ];

        cpu.registers.acc = 0xFF;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0xFF ^ 0x55);
    }

    #[test]
    fn eor_test_x_indexed_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2230);
        cpu.memory[0x2233] = 0x55;

        let bytes = [
            0x5D, low_byte, high_byte
        ];

        cpu.registers.acc = 0xFF;
        cpu.registers.xr = 0x03;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0xFF ^ 0x55);
    }

    #[test]
    fn eor_test_y_indexed_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2230);
        cpu.memory[0x2233] = 0x55;

        let bytes = [
            0x59, low_byte, high_byte
        ];

        cpu.registers.acc = 0xFF;
        cpu.registers.yr = 0x03;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0xFF ^ 0x55);
    }

    #[test]
    fn eor_test_zero_page() {
        let mut cpu = CPU::new();

        let bytes = [
            0x45, 0x33
        ];

        cpu.registers.acc = 0xFF;
        cpu.memory[0x33] = 0x55;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0xFF ^ 0x55);
    }

    #[test]
    fn eor_test_x_indexed_zero_page() {
        let mut cpu = CPU::new();

        let bytes = [
            0x55, 0x30
        ];

        cpu.registers.acc = 0xFF;
        cpu.registers.xr = 0x03;
        cpu.memory[0x33] = 0x55;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0xFF ^ 0x55);
    }

    #[test]
    fn eor_test_x_indexed_zero_page_indirect() {
        let mut cpu = CPU::new();

        let bytes = [
            0x41, 0x1D
        ];

        cpu.registers.acc = 0xFF;
        cpu.registers.xr = 0x03;
        cpu.memory[0x20] = 0x30;
        cpu.memory[0x21] = 0x40;
        cpu.memory[0x4030] = 0x55;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0xFF ^ 0x55);
    }

    #[test]
    fn eor_test_zero_page_indirect_y_indexed() {
        let mut cpu = CPU::new();

        let bytes = [
            0x51, 0x22
        ];

        cpu.registers.acc = 0xFF;
        cpu.registers.yr = 0x02;

        cpu.memory[0x22] = 0x30;
        cpu.memory[0x23] = 0x40;
        cpu.memory[0x4032] = 0x55;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0xFF ^ 0x55);
    }
}