#[cfg(test)]
mod tya_test {
    use crate::cpu::cpu::CPU;

    #[test]
    fn tya_test_implied(){
        let mut cpu = CPU::new();
        let bytes = [
            0x98
        ];

        cpu.registers.yr = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0x20);
    }

}