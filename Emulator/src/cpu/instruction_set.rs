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
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),
            (0x6D, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_adc,
                result_handler: CPU::result_into_acc,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0x7D, Instruction{
                address_mode: AddressingMode::XIndexedAbsolute,
                operation: CPU::op_adc,
                result_handler: CPU::result_into_acc,
                cycle_increase: 4,
                cycle_increases_on_page_cross: true
            }),
            (0x79, Instruction{
                address_mode: AddressingMode::YIndexedAbsolute,
                operation: CPU::op_adc,
                result_handler: CPU::result_into_acc,
                cycle_increase: 4,
                cycle_increases_on_page_cross: true
            }),
            (0x65, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_adc,
                result_handler: CPU::result_into_acc,
                cycle_increase: 3,
                cycle_increases_on_page_cross: false
            }),
            (0x75, Instruction{
                address_mode: AddressingMode::XIndexedZeroPage,
                operation: CPU::op_adc,
                result_handler: CPU::result_into_acc,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0x61, Instruction{
                address_mode: AddressingMode::XIndexedZeroPageIndirect,
                operation: CPU::op_adc,
                result_handler: CPU::result_into_acc,
                cycle_increase: 6,
                cycle_increases_on_page_cross: false
            }),
            (0x71, Instruction{
                address_mode: AddressingMode::ZeroPageIndirectYIndexed,
                operation: CPU::op_adc,
                result_handler: CPU::result_into_acc,
                cycle_increase: 5,
                cycle_increases_on_page_cross: true
            }),

            //AND
            (0x29, Instruction{
                address_mode: AddressingMode::Immediate,
                operation: CPU::op_and,
                result_handler: CPU::result_into_acc,
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),
            (0x2D, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_and,
                result_handler: CPU::result_into_acc,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0x3D, Instruction{
                address_mode: AddressingMode::XIndexedAbsolute,
                operation: CPU::op_and,
                result_handler: CPU::result_into_acc,
                cycle_increase: 4,
                cycle_increases_on_page_cross: true
            }),
            (0x39, Instruction{
                address_mode: AddressingMode::YIndexedAbsolute,
                operation: CPU::op_and,
                result_handler: CPU::result_into_acc,
                cycle_increase: 4,
                cycle_increases_on_page_cross: true
            }),
            (0x25, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_and,
                result_handler: CPU::result_into_acc,
                cycle_increase: 3,
                cycle_increases_on_page_cross: false
            }),
            (0x35, Instruction{
                address_mode: AddressingMode::XIndexedZeroPage,
                operation: CPU::op_and,
                result_handler: CPU::result_into_acc,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0x21, Instruction{
                address_mode: AddressingMode::XIndexedZeroPageIndirect,
                operation: CPU::op_and,
                result_handler: CPU::result_into_acc,
                cycle_increase: 6,
                cycle_increases_on_page_cross: false
            }),
            (0x31, Instruction{
                address_mode: AddressingMode::ZeroPageIndirectYIndexed,
                operation: CPU::op_and,
                result_handler: CPU::result_into_acc,
                cycle_increase: 5,
                cycle_increases_on_page_cross: true
            }),

            //ASL
            (0x0A, Instruction{
                address_mode: AddressingMode::Accumulator,
                operation: CPU::op_asl,
                result_handler: CPU::result_into_acc,
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),
            (0x0E, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_asl,
                result_handler: CPU::result_into_acc,
                cycle_increase: 6,
                cycle_increases_on_page_cross: false
            }),
            (0x1E, Instruction{
                address_mode: AddressingMode::XIndexedAbsolute,
                operation: CPU::op_asl,
                result_handler: CPU::result_into_acc,
                cycle_increase: 7,
                cycle_increases_on_page_cross: false
            }),
            (0x06, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_asl,
                result_handler: CPU::result_into_acc,
                cycle_increase: 5,
                cycle_increases_on_page_cross: false
            }),
            (0x16, Instruction{
                address_mode: AddressingMode::XIndexedZeroPage,
                operation: CPU::op_asl,
                result_handler: CPU::result_into_acc,
                cycle_increase: 6,
                cycle_increases_on_page_cross: false
            }),

            //BCC
            (0x90, Instruction{
                address_mode: AddressingMode::Relative,
                operation: CPU::op_bcc,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: true
            }),

            //BCS
            (0xB0, Instruction{
                address_mode: AddressingMode::Relative,
                operation: CPU::op_bcs,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: true
            }),

            //BEQ
            (0xF0, Instruction{
                address_mode: AddressingMode::Relative,
                operation: CPU::op_beq,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: true
            }),

            //BIT
            (0x2C, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_bit,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0x24, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_bit,
                result_handler: CPU::no_handler,
                cycle_increase: 3,
                cycle_increases_on_page_cross: false
            }),

            //BMI
            (0x30, Instruction{
                address_mode: AddressingMode::Relative,
                operation: CPU::op_bmi,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: true
            }),

            //BNE
            (0xD0, Instruction{
                address_mode: AddressingMode::Relative,
                operation: CPU::op_bne,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: true
            }),

            //BPL
            (0x10, Instruction{
                address_mode: AddressingMode::Relative,
                operation: CPU::op_bpl,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: true
            }),

            //BRK
            (0x00, Instruction{
                address_mode: AddressingMode::Implied,
                operation: CPU::op_brk,
                result_handler: CPU::no_handler,
                cycle_increase: 7,
                cycle_increases_on_page_cross: false
            }),

            //BVC
            (0x50, Instruction{
                address_mode: AddressingMode::Relative,
                operation: CPU::op_bvc,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: true
            }),

            //BVS
            (0x70, Instruction{
                address_mode: AddressingMode::Relative,
                operation: CPU::op_bvs,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: true
            }),

            //CLC,
            (0x18, Instruction{
                address_mode: AddressingMode::Implied,
                operation: CPU::op_clc,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),

            //CLD
            (0xD8, Instruction{
                address_mode: AddressingMode::Implied,
                operation: CPU::op_cld,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),

            //CLV
            (0xB8, Instruction{
                address_mode: AddressingMode::Implied,
                operation: CPU::op_clv,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),

            //CMP
            (0xC9, Instruction{
                address_mode: AddressingMode::Immediate,
                operation: CPU::op_cmp,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),
            (0xCD, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_cmp,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0xDD, Instruction{
                address_mode: AddressingMode::XIndexedAbsolute,
                operation: CPU::op_cmp,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: true
            }),
            (0xD9, Instruction{
                address_mode: AddressingMode::YIndexedAbsolute,
                operation: CPU::op_cmp,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: true
            }),
            (0xC5, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_cmp,
                result_handler: CPU::no_handler,
                cycle_increase: 3,
                cycle_increases_on_page_cross: false
            }),
            (0xD5, Instruction{
                address_mode: AddressingMode::XIndexedZeroPage,
                operation: CPU::op_cmp,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0xC1, Instruction{
                address_mode: AddressingMode::XIndexedZeroPageIndirect,
                operation: CPU::op_cmp,
                result_handler: CPU::no_handler,
                cycle_increase: 6,
                cycle_increases_on_page_cross: false
            }),
            (0xD1, Instruction{
                address_mode: AddressingMode::ZeroPageIndirectYIndexed,
                operation: CPU::op_cmp,
                result_handler: CPU::no_handler,
                cycle_increase: 5,
                cycle_increases_on_page_cross: true
            }),

            //CPX
            (0xE0, Instruction{
                address_mode: AddressingMode::Immediate,
                operation: CPU::op_cpx,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),
            (0xEC, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_cpx,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0xE4, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_cpx,
                result_handler: CPU::no_handler,
                cycle_increase: 3,
                cycle_increases_on_page_cross: false
            }),

            //CPY
            (0xC0, Instruction{
                address_mode: AddressingMode::Immediate,
                operation: CPU::op_cpy,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),
            (0xCC, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_cpy,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0xC4, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_cpy,
                result_handler: CPU::no_handler,
                cycle_increase: 3,
                cycle_increases_on_page_cross: false
            }),

            //DEC
            (0xCE, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_dec,
                result_handler: CPU::result_into_memory,
                cycle_increase: 6,
                cycle_increases_on_page_cross: false
            }),
            (0xDE, Instruction{
                address_mode: AddressingMode::XIndexedAbsolute,
                operation: CPU::op_dec,
                result_handler: CPU::result_into_memory,
                cycle_increase: 7,
                cycle_increases_on_page_cross: false
            }),
            (0xC6, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_dec,
                result_handler: CPU::result_into_memory,
                cycle_increase: 5,
                cycle_increases_on_page_cross: false
            }),
            (0xD6, Instruction{
                address_mode: AddressingMode::XIndexedZeroPage,
                operation: CPU::op_dec,
                result_handler: CPU::result_into_memory,
                cycle_increase: 6,
                cycle_increases_on_page_cross: false
            }),

            //DEX
            (0xCA, Instruction{
                address_mode: AddressingMode::Implied,
                operation: CPU::op_dex,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),

            //DEY
            (0x88, Instruction{
                address_mode: AddressingMode::Implied,
                operation: CPU::op_dey,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),

            //EOR
            (0x49, Instruction{
                address_mode: AddressingMode::Immediate,
                operation: CPU::op_eor,
                result_handler: CPU::result_into_acc,
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),
            (0x4D, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_eor,
                result_handler: CPU::result_into_acc,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0x5D, Instruction{
                address_mode: AddressingMode::XIndexedAbsolute,
                operation: CPU::op_eor,
                result_handler: CPU::result_into_acc,
                cycle_increase: 4,
                cycle_increases_on_page_cross: true
            }),
            (0x59, Instruction{
                address_mode: AddressingMode::YIndexedAbsolute,
                operation: CPU::op_eor,
                result_handler: CPU::result_into_acc,
                cycle_increase: 4,
                cycle_increases_on_page_cross: true
            }),
            (0x45, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_eor,
                result_handler: CPU::result_into_acc,
                cycle_increase: 3,
                cycle_increases_on_page_cross: false
            }),
            (0x55, Instruction{
                address_mode: AddressingMode::XIndexedZeroPage,
                operation: CPU::op_eor,
                result_handler: CPU::result_into_acc,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0x41, Instruction{
                address_mode: AddressingMode::XIndexedZeroPageIndirect,
                operation: CPU::op_eor,
                result_handler: CPU::result_into_acc,
                cycle_increase: 6,
                cycle_increases_on_page_cross: false
            }),
            (0x51, Instruction{
                address_mode: AddressingMode::ZeroPageIndirectYIndexed,
                operation: CPU::op_eor,
                result_handler: CPU::result_into_acc,
                cycle_increase: 5,
                cycle_increases_on_page_cross: true
            }),

            //INC
            (0xEE, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_inc,
                result_handler: CPU::result_into_memory,
                cycle_increase: 6,
                cycle_increases_on_page_cross: false
            }),
            (0xFE, Instruction{
                address_mode: AddressingMode::XIndexedAbsolute,
                operation: CPU::op_inc,
                result_handler: CPU::result_into_memory,
                cycle_increase: 7,
                cycle_increases_on_page_cross: false
            }),
            (0xE6, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_inc,
                result_handler: CPU::result_into_memory,
                cycle_increase: 5,
                cycle_increases_on_page_cross: false
            }),
            (0xF6, Instruction{
                address_mode: AddressingMode::XIndexedZeroPage,
                operation: CPU::op_inc,
                result_handler: CPU::result_into_memory,
                cycle_increase: 6,
                cycle_increases_on_page_cross: false
            }),

            //INX
            (0xE8, Instruction{
                address_mode: AddressingMode::Implied,
                operation: CPU::op_inx,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),

            //INY
            (0xC8, Instruction{
                address_mode: AddressingMode::Implied,
                operation: CPU::op_iny,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),

            //JMP
            (0x4C, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_jmp,
                result_handler: CPU::no_handler,
                cycle_increase: 3,
                cycle_increases_on_page_cross: false
            }),
            (0x6C, Instruction{
                address_mode: AddressingMode::AbsoluteIndirect,
                operation: CPU::op_jmp,
                result_handler: CPU::no_handler,
                cycle_increase: 5,
                cycle_increases_on_page_cross: false
            }),

            //LDA
            (0xA9, Instruction{
                address_mode: AddressingMode::Immediate,
                operation: CPU::op_lda,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),
            (0xAD, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_lda,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0xBD, Instruction{
                address_mode: AddressingMode::XIndexedAbsolute,
                operation: CPU::op_lda,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: true
            }),
            (0xB9, Instruction{
                address_mode: AddressingMode::YIndexedAbsolute,
                operation: CPU::op_lda,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: true
            }),
            (0xA5, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_lda,
                result_handler: CPU::no_handler,
                cycle_increase: 3,
                cycle_increases_on_page_cross: false
            }),
            (0xB5, Instruction{
                address_mode: AddressingMode::XIndexedZeroPage,
                operation: CPU::op_lda,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0xA1, Instruction{
                address_mode: AddressingMode::XIndexedZeroPageIndirect,
                operation: CPU::op_lda,
                result_handler: CPU::no_handler,
                cycle_increase: 6,
                cycle_increases_on_page_cross: false
            }),
            (0xB1, Instruction{
                address_mode: AddressingMode::ZeroPageIndirectYIndexed,
                operation: CPU::op_lda,
                result_handler: CPU::no_handler,
                cycle_increase: 5,
                cycle_increases_on_page_cross: true
            }),

            //LDX
            (0xA2, Instruction{
                address_mode: AddressingMode::Immediate,
                operation: CPU::op_ldx,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),
            (0xAE, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_ldx,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0xBE, Instruction{
                address_mode: AddressingMode::YIndexedAbsolute,
                operation: CPU::op_ldx,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: true
            }),
            (0xA6, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_ldx,
                result_handler: CPU::no_handler,
                cycle_increase: 3,
                cycle_increases_on_page_cross: false
            }),
            (0xB6, Instruction{
                address_mode: AddressingMode::YIndexedZeroPage,
                operation: CPU::op_ldx,
                result_handler: CPU::no_handler,
                cycle_increase: 3,
                cycle_increases_on_page_cross: false
            }),

            //LDY
            (0xA0, Instruction{
                address_mode: AddressingMode::Immediate,
                operation: CPU::op_ldy,
                result_handler: CPU::no_handler,
                cycle_increase: 2,
                cycle_increases_on_page_cross: false
            }),
            (0xAC, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_ldy,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0xBC, Instruction{
                address_mode: AddressingMode::XIndexedAbsolute,
                operation: CPU::op_ldy,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: true
            }),
            (0xA4, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_ldy,
                result_handler: CPU::no_handler,
                cycle_increase: 3,
                cycle_increases_on_page_cross: false
            }),
            (0xB4, Instruction{
                address_mode: AddressingMode::XIndexedZeroPage,
                operation: CPU::op_ldy,
                result_handler: CPU::no_handler,
                cycle_increase: 3,
                cycle_increases_on_page_cross: false
            }),

            //STA
            (0x8D, Instruction{
                address_mode: AddressingMode::Absolute,
                operation: CPU::op_sta,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0x9D, Instruction{
                address_mode: AddressingMode::XIndexedAbsolute,
                operation: CPU::op_sta,
                result_handler: CPU::no_handler,
                cycle_increase: 5,
                cycle_increases_on_page_cross: false
            }),
            (0x99, Instruction{
                address_mode: AddressingMode::YIndexedAbsolute,
                operation: CPU::op_sta,
                result_handler: CPU::no_handler,
                cycle_increase: 5,
                cycle_increases_on_page_cross: false
            }),
            (0x85, Instruction{
                address_mode: AddressingMode::ZeroPage,
                operation: CPU::op_sta,
                result_handler: CPU::no_handler,
                cycle_increase: 3,
                cycle_increases_on_page_cross: false
            }),
            (0x95, Instruction{
                address_mode: AddressingMode::XIndexedZeroPage,
                operation: CPU::op_sta,
                result_handler: CPU::no_handler,
                cycle_increase: 4,
                cycle_increases_on_page_cross: false
            }),
            (0x81, Instruction{
                address_mode: AddressingMode::XIndexedZeroPageIndirect,
                operation: CPU::op_sta,
                result_handler: CPU::no_handler,
                cycle_increase: 6,
                cycle_increases_on_page_cross: false
            }),
            (0x91, Instruction{
                address_mode: AddressingMode::ZeroPageIndirectYIndexed,
                operation: CPU::op_sta,
                result_handler: CPU::no_handler,
                cycle_increase: 6,
                cycle_increases_on_page_cross: false
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
                if (result_address as usize) >= self.memory.len() {
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