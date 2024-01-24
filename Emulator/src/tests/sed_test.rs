#[cfg(test)]
mod sed_test {
    use crate::cpu::cpu::CPU;

    #[test]
    fn sed_test_implied(){
        let mut cpu = CPU::new();
        let bytes = [
            0xF8
        ];

        cpu.flags.decimal = false;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.decimal, true);
    }

}