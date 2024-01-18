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
    Custom(u16),
    Implied,
    Immediate,
    Absolute,
    XIndexedAbsolute,
    YIndexedAbsolute,
    ZeroPage,
    XIndexedZeroPage,
    XIndexedZeroPageIndirect,
    ZeroPageIndirectYIndexed,
    Relative,
}

pub struct Instruction {
    pub(crate) address_mode: AddressingMode,
    pub(crate) operation: InstructionFn,
    pub(crate) result_handler: ResultHandlerFn
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
            AddressingMode::Custom(cycles) => {
                let result = (instruction.operation)(self, InstructionParameter::None);
                (instruction.result_handler)(self, result, None);
                self.cycles += cycles;
            }
            AddressingMode::Implied => {
                let result = (instruction.operation)(self, InstructionParameter::None);
                (instruction.result_handler)(self, result, None);
                self.cycles += 2;
            }
            AddressingMode::Immediate => {
                let value = self.get_next_byte(&mut bytes);
                let result = (instruction.operation)(self, InstructionParameter::Byte(value));
                (instruction.result_handler)(self, result, None);
                self.cycles += 2;
            }
            AddressingMode::Absolute => {
                let address = self.get_next_word(&mut bytes);
                let stored = *self.memory.get(address as usize).unwrap_or_else(|| {
                    panic!("Memory out of bounds")
                }) as u16;
                let result = (instruction.operation)(self, InstructionParameter::Word(stored));
                (instruction.result_handler)(self, result, None);
                self.cycles += 4;
            }
            AddressingMode::XIndexedAbsolute => {
                let address = self.get_next_word(&mut bytes);
                let (stored, final_address) = self.handle_x_indexed_absolute(address);
                let result = (instruction.operation)(self, InstructionParameter::Byte(stored));
                (instruction.result_handler)(self, result, Some(final_address));
            }
            AddressingMode::YIndexedAbsolute => {
                let address = self.get_next_word(&mut bytes);
                let (stored, final_address) = self.handle_y_indexed_absolute(address);
                let result = (instruction.operation)(self, InstructionParameter::Byte(stored));
                (instruction.result_handler)(self, result, Some(final_address));
            }
            AddressingMode::ZeroPage => {
                let value = self.get_next_byte(&mut bytes);
                let (stored, final_address) = self.handle_zero_paged(value);
                let result = (instruction.operation)(self, InstructionParameter::Byte(stored));
                (instruction.result_handler)(self, result, Some(final_address));
            }
            AddressingMode::XIndexedZeroPage => {
                let value = self.get_next_byte(&mut bytes);
                let (stored, final_address) = self.handle_x_indexed_zero_paged(value);
                let result = (instruction.operation)(self, InstructionParameter::Byte(stored));
                (instruction.result_handler)(self, result, Some(final_address));
            }
            AddressingMode::XIndexedZeroPageIndirect => {
                let value = self.get_next_byte(&mut bytes);
                let (stored, final_address) = self.handle_x_indexed_zero_paged_indirect(value);
                let result = (instruction.operation)(self, InstructionParameter::Byte(stored));
                (instruction.result_handler)(self, result, Some(final_address));
            }
            AddressingMode::ZeroPageIndirectYIndexed => {
                let value = self.get_next_byte(&mut bytes);
                let (stored, final_address) = self.handle_zero_paged_indirect_y_indexed(value);
                let result = (instruction.operation)(self, InstructionParameter::Byte(stored));
                (instruction.result_handler)(self, result, Some(final_address));
            }
            AddressingMode::Relative => {
                let value = self.get_next_byte(&mut bytes);
                self.handle_index_relative(value as i8);
                (instruction.operation)(self, InstructionParameter::None);
                (instruction.result_handler)(self, None, None);
            }
        }
    }

    pub fn handle_x_indexed_absolute(&mut self, address: u16) -> (u8, u16) {
        let (value, page_crossed, final_address) = self.index_absolute_indexed(address, self.registers.xr);
        self.cycles += 4 + (page_crossed as u16);

        (value, final_address)
    }

    pub fn handle_y_indexed_absolute(&mut self, address: u16) -> (u8, u16) {
        let (value, page_crossed, final_address) = self.index_absolute_indexed(address, self.registers.yr);
        self.cycles += 4 + (page_crossed as u16);

        (value, final_address)
    }

    pub fn handle_zero_paged(&mut self, address: u8) -> (u8, u16) {
        let value = self.index_zero_page(address);
        self.cycles += 3;

        (value, address as u16)
    }

    pub fn handle_x_indexed_zero_paged(&mut self, address: u8) -> (u8, u16) {
        let (value, final_address) = self.index_zero_page_indexed(address, self.registers.xr);
        self.cycles += 4;

        (value, final_address)
    }

    pub fn handle_x_indexed_zero_paged_indirect(&mut self, address: u8) -> (u8, u16) {
        let (value, final_address) = self.index_zero_paged_indexed_indirect(address, self.registers.xr);
        self.cycles += 6;

        (value, final_address)
    }

    pub fn handle_zero_paged_indirect_y_indexed(&mut self, address: u8) -> (u8, u16) {
        let (value, page_crossed, final_address) = self.index_zero_paged_indirect_indexed(address, self.registers.yr);
        self.cycles += 5 + (page_crossed as u16);

        (value, final_address)
    }

    pub fn handle_index_relative(&mut self, offset: i8){
        let page_crossed = self.index_relative(offset);
        self.cycles += 2 + (page_crossed as u16);
    }
}