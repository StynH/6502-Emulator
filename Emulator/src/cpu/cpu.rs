use crate::helpers::bitwise::is_highest_bit_set;

pub struct CPU{
    pub registers: Registers,
    pub flags: Flags,
    pub stack: Vec<u16>,
    pub memory: Vec<u8>
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
                sp: 255,
            },
            flags: Flags {
                negative: false,
                overflow: false,
                brk: false,
                decimal: false,
                interrupt: false,
                zero: false,
                carry: true,
            },
            stack: Vec::new(),
            memory: vec![0;32 * 32 * 32 * 2]
        }
    }

    pub fn op_adc(&mut self, val: u8) -> u8 {
        let mut sum = self.registers.acc as u16 + val as u16;
        if self.flags.carry {
            sum += 1
        }
        self.flags.carry = sum > 0xFF;
        self.flags.zero = sum == 0;
        self.flags.negative = is_highest_bit_set(sum as u8);

        sum as u8
    }

    pub fn op_and(&mut self, val: u8) -> u8 {
        let and = self.registers.acc & val;
        self.flags.zero = and == 0;
        self.flags.negative = is_highest_bit_set(and);

        and
    }

    pub fn op_asl(&mut self, value: u8) -> u8 {
        let result = value << 1;
        self.flags.carry = value & 0x80 != 0;
        self.flags.zero = result == 0;
        self.flags.negative = result & 0x80 != 0;

        result
    }

    pub fn op_bcc(&mut self, offset: u8){
        if !self.flags.carry {
            let new_pc = self.registers.pc.wrapping_add(offset as u16);
            self.registers.pc = new_pc;
        }
    }

    pub fn op_bcs(&mut self, offset: u8){
        if self.flags.carry {
            let new_pc = self.registers.pc.wrapping_add(offset as u16);
            self.registers.pc = new_pc;
        }
    }

    pub fn op_beq(&mut self, offset: u8){
        if self.flags.zero {
            let new_pc = self.registers.pc.wrapping_add(offset as u16);
            self.registers.pc = new_pc;
        }
    }

    pub fn op_bmi(&mut self, offset: u8){
        if self.flags.negative {
            let new_pc = self.registers.pc.wrapping_add(offset as u16);
            self.registers.pc = new_pc;
        }
    }

    pub fn op_bne(&mut self, offset: u8){
        if !self.flags.zero {
            let new_pc = self.registers.pc.wrapping_add(offset as u16);
            self.registers.pc = new_pc;
        }
    }

    pub fn op_bpl(&mut self, offset: u8){
        if !self.flags.negative {
            let new_pc = self.registers.pc.wrapping_add(offset as u16);
            self.registers.pc = new_pc;
        }
    }

    pub fn op_brk(&mut self){
        self.stack.push(self.registers.pc + 2);
        self.registers.sr = 1;
        self.stack.push(self.registers.sr as u16);
    }

    pub fn op_bvc(&mut self, offset: u8){
        if !self.flags.overflow {
            self.registers.pc = self.registers.pc.wrapping_add(offset as u16);
        }
    }

    pub fn op_bvs(&mut self, offset: u8){
        if self.flags.overflow {
            self.registers.pc = self.registers.pc.wrapping_add(offset as u16);
        }
    }

    pub fn op_clc(&mut self){
        self.flags.carry = false;
    }

    pub fn op_cld(&mut self){
        self.flags.decimal = false;
    }

    pub fn op_cli(&mut self){
        self.flags.interrupt = false;
    }

    pub fn op_clv(&mut self){
        self.flags.overflow = false;
    }

    pub fn op_cmp(&mut self, value: u8){
        let result = self.registers.acc.wrapping_sub(value);
        self.flags.zero = self.registers.acc.eq(&value);
        self.flags.negative = result & (1 << 7) != 0;
        self.flags.carry = self.registers.acc >= value;
    }

    pub fn op_cpx(&mut self, value: u8){
        let result = self.registers.xr.wrapping_sub(value);
        self.flags.zero = self.registers.xr.eq(&value);
        self.flags.negative = result & (1 << 7) != 0;
        self.flags.carry = self.registers.xr >= value;
    }

    pub fn op_cpy(&mut self, value: u8){
        let result = self.registers.yr.wrapping_sub(value);
        self.flags.zero = self.registers.yr.eq(&value);
        self.flags.negative = result & (1 << 7) != 0;
        self.flags.carry = self.registers.yr >= value;
    }

    pub fn op_dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.flags.zero = result == 0;
        self.flags.negative = result & (1 << 7) != 0;

        result
    }

    pub fn op_dex(&mut self){
        let result = self.registers.xr.wrapping_sub(1);
        self.flags.zero = result == 0;
        self.flags.negative = result & (1 << 7) != 0;
        self.registers.xr = result;
    }

    pub fn op_dey(&mut self){
        let result = self.registers.yr.wrapping_sub(1);
        self.flags.zero = result == 0;
        self.flags.negative = result & (1 << 7) != 0;
        self.registers.yr = result;
    }

    pub fn op_eor(&mut self, value: u8){
        self.registers.acc ^= value;
        self.flags.zero = self.registers.acc == 0;
        self.flags.negative = self.registers.acc & (1 << 7) != 0;
    }

    pub fn op_inc(&mut self, value: u8) -> u8{
        let mut result = value.wrapping_add(1);
        self.flags.zero = result == 0;
        self.flags.negative = result & (1 << 7) != 0;

        result
    }

    pub fn op_inx(&mut self){
        self.registers.xr = self.registers.xr.wrapping_add(1);
        self.flags.zero = self.registers.xr == 0;
        self.flags.negative = self.registers.xr & (1 << 7) != 0;
    }

    pub fn op_iny(&mut self){
        self.registers.yr = self.registers.yr.wrapping_add(1);
        self.flags.zero = self.registers.yr == 0;
        self.flags.negative = self.registers.yr & (1 << 7) != 0;
    }

    pub fn op_jmp(&mut self, address: u16){
        self.registers.pc = address;
    }

    pub fn op_jsr(&mut self, address: u16){
        self.stack.push(self.registers.pc - 1);
        self.registers.pc = address;
    }

    pub fn op_lda(&mut self, value: u8){
        self.registers.acc = value;
        self.flags.zero = self.registers.acc == 0;
        self.flags.negative = self.registers.acc & (1 << 7) != 0;
    }

    pub fn op_ldx(&mut self, value: u8){
        self.registers.xr = value;
        self.flags.zero = self.registers.xr == 0;
        self.flags.negative = self.registers.xr & (1 << 7) != 0;
    }

    pub fn op_ldy(&mut self, value: u8){
        self.registers.yr = value;
        self.flags.zero = self.registers.yr == 0;
        self.flags.negative = self.registers.yr & (1 << 7) != 0;
    }

    pub fn op_lsr(&mut self, value: u8) -> u8 {
        self.flags.carry = value & 1 != 0;

        let mut result = value.wrapping_shr(1);
        self.flags.negative = false;
        self.flags.zero = result == 0;

        result
    }

    pub fn op_ora(&mut self, value: u8){
        self.registers.acc |= value;
        self.flags.zero = self.registers.acc == 0;
        self.flags.negative = self.registers.acc & (1 << 7) != 0;
    }

    pub fn op_pha(&mut self){
        self.stack.push(self.registers.acc as u16);
    }

    pub fn op_php(&mut self){
        self.stack.push(self.flags.to_byte() as u16);
    }

    pub fn op_pla(&mut self){
        if let Some(result) = self.stack.pop(){
            self.registers.acc = result as u8;
            self.flags.zero = self.registers.acc == 0;
            self.flags.negative = self.registers.acc & (1 << 7) != 0;
        }
    }

    pub fn op_plp(&mut self){
        if let Some(result) = self.stack.pop(){
            self.flags.load_from_byte(result as u8);
        }
    }

    pub fn op_rol(&mut self, value: u8) -> u8 {
        self.flags.carry = value & (1 << 7) != 0;

        let result = value.rotate_left(1);
        self.flags.negative = result & (1 << 7) != 0;
        self.flags.zero = result == 0;

        result
    }

    pub fn op_ror(&mut self, value: u8) -> u8 {
        self.flags.carry = value & 1 != 0;

        let result = value.rotate_right(1);
        self.flags.negative = result & (1 << 7) != 0;
        self.flags.zero = result == 0;

        result
    }

    pub fn op_rti(&mut self) {
        if let Some(status) = self.stack.pop() {
            self.flags.load_from_byte(status as u8);
        }

        if let Some(low_byte) = self.stack.pop() {
            if let Some(high_byte) = self.stack.pop() {
                self.registers.pc = high_byte << 8 | low_byte;
            }
        }
    }

    pub fn op_rts(&mut self) {
        if let Some(low_byte) = self.stack.pop() {
            if let Some(high_byte) = self.stack.pop() {
                self.registers.pc = high_byte << 8 | low_byte;
                self.registers.pc = self.registers.pc.wrapping_add(1);
            }
        }
    }

    pub fn op_sbc(&mut self, value: u8) {
        let acc = self.registers.acc as u16;
        let value = value as u16;
        let carry = if self.flags.carry { 1 } else { 0 };

        let result = acc.wrapping_sub(value).wrapping_sub(1 - carry);

        self.flags.carry = acc >= value + (1 - carry);
        self.flags.zero = (result as u8) == 0;
        self.flags.negative = (result as u8) & (1 << 7) != 0;
        self.flags.overflow = (((acc ^ result) & (value ^ result)) & 0x80) != 0;

        self.registers.acc = result as u8;
    }

    pub fn op_sec(&mut self) {
        self.flags.carry = true;
    }

    pub fn op_sed(&mut self) {
        self.flags.decimal = true;
    }

    pub fn op_sei(&mut self) {
        self.flags.interrupt = true;
    }

    pub fn op_sta(&mut self, address: usize) {
        if address >= self.memory.len() {
            panic!("Memory access out of bounds");
        }
        self.memory[address] = self.registers.acc;
    }

    pub fn op_stx(&mut self, address: usize) {
        if address >= self.memory.len() {
            panic!("Memory access out of bounds");
        }
        self.memory[address] = self.registers.xr;
    }

    pub fn op_sty(&mut self, address: usize) {
        if address >= self.memory.len() {
            panic!("Memory access out of bounds");
        }
        self.memory[address] = self.registers.yr;
    }

    pub fn op_tax(&mut self) {
        self.registers.xr = self.registers.acc;
        self.flags.negative = self.registers.xr & (1 << 7) != 0;
        self.flags.zero = self.registers.xr == 0;
    }

    pub fn op_tay(&mut self) {
        self.registers.yr = self.registers.acc;
        self.flags.negative = self.registers.yr & (1 << 7) != 0;
        self.flags.zero = self.registers.yr == 0;
    }

    pub fn op_tsx(&mut self) {
        self.registers.xr = self.registers.sp;
        self.flags.negative = self.registers.xr & (1 << 7) != 0;
        self.flags.zero = self.registers.xr == 0;
    }

    pub fn op_txa(&mut self) {
        self.registers.acc = self.registers.xr;
        self.flags.negative = self.registers.acc & (1 << 7) != 0;
        self.flags.zero = self.registers.acc == 0;
    }

    pub fn op_txs(&mut self) {
        self.registers.sp = self.registers.xr;
    }

    pub fn op_tya(&mut self) {
        self.registers.acc = self.registers.yr;
        self.flags.negative = self.registers.acc & (1 << 7) != 0;
        self.flags.zero = self.registers.acc == 0;
    }
}