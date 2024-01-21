use crate::cpu::cpu::CPU;
use crate::helpers::addressing::page_crossed;
use crate::helpers::bitwise::merge_bytes_into_word;

pub enum InstructionParameter {
    None,
    Byte(u8),
    Word(u16),
}

pub enum ValueOrAddress{
    Value,
    Address
}

type InstructionFn = fn(&mut CPU, val: InstructionParameter) -> Option<u8>;

type ResultHandlerFn = fn(&mut CPU, val: Option<u8>, address: Option<u16>);

pub enum AddressingMode {
    Implied,
    Immediate,
    Accumulator,
    Absolute,
    AbsoluteIndirect,
    XIndexedAbsolute,
    YIndexedAbsolute,
    ZeroPage,
    XIndexedZeroPage,
    YIndexedZeroPage,
    XIndexedZeroPageIndirect,
    ZeroPageIndirectYIndexed,
    Relative,
}

pub struct Instruction {
    pub address_mode: AddressingMode,
    pub operation: InstructionFn,
    pub result_handler: ResultHandlerFn,
    pub value_or_address: ValueOrAddress,
    pub cycle_increase: u32,
    pub cycle_increases_on_page_cross: bool
}

impl CPU {

    pub fn get_next_byte(&mut self, bytes: &mut &[u8]) -> u8 {
        if bytes.is_empty() {
            panic!("Bytes array is empty.");
        }

        let byte = *bytes.get(self.registers.pc as usize).unwrap_or_else(|| {
            panic!("Memory out of bounds.")
        });
        self.registers.pc += 1;

        byte
    }

    pub fn get_next_word(&mut self, bytes: &mut &[u8]) -> u16 {
        let low_byte = self.get_next_byte(bytes) as u16;
        let high_byte = self.get_next_byte(bytes) as u16;
        (high_byte << 8) | low_byte
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction, mut bytes: &[u8]) {
        match instruction.address_mode {
            AddressingMode::Implied => {
                let result = (instruction.operation)(self, InstructionParameter::None);
                (instruction.result_handler)(self, result, None);

                self.cycles += instruction.cycle_increase
            }
            AddressingMode::Immediate => {
                let value = self.get_next_byte(&mut bytes);
                let result = (instruction.operation)(self, InstructionParameter::Byte(value));
                (instruction.result_handler)(self, result, None);

                self.cycles += instruction.cycle_increase
            }
            AddressingMode::Accumulator => {
                let value = self.registers.acc;
                let result = (instruction.operation)(self, InstructionParameter::Byte(value));
                (instruction.result_handler)(self, result, None);

                self.cycles += instruction.cycle_increase
            }
            AddressingMode::Absolute => {
                let address = self.get_next_word(&mut bytes);
                match instruction.value_or_address {
                    ValueOrAddress::Value => {
                        let stored = self.index_absolute(address);
                        let result = (instruction.operation)(self, InstructionParameter::Byte(stored));
                        (instruction.result_handler)(self, result, Some(address));
                    }
                    ValueOrAddress::Address => {
                        let result = (instruction.operation)(self, InstructionParameter::Word(address));
                        (instruction.result_handler)(self, result, Some(address));
                    }
                }

                self.cycles += instruction.cycle_increase
            }
            AddressingMode::AbsoluteIndirect => {
                let address = self.get_next_word(&mut bytes);
                let low_byte = *self.memory.get(address as usize).unwrap_or_else(|| {
                    panic!("Memory out of bounds")
                });
                let high_byte = *self.memory.get(address.wrapping_add(1) as usize).unwrap_or_else(|| {
                    panic!("Memory out of bounds")
                });
                let final_address = merge_bytes_into_word(low_byte, high_byte);
                let stored = *self.memory.get(final_address as usize).unwrap_or_else(|| {
                    panic!("Memory out of bounds")
                });

                self.handle_instruction(instruction, final_address, stored);
                self.cycles += instruction.cycle_increase
            }
            AddressingMode::XIndexedAbsolute => {
                let address = self.get_next_word(&mut bytes);
                let (stored, page_crossed, final_address) = self.index_absolute_indexed(address, self.registers.xr);
                self.handle_instruction(instruction, final_address, stored);
                self.cycles += instruction.cycle_increase + ((instruction.cycle_increases_on_page_cross && page_crossed) as u32);
            }
            AddressingMode::YIndexedAbsolute => {
                let address = self.get_next_word(&mut bytes);
                let (stored, page_crossed, final_address) = self.index_absolute_indexed(address, self.registers.yr);
                self.handle_instruction(instruction, final_address, stored);
                self.cycles += instruction.cycle_increase + ((instruction.cycle_increases_on_page_cross && page_crossed) as u32);
            }
            AddressingMode::ZeroPage => {
                let address = self.get_next_byte(&mut bytes);
                match instruction.value_or_address {
                    ValueOrAddress::Value => {
                        let stored = self.index_zero_page(address);
                        let result = (instruction.operation)(self, InstructionParameter::Byte(stored));
                        (instruction.result_handler)(self, result, Some(address as u16));
                    }
                    ValueOrAddress::Address => {
                        let result = (instruction.operation)(self, InstructionParameter::Word(address as u16));
                        (instruction.result_handler)(self, result, Some(address as u16));
                    }
                }

                self.cycles += instruction.cycle_increase;
            }
            AddressingMode::XIndexedZeroPage => {
                let address = self.get_next_byte(&mut bytes);
                let (stored, final_address) = self.index_zero_page_indexed(address, self.registers.xr);
                self.handle_instruction(instruction, final_address, stored);
                self.cycles += instruction.cycle_increase;
            }
            AddressingMode::YIndexedZeroPage => {
                let address = self.get_next_byte(&mut bytes);
                let (stored, final_address) = self.index_zero_page_indexed(address, self.registers.yr);
                self.handle_instruction(instruction, final_address, stored);
                self.cycles += instruction.cycle_increase;
            }
            AddressingMode::XIndexedZeroPageIndirect => {
                let address = self.get_next_byte(&mut bytes);
                let (stored, final_address) = self.index_zero_paged_indexed_indirect(address, self.registers.xr);
                self.handle_instruction(instruction, final_address, stored);
                self.cycles += instruction.cycle_increase;
            }
            AddressingMode::ZeroPageIndirectYIndexed => {
                let address = self.get_next_byte(&mut bytes);
                let (stored, page_crossed, final_address) = self.index_zero_paged_indirect_indexed(address, self.registers.yr);
                self.handle_instruction(instruction, final_address, stored);
                self.cycles += instruction.cycle_increase + ((instruction.cycle_increases_on_page_cross && page_crossed) as u32);
            }
            AddressingMode::Relative => {
                let value = self.get_next_byte(&mut bytes);
                (instruction.operation)(self, InstructionParameter::Word(value as u16));
                (instruction.result_handler)(self, None, None);

                self.cycles += instruction.cycle_increase + ((instruction.cycle_increases_on_page_cross && page_crossed(self.registers.pc, value as u16)) as u32);
            }
        }
    }

    pub fn handle_instruction(&mut self, instruction: &Instruction, address: u16, value: u8){
        match instruction.value_or_address {
            ValueOrAddress::Value => {
                let result = (instruction.operation)(self, InstructionParameter::Byte(value));
                (instruction.result_handler)(self, result, Some(address));
            }
            ValueOrAddress::Address => {
                let result = (instruction.operation)(self, InstructionParameter::Word(address));
                (instruction.result_handler)(self, result, Some(address));
            }
        }
    }
}