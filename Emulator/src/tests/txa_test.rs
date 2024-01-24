#[cfg(test)]
mod txa_test {
    use crate::cpu::cpu::CPU;

    #[test]
    fn txa_test_implied(){
        let mut cpu = CPU::new();
        let bytes = [
            0x8A
        ];

        cpu.registers.xr = 0x20;
        cpu.registers.acc = 0;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.acc, 0x20);
    }

}