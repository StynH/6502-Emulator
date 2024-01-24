#[cfg(test)]
mod sec_test {
    use crate::cpu::cpu::CPU;

    #[test]
    fn sec_test_implied(){
        let mut cpu = CPU::new();
        let bytes = [
            0x38
        ];

        cpu.flags.carry = false;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.carry, true);
    }

}