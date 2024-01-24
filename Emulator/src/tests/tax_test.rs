#[cfg(test)]
mod tax_test {
    use crate::cpu::cpu::CPU;

    #[test]
    fn tax_test_implied(){
        let mut cpu = CPU::new();
        let bytes = [
            0xAA
        ];

        cpu.registers.acc = 0x20;
        cpu.registers.xr = 0;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.xr, 0x20);
    }

}