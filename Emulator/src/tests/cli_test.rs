#[cfg(test)]
mod cli_test {
    use crate::cpu::cpu::CPU;

    #[test]
    fn cli_test_implied(){
        let mut cpu = CPU::new();
        let bytes = [
            0x58
        ];

        cpu.flags.interrupt = true;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.interrupt, false);
    }

}