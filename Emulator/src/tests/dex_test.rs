#[cfg(test)]
mod dex_test {
    use crate::cpu::cpu::CPU;

    #[test]
    fn dex_test_implied() {
        let mut cpu = CPU::new();
        let bytes = [
            0xCA
        ];

        cpu.registers.xr = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.xr, 0x20 - 1);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn dex_test_implied_negative() {
        let mut cpu = CPU::new();
        let bytes = [
            0xCA
        ];

        cpu.registers.xr = 0x00;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.xr, 0xFF);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.negative, true);
    }

    #[test]
    fn dex_test_implied_zero() {
        let mut cpu = CPU::new();
        let bytes = [
            0xCA
        ];

        cpu.registers.xr = 0x01;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.xr, 0x01 - 1);
        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.negative, false);
    }
}