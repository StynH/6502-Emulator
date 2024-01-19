use crate::cpu::cpu::CPU;
use crate::cpu::instructions::AddressingMode::Immediate;

pub enum InstructionParameter {
    None,
    Byte(u8),
    Word(u16),
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
                let stored = *self.memory.get(address as usize).unwrap_or_else(|| {
                    panic!("Memory out of bounds")
                }) as u16;
                let result = (instruction.operation)(self, InstructionParameter::Word(stored));
                (instruction.result_handler)(self, result, None);

                self.cycles += instruction.cycle_increase
            }
            AddressingMode::AbsoluteIndirect => {
                let address = self.get_next_word(&mut bytes);
                let final_address = *self.memory.get(address as usize).unwrap_or_else(|| {
                    panic!("Memory out of bounds")
                }) as u16;
                let stored = *self.memory.get(final_address as usize).unwrap_or_else(|| {
                    panic!("Memory out of bounds")
                });
                let result = (instruction.operation)(self, InstructionParameter::Byte(stored));
                (instruction.result_handler)(self, result, Some(final_address));

                self.cycles += instruction.cycle_increase
            }
            AddressingMode::XIndexedAbsolute => {
                let address = self.get_next_word(&mut bytes);
                let (stored, final_address) = self.handle_x_indexed_absolute(address, instruction.cycle_increase, instruction.cycle_increases_on_page_cross);
                let result = (instruction.operation)(self, InstructionParameter::Byte(stored));
                (instruction.result_handler)(self, result, Some(final_address));
            }
            AddressingMode::YIndexedAbsolute => {
                let address = self.get_next_word(&mut bytes);
                let (stored, final_address) = self.handle_y_indexed_absolute(address, instruction.cycle_increase, instruction.cycle_increases_on_page_cross);
                let result = (instruction.operation)(self, InstructionParameter::Byte(stored));
                (instruction.result_handler)(self, result, Some(final_address));
            }
            AddressingMode::ZeroPage => {
                let value = self.get_next_byte(&mut bytes);
                let (stored, final_address) = self.handle_zero_paged(value, instruction.cycle_increase);
                let result = (instruction.operation)(self, InstructionParameter::Byte(stored));
                (instruction.result_handler)(self, result, Some(final_address));
            }
            AddressingMode::XIndexedZeroPage => {
                let value = self.get_next_byte(&mut bytes);
                let (stored, final_address) = self.handle_x_indexed_zero_paged(value, instruction.cycle_increase);
                let result = (instruction.operation)(self, InstructionParameter::Byte(stored));
                (instruction.result_handler)(self, result, Some(final_address));
            }
            AddressingMode::YIndexedZeroPage => {
                let value = self.get_next_byte(&mut bytes);
                let (stored, final_address) = self.handle_y_indexed_zero_paged(value, instruction.cycle_increase);
                let result = (instruction.operation)(self, InstructionParameter::Byte(stored));
                (instruction.result_handler)(self, result, Some(final_address));
            }
            AddressingMode::XIndexedZeroPageIndirect => {
                let value = self.get_next_byte(&mut bytes);
                let (stored, final_address) = self.handle_x_indexed_zero_paged_indirect(value, instruction.cycle_increase);
                let result = (instruction.operation)(self, InstructionParameter::Byte(stored));
                (instruction.result_handler)(self, result, Some(final_address));
            }
            AddressingMode::ZeroPageIndirectYIndexed => {
                let value = self.get_next_byte(&mut bytes);
                let (stored, final_address) = self.handle_zero_paged_indirect_y_indexed(value, instruction.cycle_increase, instruction.cycle_increases_on_page_cross);
                let result = (instruction.operation)(self, InstructionParameter::Byte(stored));
                (instruction.result_handler)(self, result, Some(final_address));
            }
            AddressingMode::Relative => {
                let value = self.get_next_byte(&mut bytes);
                (instruction.operation)(self, InstructionParameter::Word(value as u16));
                (instruction.result_handler)(self, None, None);
            }
        }
    }

    pub fn handle_x_indexed_absolute(&mut self, address: u16, cycle_increase: u32, cycle_increase_on_page_cross: bool) -> (u8, u16) {
        let (value, page_crossed, final_address) = self.index_absolute_indexed(address, self.registers.xr);
        self.cycles += cycle_increase + ((cycle_increase_on_page_cross && page_crossed) as u32);

        (value, final_address)
    }

    pub fn handle_y_indexed_absolute(&mut self, address: u16, cycle_increase: u32, cycle_increase_on_page_cross: bool) -> (u8, u16) {
        let (value, page_crossed, final_address) = self.index_absolute_indexed(address, self.registers.yr);
        self.cycles += cycle_increase + ((cycle_increase_on_page_cross && page_crossed) as u32);

        (value, final_address)
    }

    pub fn handle_zero_paged(&mut self, address: u8, cycle_increase: u32) -> (u8, u16) {
        let value = self.index_zero_page(address);
        self.cycles += cycle_increase;

        (value, address as u16)
    }

    pub fn handle_x_indexed_zero_paged(&mut self, address: u8, cycle_increase: u32) -> (u8, u16) {
        let (value, final_address) = self.index_zero_page_indexed(address, self.registers.xr);
        self.cycles += cycle_increase;

        (value, final_address)
    }

    pub fn handle_y_indexed_zero_paged(&mut self, address: u8, cycle_increase: u32) -> (u8, u16) {
        let (value, final_address) = self.index_zero_page_indexed(address, self.registers.yr);
        self.cycles += cycle_increase;

        (value, final_address)
    }

    pub fn handle_x_indexed_zero_paged_indirect(&mut self, address: u8, cycle_increase: u32) -> (u8, u16) {
        let (value, final_address) = self.index_zero_paged_indexed_indirect(address, self.registers.xr);
        self.cycles += cycle_increase;

        (value, final_address)
    }

    pub fn handle_zero_paged_indirect_y_indexed(&mut self, address: u8, cycle_increase: u32, cycle_increase_on_page_cross: bool) -> (u8, u16) {
        let (value, page_crossed, final_address) = self.index_zero_paged_indirect_indexed(address, self.registers.yr);
        self.cycles += cycle_increase + ((cycle_increase_on_page_cross && page_crossed) as u32);

        (value, final_address)
    }
}