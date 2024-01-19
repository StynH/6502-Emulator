pub fn is_highest_bit_set(value: u8) -> bool {
    value & 0b10000000 != 0
}

pub fn split_word_into_bytes(value: u16) -> (u8, u8){
    ((value >> 8) as u8, (value & 0xFF) as u8)
}

pub fn merge_bytes_into_word(high_byte: u8, low_byte: u8) -> u16{
    (high_byte as u16) << 8 | low_byte as u16
}

pub fn get_msb(value: u8) -> u8{
    get_bit_at_position(value, 7)
}

pub fn get_bit_at_position(value: u8, position: u8) -> u8{
    value & (1 << position)
}