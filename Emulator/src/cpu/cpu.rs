use crate::cpu::instructions::InstructionParameter;
use crate::helpers::addressing::page_crossed;
use crate::helpers::bitwise::{get_bit_at_position, get_msb, is_highest_bit_set, merge_bytes_into_word, split_word_into_bytes};

pub struct CPU{
    pub registers: Registers,
    pub flags: Flags,
    pub memory: Vec<u8>,
    pub cycles: u32
}

pub struct Registers{
    pub acc: u8,
    pub pc: u16,
    pub xr: u8,
    pub yr: u8,
    pub sr: u8,
    pub sp: u8
}

pub struct Flags{
    pub negative: bool,
    pub overflow: bool,
    pub brk: bool,
    pub decimal: bool,
    pub interrupt: bool,
    pub zero: bool,
    pub carry: bool
}

impl Flags{

    pub fn to_byte(&self) -> u8{
        let mut byte = 0u8;

        if self.negative  { byte |= 1 << 0 }
        if self.overflow  { byte |= 1 << 1 }
        if self.brk       { byte |= 1 << 3 }
        if self.decimal   { byte |= 1 << 4 }
        if self.interrupt { byte |= 1 << 5 }
        if self.zero      { byte |= 1 << 6 }
        if self.carry     { byte |= 1 << 7 }

        byte
    }

    pub fn load_from_byte(&mut self, byte: u8){
        self.negative =  byte & 1 != 0;
        self.overflow =  byte >> 1 & 1 != 0;
        self.brk =       byte >> 3 & 1 != 0;
        self.decimal =   byte >> 4 & 1 != 0;
        self.interrupt = byte >> 5 & 1 != 0;
        self.zero =      byte >> 6 & 1 != 0;
        self.carry =     byte >> 7 & 1 != 0;
    }
}

impl CPU{

    pub fn new() -> Self{
        Self{
            registers: Registers {
                acc: 0,
                pc: 0,
                xr: 0,
                yr: 0,
                sr: 0,
                sp: 0xFF,
            },
            flags: Flags {
                negative: false,
                overflow: false,
                brk: false,
                decimal: false,
                interrupt: false,
                zero: false,
                carry: false,
            },
            memory: vec![0;32 * 32 * 32 * 2],
            cycles: 0
        }
    }

    pub fn op_adc(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                let mut sum = self.registers.acc as u16 + value as u16;
                if self.flags.carry {
                    sum += 1
                }
                self.flags.carry = sum > 0xFF;
                self.flags.zero = sum == 0;
                self.flags.negative = is_highest_bit_set(sum as u8);

                Some(sum as u8)
            }
            _ => { None }
        }
    }

    pub fn op_and(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                let and = self.registers.acc & value;
                self.flags.zero = and == 0;
                self.flags.negative = is_highest_bit_set(and);

                Some(and)
            }
            _ => { None }
        }
    }

    pub fn op_asl(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                let result = value << 1;
                self.flags.carry = value & 0x80 != 0;
                self.flags.zero = result == 0;
                self.flags.negative = result & 0x80 != 0;

                Some(result)
            }
            _ => { None }
        }
    }

    pub fn op_bcc(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(offset) => {
                if !self.flags.carry {
                    let new_pc = self.registers.pc.wrapping_add(offset as u16);
                    self.registers.pc = new_pc;
                }

                None
            }
            _ => { None }
        }
    }

    pub fn op_bcs(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(offset) => {
                if self.flags.carry {
                    let new_pc = self.registers.pc.wrapping_add(offset as u16);
                    self.registers.pc = new_pc;
                }

                None
            }
            _ => { None }
        }
    }

    pub fn op_beq(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(offset) => {
                if self.flags.zero {
                    let new_pc = self.registers.pc.wrapping_add(offset as u16);
                    self.registers.pc = new_pc;
                }

                None
            }
            _ => { None }
        }
    }

    pub fn op_bit(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                let result = self.registers.acc & value;

                self.flags.negative = get_msb(value) != 0;
                self.flags.overflow = get_bit_at_position(value, 6) != 0;
                self.flags.zero = result == 0;

                None
            }
            _ => { None }
        }
    }

    pub fn op_bmi(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(offset) => {
                if self.flags.negative {
                    let new_pc = self.registers.pc.wrapping_add(offset as u16);
                    self.registers.pc = new_pc;
                    self.cycles += 1;
                }

                None

            }
            _ => { None }
        }
    }

    pub fn op_bne(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Word(offset) => {
                if !self.flags.zero {
                    let new_pc = self.registers.pc.wrapping_add(offset as i16 as u16);
                    self.registers.pc = new_pc;
                    self.cycles += 1;
                }

                None
            }
            _ => { None }
        }
    }

    pub fn op_bpl(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(offset) => {
                if !self.flags.negative {
                    let new_pc = self.registers.pc.wrapping_add(offset as u16);
                    self.registers.pc = new_pc;
                    self.cycles += 1;
                }

                None
            }
            _ => { None }
        }
    }

    pub fn op_brk(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.push_word_to_stack(self.registers.pc + 2);
                self.registers.sr = 1;
                self.push_byte_to_stack(self.registers.sr);

                None
            }
            _ => { None }
        }
    }

    pub fn op_bvc(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(offset) => {
                if !self.flags.overflow {
                    self.registers.pc = self.registers.pc.wrapping_add(offset as u16);
                    self.cycles += 1;
                }

                None
            }
            _ => { None }
        }
    }

    pub fn op_bvs(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(offset) => {
                if self.flags.overflow {
                    self.registers.pc = self.registers.pc.wrapping_add(offset as u16);
                    self.cycles += 1;
                }

                None
            }
            _ => { None }
        }
    }

    pub fn op_clc(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.flags.carry = false;

                None
            }
            _ => { None }
        }
    }

    pub fn op_cld(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.flags.decimal = false;

                None
            }
            _ => { None }
        }
    }

    pub fn op_cli(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.flags.interrupt = false;

                None
            }
            _ => { None }
        }
    }

    pub fn op_clv(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.flags.overflow = false;

                None
            }
            _ => { None }
        }
    }

    pub fn op_cmp(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                let result = self.registers.acc.wrapping_sub(value);
                self.flags.zero = self.registers.acc.eq(&value);
                self.flags.negative = get_msb(result) != 0;
                self.flags.carry = self.registers.acc >= value;

                None
            }
            _ => { None }
        }
    }

    pub fn op_cpx(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                let result = self.registers.xr.wrapping_sub(value);
                self.flags.zero = self.registers.xr.eq(&value);
                self.flags.negative = get_msb(result) != 0;
                self.flags.carry = self.registers.xr >= value;

                None
            }
            _ => { None }
        }
    }

    pub fn op_cpy(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                let result = self.registers.yr.wrapping_sub(value);
                self.flags.zero = self.registers.yr.eq(&value);
                self.flags.negative = get_msb(result) != 0;
                self.flags.carry = self.registers.yr >= value;

                None
            }
            _ => { None }
        }
    }

    pub fn op_dec(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                let result = value.wrapping_sub(1);
                self.flags.zero = result == 0;
                self.flags.negative = get_msb(value) != 0;

                Some(result)
            }
            _ => { None }
        }
    }

    pub fn op_dex(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                let result = self.registers.xr.wrapping_sub(1);
                self.flags.zero = result == 0;
                self.flags.negative = get_msb(result) != 0;
                self.registers.xr = result;

                None
            }
            _ => { None }
        }
    }

    pub fn op_dey(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                let result = self.registers.yr.wrapping_sub(1);
                self.flags.zero = result == 0;
                self.flags.negative = get_msb(result) != 0;
                self.registers.yr = result;

                None
            }
            _ => { None }
        }
    }

    pub fn op_eor(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                self.registers.acc ^= value;
                self.flags.zero = self.registers.acc == 0;
                self.flags.negative = get_msb(self.registers.acc) != 0;

                None
            }
            _ => { None }
        }
    }

    pub fn op_inc(&mut self, parameter: InstructionParameter) -> Option<u8>{
        match parameter {
            InstructionParameter::Byte(value) => {
                let mut result = value.wrapping_add(1);
                self.flags.zero = result == 0;
                self.flags.negative = get_msb(value) != 0;

                Some(result)
            }
            _ => { None }
        }
    }

    pub fn op_inx(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.registers.xr = self.registers.xr.wrapping_add(1);
                self.flags.zero = self.registers.xr == 0;
                self.flags.negative = get_msb(self.registers.xr) != 0;

                None
            }
            _ => { None }
        }
    }

    pub fn op_iny(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.registers.yr = self.registers.yr.wrapping_add(1);
                self.flags.zero = self.registers.yr == 0;
                self.flags.negative = get_msb(self.registers.yr) != 0;

                None
            }
            _ => { None }
        }
    }

    pub fn op_jmp(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Word(address) => {
                self.registers.pc = address;

                None
            }
            _ => { None }
        }
    }

    pub fn op_jsr(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Word(address) => {
                self.push_word_to_stack(self.registers.pc - 1);
                self.registers.pc = address;

                None
            }
            _ => { None }
        }
    }

    pub fn op_lda(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                self.registers.acc = value;
                self.flags.zero = self.registers.acc == 0;
                self.flags.negative = get_msb(self.registers.acc) != 0;

                None
            }
            _ => { None }
        }
    }

    pub fn op_ldx(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                self.registers.xr = value;
                self.flags.zero = self.registers.xr == 0;
                self.flags.negative = get_msb(self.registers.xr) != 0;

                None
            }
            _ => { None }
        }

    }

    pub fn op_ldy(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                self.registers.yr = value;
                self.flags.zero = self.registers.yr == 0;
                self.flags.negative = get_msb(self.registers.yr) != 0;

                None
            }
            _ => { None }
        }
    }

    pub fn op_lsr(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                self.flags.carry = value & 1 != 0;

                let mut result = value.wrapping_shr(1);
                self.flags.negative = false;
                self.flags.zero = result == 0;

                Some(result)
            }
            _ => { None }
        }
    }

    pub fn op_nop(&mut self, parameter: InstructionParameter) -> Option<u8>{
        None
    }

    pub fn op_ora(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                self.registers.acc |= value;
                self.flags.zero = self.registers.acc == 0;
                self.flags.negative = get_msb(self.registers.acc) != 0;

                None
            }
            _ => { None }
        }
    }

    pub fn op_pha(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.push_byte_to_stack(self.registers.acc);

                None
            }
            _ => { None }
        }
    }

    pub fn op_php(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.push_byte_to_stack(self.flags.to_byte());

                None
            }
            _ => { None }
        }
    }

    pub fn op_pla(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                if let Some(result) = self.pop_byte_from_stack(){
                    self.registers.acc = result;
                    self.flags.zero = self.registers.acc == 0;
                    self.flags.negative = get_msb(self.registers.acc) != 0;
                }

                None
            }
            _ => { None }
        }
    }

    pub fn op_plp(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                if let Some(result) = self.pop_byte_from_stack(){
                    self.flags.load_from_byte(result);
                }

                None
            }
            _ => { None }
        }
    }

    pub fn op_rol(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                self.flags.carry = get_msb(value) != 0;

                let result = value.rotate_left(1);
                self.flags.negative = get_msb(result) != 0;
                self.flags.zero = result == 0;

                Some(result)
            }
            _ => { None }
        }
    }

    pub fn op_ror(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                self.flags.carry = value & 1 != 0;

                let result = value.rotate_right(1);
                self.flags.negative = get_msb(value) != 0;
                self.flags.zero = result == 0;

                Some(result)
            }
            _ => { None }
        }
    }

    pub fn op_rti(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                if let Some(status) = self.pop_byte_from_stack() {
                    self.flags.load_from_byte(status);
                }

                if let Some(value) = self.pop_word_from_stack() {
                    self.registers.pc = value;
                }

                None
            }
            _ => { None }
        }
    }

    pub fn op_rts(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                if let Some(value) = self.pop_word_from_stack() {
                    self.registers.pc = value;
                    self.registers.pc = self.registers.pc.wrapping_add(1);
                }

                None
            }
            _ => { None }
        }
    }

    pub fn op_sbc(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Byte(value) => {
                let acc = self.registers.acc as u16;
                let value = value as u16;
                let carry = if self.flags.carry { 1 } else { 0 };

                let result = acc.wrapping_sub(value).wrapping_sub(1 - carry);

                self.flags.carry = acc >= value + (1 - carry);
                self.flags.zero = (result as u8) == 0;
                self.flags.negative = get_msb(result as u8) != 0;
                self.flags.overflow = (((acc ^ result) & (value ^ result)) & 0x80) != 0;

                self.registers.acc = result as u8;

                None
            }
            _ => { None }
        }
    }

    pub fn op_sec(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.flags.carry = true;

                None
            }
            _ => { None }
        }
    }

    pub fn op_sed(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.flags.decimal = true;

                None
            }
            _ => { None }
        }
    }

    pub fn op_sei(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.flags.interrupt = true;

                None
            }
            _ => { None }
        }
    }

    pub fn op_sta(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Word(address) => {
                if (address as usize) >= self.memory.len() {
                    panic!("Memory access out of bounds");
                }
                self.memory[address as usize] = self.registers.acc;

                None
            }
            _ => { None }
        }
    }

    pub fn op_stx(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Word(address) => {
                if (address as usize) >= self.memory.len() {
                    panic!("Memory access out of bounds");
                }
                self.memory[address as usize] = self.registers.xr;

                None
            }
            _ => { None }
        }
    }

    pub fn op_sty(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::Word(address) => {
                if (address as usize) >= self.memory.len() {
                    panic!("Memory access out of bounds");
                }
                self.memory[address as usize] = self.registers.yr;

                None
            }
            _ => { None }
        }
    }

    pub fn op_tax(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.registers.xr = self.registers.acc;
                self.flags.negative = get_msb(self.registers.xr) != 0;
                self.flags.zero = self.registers.xr == 0;

                None
            }
            _ => { None }
        }
    }

    pub fn op_tay(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.registers.yr = self.registers.acc;
                self.flags.negative = get_msb(self.registers.yr) != 0;
                self.flags.zero = self.registers.yr == 0;

                None
            }
            _ => { None }
        }
    }

    pub fn op_tsx(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.registers.xr = self.registers.sp;
                self.flags.negative = get_msb(self.registers.xr) != 0;
                self.flags.zero = self.registers.xr == 0;

                None
            }
            _ => { None }
        }
    }

    pub fn op_txa(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.registers.acc = self.registers.xr;
                self.flags.negative = get_msb(self.registers.acc) != 0;
                self.flags.zero = self.registers.acc == 0;

                None
            }
            _ => { None }
        }
    }

    pub fn op_txs(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.registers.sp = self.registers.xr;

                None
            }
            _ => { None }
        }
    }

    pub fn op_tya(&mut self, parameter: InstructionParameter) -> Option<u8> {
        match parameter {
            InstructionParameter::None => {
                self.registers.acc = self.registers.yr;
                self.flags.negative = get_msb(self.registers.acc) != 0;
                self.flags.zero = self.registers.acc == 0;

                None
            }
            _ => { None }
        }
    }
}