use crate::cpu::cpu::CPU;

mod cpu;
mod helpers;

fn main() {
    let mut cpu = CPU::new();
    let instruction_set = cpu.get_instruction_set();

    let mut bytes = [
        0xA0, 0x05, // LDY #$05 - Load 5 into the Y register
        0xC8,       // INY - Increment Y register (Y becomes 6)
        0x84, 0x20, // STY $20 - Store the Y register result (6) at memory location $20
        0xA4, 0x20, // LDY $20 - Load the value from memory location $20 into the Y register
        0x88,       // DEY - Decrement Y register (Y becomes 5)
        0xC0, 0x03, // CPY #$03 - Compare Y register with 3
        0xD0, 0x01, // BNE $01 - Branch forward 1 byte if comparison is not equal (it should not be equal, Y is 5)
        0x88,       // DEY - Decrement Y register (this will be executed only if the branch is not taken, Y becomes 4)
        0xA2, 0x20, // LDX - Load 32 into the X register
        0xE0, 0x20, // CPX $20 - Compare X register with 32
        0xD0, 0x01, // BNE $01 - Branch forward 1 byte if comparison is not equal (it should be equal, X is 32)
        0xE8,       // INX - Increment X register (X becomes 33)
        0x00        // BRK - Break
    ];

    while (cpu.registers.pc as usize) < bytes.len() {
        let opcode = cpu.get_next_byte(&mut bytes.as_slice());
        let instruction = instruction_set.get(&opcode).unwrap_or_else(|| {
            panic!("Instruction {:#04X?} not found.", opcode)
        });
        cpu.execute_instruction(instruction, bytes.as_slice());

        println!("Instruction {:#04X?}.", opcode)
    }

    println!("PC {}, SP: {}", cpu.registers.pc, cpu.registers.sp);
    println!("ACC: {}, XR: {}, YR: {}", cpu.registers.acc, cpu.registers.xr, cpu.registers.yr);
    println!("Carry: {}, Zero: {}, Interrupt: {}, Decimal: {}, Overflow: {}, Negative: {}", cpu.flags.carry, cpu.flags.zero, cpu.flags.interrupt, cpu.flags.decimal, cpu.flags.overflow, cpu.flags.negative);
    println!("Used Memory:");
    println!("============");
    print_used_memory(cpu);
    println!("============");
}

fn print_used_memory(cpu: CPU) {
    cpu.memory.iter()
        .enumerate()
        .filter(|&(_idx, &value)| value != 0)
        .for_each(|(index, &value)| {
            println!("Index: {}, Value: {}", index, value);
        });
}
