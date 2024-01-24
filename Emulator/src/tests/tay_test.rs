#[cfg(test)]
mod tay_test {
    use crate::cpu::cpu::CPU;

    #[test]
    fn tay_test_implied(){
        let mut cpu = CPU::new();
        let bytes = [
            0xA8
        ];

        cpu.registers.acc = 0x20;
        cpu.registers.yr = 0;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.yr, 0x20);
    }

}