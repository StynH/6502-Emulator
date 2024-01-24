#[cfg(test)]
mod brk_test {
    use crate::cpu::cpu::{CPU, Flags};

    #[test]
    fn brk_test() {
        let mut cpu = CPU::new();
        let bytes = [0x00];

        cpu.set_interrupt_vector(0x12, 0x34);
        cpu.execute_instruction_sequence(&mut bytes.as_slice());

        let stored_status = cpu.pop_byte_from_stack().unwrap();
        let stored_flags = Flags::from_byte(stored_status);

        let expected_return_address = 0x03;
        let stored_return_address = cpu.pop_word_from_stack().unwrap();

        assert_eq!(cpu.registers.pc, 0x1234);
        assert_eq!(stored_return_address, expected_return_address);
        assert_eq!(cpu.flags.interrupt, true);
        assert_eq!(stored_flags.interrupt, false);
        assert_eq!(cpu.flags.negative, stored_flags.negative);
        assert_eq!(cpu.flags.overflow, stored_flags.overflow);
        assert_eq!(cpu.flags.brk, stored_flags.brk);
        assert_eq!(cpu.flags.decimal, stored_flags.decimal);
        assert_eq!(cpu.flags.zero, stored_flags.zero);
        assert_eq!(cpu.flags.carry, stored_flags.carry);
    }
}