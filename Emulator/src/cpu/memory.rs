use crate::cpu::cpu::CPU;
use crate::helpers::addressing::page_crossed;

impl CPU {

    pub fn index_zero_page(&self, index: u8) -> u8{
        *self.memory.get(index as usize).unwrap_or_else(|| {
            panic!("Memory index out of bounds!")
        })
    }

    pub fn index_zero_page_indexed(&self, index: u8, offset: u8) -> (u8, u16){
        let address = index.wrapping_add(offset);
        (*self.memory.get(address as usize).unwrap_or_else(|| {
            panic!("Memory index out of bounds!")
        }), address as u16)
    }

    pub fn index_absolute(&self, index: u16) -> u8{
        *self.memory.get(index as usize).unwrap_or_else(|| {
            panic!("Memory index out of bounds!")
        })
    }

    pub fn index_absolute_indexed(&self, index: u16, offset: u8) -> (u8, bool, u16){
        let address = index.wrapping_add(offset as u16);
        let fetched = *self.memory.get(address as usize).unwrap_or_else(|| {
            panic!("Memory index out of bounds!")
        });

        (fetched, page_crossed(index, address), address)
    }

    pub fn index_zero_paged_indexed_indirect(&self, index: u8, offset: u8) -> (u8, u16) {
        let zero_page_address = index.wrapping_add(offset);
        let low_byte = self.memory[zero_page_address as usize];
        let high_byte = self.memory[zero_page_address.wrapping_add(1) as usize];
        let final_address = ((high_byte as u16) << 8) | (low_byte as u16);

        (self.memory[final_address as usize], final_address)
    }

    pub fn index_zero_paged_indirect_indexed(&self, index: u8, offset: u8) -> (u8, bool, u16) {
        let low_byte = self.memory[index as usize] as u16;
        let high_byte = self.memory[index.wrapping_add(1) as usize] as u16;
        let effective_address = (high_byte << 8) | low_byte;
        let final_address = effective_address.wrapping_add(offset as u16);

        (self.memory[final_address as usize], page_crossed(effective_address, final_address), final_address)
    }
}