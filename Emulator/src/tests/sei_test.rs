#[cfg(test)]
mod sei_test {
    use crate::cpu::cpu::CPU;

    #[test]
    fn sei_test_implied(){
        let mut cpu = CPU::new();
        let bytes = [
            0x78
        ];

        cpu.flags.interrupt = false;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.interrupt, true);
    }

}