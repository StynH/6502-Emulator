use crate::cpu::cpu::CPU;

impl CPU {

    pub fn index_zero_page(&self, index: usize) -> u8{
        if index >= 256 {
            panic!("Index out of zero page range!");
        }

        *self.memory.get(index).unwrap_or_else(|| {
            panic!("Memory index out of bounds!")
        })
    }

    pub fn index_zero_page_indexed(&self, index: usize, offset: usize) -> u8{
        let address = index.wrapping_add(offset);
        if address >= 256 {
            panic!("Index out of zero page range!");
        }

        *self.memory.get(address).unwrap_or_else(|| {
            panic!("Memory index out of bounds!")
        })
    }

    pub fn index_absolute(&self, index: usize) -> u8{
        *self.memory.get(index).unwrap_or_else(|| {
            panic!("Memory index out of bounds!")
        })
    }

    pub fn index_absolute_indexed(&self, index: usize, offset: usize) -> u8{
        let address = index.wrapping_add(offset);
        *self.memory.get(address).unwrap_or_else(|| {
            panic!("Memory index out of bounds!")
        })
    }

    pub fn index_indexed_indirect(&self, index: usize, offset: usize) -> u8{
        let zero_page_address = index.wrapping_add(offset);
        let low_byte = self.memory[zero_page_address];
        let high_byte = self.memory[zero_page_address.wrapping_add(1)];
        let final_address = ((high_byte as u16) << 8) | (low_byte as u16);

        self.memory[final_address as usize]
    }

    pub fn index_indirect_indexed(&self, index: usize, offset: usize) -> u8{
        let low_byte = self.memory[index];
        let high_byte = self.memory[index.wrapping_add(1)];
        let final_address = (((high_byte as u16) << 8) | (low_byte as u16)).wrapping_add(offset as u16);

        self.memory[final_address as usize]
    }

    pub fn index_relative(&mut self, offset: i8) {
        self.registers.pc = self.registers.pc.wrapping_add(offset as i16 as u16);
    }
}