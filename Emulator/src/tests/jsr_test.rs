#[cfg(test)]
mod jsr_test {
    use crate::cpu::cpu::CPU;
    use crate::helpers::bitwise::{split_word_into_bytes};

    #[test]
    fn jsr_test_absolute() {
        let mut cpu = CPU::new();
        let (high_byte, low_byte) = split_word_into_bytes(0x0000);

        let bytes = [
            0xE8, 0x60, //Subroutine INX and RTS
            0xC8, //Filler, should be skipped
            0x20, low_byte, high_byte, 0xE8 //JSR
        ];

        cpu.registers.pc = 3;
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        assert_eq!(cpu.registers.xr, 0x02); //Executed once in the subroutine, and once at the return.
        assert_eq!(cpu.registers.yr, 0x00); //Should be skipped
    }
}