use crate::cpu::cpu::CPU;

mod cpu;
mod helpers;

fn main() {
    let mut cpu = CPU::new();
    cpu.memory[0x00] = 0x20;

    let instruction_set = cpu.get_instruction_set();

    let mut bytes = [
        0x18, //CDC
        0xA9, 0x20, //LDA 32
        0xC9, 0x20 //CMP 32
    ];

    while(cpu.registers.pc < bytes.len() as u16){
        let opcode = cpu.get_next_byte(&mut bytes.as_slice());
        let instruction = instruction_set.get(&opcode).unwrap_or_else(|| {
            panic!("Instruction not found.")
        });
        cpu.execute_instruction(instruction, bytes.as_slice());
    }

    println!("ACC: {}", cpu.registers.acc);
    println!("Carry: {}, Zero: {}, Interrupt: {}, Decimal: {}, Overflow: {}, Negative: {}", cpu.flags.carry, cpu.flags.zero, cpu.flags.zero, cpu.flags.decimal, cpu.flags.overflow, cpu.flags.negative);
}
