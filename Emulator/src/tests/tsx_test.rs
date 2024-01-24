#[cfg(test)]
mod tsx_test {
    use crate::cpu::cpu::CPU;

    #[test]
    fn tsx_test_implied(){
        let mut cpu = CPU::new();
        let bytes = [
            0xBA
        ];

        cpu.registers.xr = 0;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.xr, cpu.registers.sp);
    }

}