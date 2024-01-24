use crate::cpu::cpu::CPU;

impl CPU {

    pub(crate) fn execute_instruction_sequence(&mut self, mut bytes: &[u8]){
        let instruction_set = self.get_instruction_set();
        while (self.registers.pc as usize) < bytes.len() {
            let opcode = self.get_next_byte(&mut bytes);
            let instruction = instruction_set.get(&opcode).unwrap_or_else(|| {
                panic!("Instruction {:#04X?} not found.", opcode)
            });
            self.execute_instruction(instruction, bytes);
        }
    }

}