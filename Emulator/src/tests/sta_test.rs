#[cfg(test)]
mod sta_test {
    use crate::cpu::cpu::CPU;
    use crate::helpers::bitwise::split_word_into_bytes;

    #[test]
    fn sta_test_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        cpu.memory[0x2233] = 0x00;
        let bytes = [
            0x8D, low_byte, high_byte
        ];

        cpu.registers.acc = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x2233], 0x20);
        assert_eq!(cpu.registers.acc, 0x00);
    }

    #[test]
    fn sta_test_x_indexed_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2230);
        cpu.memory[0x2233] = 0x00;
        let bytes = [
            0x9D, low_byte, high_byte
        ];

        //X + ACC
        cpu.registers.xr = 3;
        cpu.registers.acc = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x2233], 0x20);
        assert_eq!(cpu.registers.acc, 0x00);
    }

    #[test]
    fn sta_test_y_indexed_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2230);
        cpu.memory[0x2233] = 0x00;
        let bytes = [
            0x99, low_byte, high_byte
        ];

        //Y + ACC
        cpu.registers.yr = 3;
        cpu.registers.acc = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x2233], 0x20);
        assert_eq!(cpu.registers.acc, 0x00);
    }

    #[test]
    fn sta_test_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x33] = 0x00;
        let bytes = [
            0x85, 0x33
        ];

        cpu.registers.acc = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x33], 0x20);
        assert_eq!(cpu.registers.acc, 0x00);
    }

    #[test]
    fn sta_test_x_indexed_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory[0x33] = 0x00;
        let bytes = [
            0x95, 0x30
        ];

        //X + Zero Page (0x30 + 3 = 0x33)
        cpu.registers.xr = 3;
        cpu.registers.acc = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x33], 0x20);
        assert_eq!(cpu.registers.acc, 0x00);
    }

    #[test]
    fn sta_test_x_indexed_zero_page_indirect() {
        let mut cpu = CPU::new();

        let bytes = [
            0x81, 0x1D
        ];

        cpu.registers.xr = 3;
        cpu.registers.acc = 0x20;
        cpu.memory[0x4030] = 0x00;
        cpu.memory[0x20] = 0x30;
        cpu.memory[0x21] = 0x40;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x4030], 0x20);
        assert_eq!(cpu.registers.acc, 0x00);
    }

    #[test]
    fn sta_test_zero_page_indirect_y_indexed() {
        let mut cpu = CPU::new();

        let bytes = [
            0x91, 0x22
        ];

        cpu.registers.yr = 2;
        cpu.registers.acc = 0x20;
        cpu.memory[0x4032] = 0x00;
        cpu.memory[0x22] = 0x30;
        cpu.memory[0x23] = 0x40;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.memory[0x4032], 0x20);
        assert_eq!(cpu.registers.acc, 0x00);
    }
}