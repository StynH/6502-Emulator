pub fn is_highest_bit_set(value: u8) -> bool {
    value & 0b10000000 != 0
}