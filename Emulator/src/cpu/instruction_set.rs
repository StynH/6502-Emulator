use std::collections::HashMap;
use crate::cpu::cpu::CPU;
use crate::cpu::instructions::{AddressingMode, Instruction};

impl CPU{

    pub fn get_instruction_set(&self) -> HashMap<u8, Instruction>{
        HashMap::from([
            //ADC
            (0x69, Instruction{
                address_mode: AddressingMode::Immediate,
                operation: CPU::op_adc,
                result_handler: CPU::result_into_acc,
            }),
            (0x6D, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_adc,
                result_handler: CPU::result_into_acc,
            }),
            (0x7D, Instruction{
                address_mode: AddressingMode::XIndexedAbsolute,
                operation: CPU::op_adc,
                result_handler: CPU::result_into_acc,
            }),
            (0x79, Instruction{
                address_mode: AddressingMode::YIndexedAbsolute,
                operation: CPU::op_adc,
                result_handler: CPU::result_into_acc,
            }),
            (0x65, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_adc,
                result_handler: CPU::result_into_acc,
            }),
            (0x75, Instruction{
                address_mode: AddressingMode::XIndexedZeroPage,
                operation: CPU::op_adc,
                result_handler: CPU::result_into_acc,
            }),
            (0x61, Instruction{
                address_mode: AddressingMode::XIndexedZeroPageIndirect,
                operation: CPU::op_adc,
                result_handler: CPU::result_into_acc,
            }),
            (0x71, Instruction{
                address_mode: AddressingMode::ZeroPageIndirectYIndexed,
                operation: CPU::op_adc,
                result_handler: CPU::result_into_acc,
            }),

            //AND
            (0x29, Instruction{
                address_mode: AddressingMode::Immediate,
                operation: CPU::op_and,
                result_handler: CPU::result_into_acc,
            }),
            (0x2D, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_and,
                result_handler: CPU::result_into_acc,
            }),
            (0x3D, Instruction{
                address_mode: AddressingMode::XIndexedAbsolute,
                operation: CPU::op_and,
                result_handler: CPU::result_into_acc,
            }),
            (0x39, Instruction{
                address_mode: AddressingMode::YIndexedAbsolute,
                operation: CPU::op_and,
                result_handler: CPU::result_into_acc,
            }),
            (0x25, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_and,
                result_handler: CPU::result_into_acc,
            }),
            (0x35, Instruction{
                address_mode: AddressingMode::XIndexedZeroPage,
                operation: CPU::op_and,
                result_handler: CPU::result_into_acc,
            }),
            (0x21, Instruction{
                address_mode: AddressingMode::XIndexedZeroPageIndirect,
                operation: CPU::op_and,
                result_handler: CPU::result_into_acc,
            }),
            (0x31, Instruction{
                address_mode: AddressingMode::ZeroPageIndirectYIndexed,
                operation: CPU::op_and,
                result_handler: CPU::result_into_acc,
            }),

            //LDA
            (0xA9, Instruction{
                address_mode: AddressingMode::Immediate,
                operation: CPU::op_lda,
                result_handler: CPU::no_handler,
            }),
            (0xAD, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_lda,
                result_handler: CPU::no_handler,
            }),
            (0xBD, Instruction{
                address_mode: AddressingMode::XIndexedAbsolute,
                operation: CPU::op_lda,
                result_handler: CPU::no_handler,
            }),
            (0xB9, Instruction{
                address_mode: AddressingMode::YIndexedAbsolute,
                operation: CPU::op_lda,
                result_handler: CPU::no_handler,
            }),
            (0xA5, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_lda,
                result_handler: CPU::no_handler,
            }),
            (0xB5, Instruction{
                address_mode: AddressingMode::XIndexedZeroPage,
                operation: CPU::op_lda,
                result_handler: CPU::no_handler,
            }),
            (0xA1, Instruction{
                address_mode: AddressingMode::XIndexedZeroPageIndirect,
                operation: CPU::op_lda,
                result_handler: CPU::no_handler,
            }),
            (0xB1, Instruction{
                address_mode: AddressingMode::ZeroPageIndirectYIndexed,
                operation: CPU::op_lda,
                result_handler: CPU::no_handler,
            }),

            //BMI
            (0x30, Instruction{
                address_mode: AddressingMode::Relative,
                operation: CPU::op_bmi,
                result_handler: CPU::no_handler,
            }),

            //BNE
            (0xD0, Instruction{
                address_mode: AddressingMode::Relative,
                operation: CPU::op_bne,
                result_handler: CPU::no_handler,
            }),

            //BRK
            (0x00, Instruction{
                address_mode: AddressingMode::Custom(7),
                operation: CPU::op_bpl,
                result_handler: CPU::no_handler,
            }),

            //BPL
            (0x10, Instruction{
                address_mode: AddressingMode::Relative,
                operation: CPU::op_bpl,
                result_handler: CPU::no_handler,
            }),

            //CLC,
            (0x18, Instruction{
                address_mode: AddressingMode::Implied,
                operation: CPU::op_clc,
                result_handler: CPU::no_handler,
            }),

            //CLD
            (0xD8, Instruction{
                address_mode: AddressingMode::Implied,
                operation: CPU::op_cld,
                result_handler: CPU::no_handler,
            }),

            //CLV
            (0xB8, Instruction{
                address_mode: AddressingMode::Implied,
                operation: CPU::op_clv,
                result_handler: CPU::no_handler,
            }),

            //CMP
            (0xC9, Instruction{
                address_mode: AddressingMode::Immediate,
                operation: CPU::op_cmp,
                result_handler: CPU::no_handler,
            }),
            (0xCD, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_cmp,
                result_handler: CPU::no_handler,
            }),
            (0xDD, Instruction{
                address_mode: AddressingMode::XIndexedAbsolute,
                operation: CPU::op_cmp,
                result_handler: CPU::no_handler,
            }),
            (0xD9, Instruction{
                address_mode: AddressingMode::YIndexedAbsolute,
                operation: CPU::op_cmp,
                result_handler: CPU::no_handler,
            }),
            (0xC5, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_cmp,
                result_handler: CPU::no_handler,
            }),
            (0xD5, Instruction{
                address_mode: AddressingMode::XIndexedZeroPage,
                operation: CPU::op_cmp,
                result_handler: CPU::no_handler,
            }),
            (0xC1, Instruction{
                address_mode: AddressingMode::XIndexedZeroPageIndirect,
                operation: CPU::op_cmp,
                result_handler: CPU::no_handler,
            }),
            (0xD1, Instruction{
                address_mode: AddressingMode::ZeroPageIndirectYIndexed,
                operation: CPU::op_cmp,
                result_handler: CPU::no_handler,
            }),
        ])
    }

    fn result_into_acc(&mut self, value: Option<u8>, _: Option<u16>){
        self.registers.acc = value.unwrap_or_else(|| {
            panic!("Expected value for loading into ACC.");
        })
    }

    fn result_into_memory(&mut self, value: Option<u8>, address: Option<u16>){
        match address {
            Some(result_address) => {
                if result_address >= self.memory.len() as u16 {
                    panic!("Memory out of bounds when writing result to memory.")
                }
                self.memory[result_address as usize] = value.unwrap_or_else(|| {
                    panic!("Expected value when writing result to memory.")
                });
            }
            None => {
                panic!("Expected memory address when writing result to memory.")
            }
        }
    }

    fn no_handler(&mut self, _: Option<u8>, _: Option<u16>){
        //Empty
    }
}