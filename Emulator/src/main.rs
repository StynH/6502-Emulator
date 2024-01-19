use crate::cpu::cpu::CPU;

mod cpu;
mod helpers;

fn main() {
    let mut cpu = CPU::new();
    let instruction_set = cpu.get_instruction_set();

    let bytes = [
        0xA9, 0x00, // LDA #00
        0x85, 0x00, // STA 0200
        0xE6, 0x00, // INC 0200
        0xA5, 0x00, // LDA 0200
        0xC9, 0x20, // CMP #10
        0xD0, 0xF8, // BNE -8 (to the INC instruction)
        0x00        // BRK
    ];

    while (cpu.registers.pc as usize) < bytes.len() {
        let opcode = cpu.get_next_byte(&mut bytes.as_slice());
        let instruction = instruction_set.get(&opcode).unwrap_or_else(|| {
            panic!("Instruction {:#04X?} not found.", opcode)
        });
        cpu.execute_instruction(instruction, bytes.as_slice());

        //println!("Instruction {:#04X?}.", opcode)
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
