#[cfg(test)]
mod txs_test {
    use crate::cpu::cpu::CPU;

    #[test]
    fn txs_test_implied(){
        let mut cpu = CPU::new();
        let bytes = [
            0x9A
        ];

        cpu.registers.xr = 0x20;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.xr, 0x00);
        assert_eq!(cpu.registers.sp, 0x20);
    }

}