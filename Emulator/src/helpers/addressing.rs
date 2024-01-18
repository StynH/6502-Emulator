pub fn page_crossed(base_address: u16, compare_address: u16) -> bool {
    (base_address & 0xFF00) != (compare_address & 0xFF00)
}