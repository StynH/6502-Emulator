#[cfg(test)]
mod inx_test {
    use crate::cpu::cpu::CPU;

    #[test]
    fn inx_test_implied() {
        let mut cpu = CPU::new();
        let bytes = [
            0xE8
        ];

        cpu.registers.xr = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.xr, 0x20 + 1);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.negative, false);
    }
}