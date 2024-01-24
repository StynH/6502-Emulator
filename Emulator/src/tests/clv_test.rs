#[cfg(test)]
mod clv_test {
    use crate::cpu::cpu::CPU;

    #[test]
    fn clv_test_implied(){
        let mut cpu = CPU::new();
        let bytes = [
            0xB8
        ];

        cpu.flags.overflow = true;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.overflow, false);
    }

}