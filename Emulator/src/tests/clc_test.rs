#[cfg(test)]
mod clc_test {
    use crate::cpu::cpu::CPU;

    #[test]
    fn clc_test_implied(){
        let mut cpu = CPU::new();
        let bytes = [
            0x18
        ];

        cpu.flags.carry = true;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.flags.carry, false);
    }

}