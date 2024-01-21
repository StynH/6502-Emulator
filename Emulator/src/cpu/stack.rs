use crate::cpu::cpu::CPU;
use crate::helpers::bitwise::{merge_bytes_into_word, split_word_into_bytes};

impl CPU{
    pub fn convert_address_to_stack(address: u8) -> usize {
        0x100 + (address as u16) as usize
    }

    pub fn push_byte_to_stack(&mut self, value: u8){
        let address = CPU::convert_address_to_stack(self.registers.sp);
        self.memory[address] = value;
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    pub fn push_word_to_stack(&mut self, value: u16){
        let (high_byte, low_byte) = split_word_into_bytes(value);
        self.push_byte_to_stack(low_byte);
        self.push_byte_to_stack(high_byte);
    }

    pub fn pop_byte_from_stack(&mut self) -> Option<u8> {
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let value = self.memory[CPU::convert_address_to_stack(self.registers.sp)];
        Some(value)
    }

    pub fn pop_word_from_stack(&mut self) -> Option<u16>{
        if self.registers.sp == 0xFF || self.registers.sp == 0xFE {
            None
        }
        else{
            self.registers.sp = self.registers.sp.wrapping_add(1);
            let low_byte = self.memory[CPU::convert_address_to_stack(self.registers.sp)];

            self.registers.sp = self.registers.sp.wrapping_add(1);
            let high_byte = self.memory[CPU::convert_address_to_stack(self.registers.sp)];

            Some(merge_bytes_into_word(high_byte, low_byte))
        }
    }
}