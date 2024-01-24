#[cfg(test)]
mod cld_test {
    use crate::cpu::cpu::CPU;

    #[test]
    fn cld_test_implied(){
        let mut cpu = CPU::new();
        let bytes = [
            0xD8
        ];

        cpu.flags.decimal = true;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.decimal, false);
    }

}