#[cfg(test)]
mod bmi_test {
    use crate::cpu::cpu::CPU;
    use crate::helpers::bitwise::split_word_into_bytes;

    #[test]
    fn bmi_test_relative(){
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        let bytes = [
            0x30, low_byte, high_byte
        ];

        cpu.flags.negative = true;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.pc, 0x2233 + 0x03);
    }

    #[test]
    fn bmi_test_relative_false(){
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x2233);
        let bytes = [
            0x30, low_byte, high_byte
        ];

        cpu.flags.negative = false;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.pc, 0x03);
    }
}