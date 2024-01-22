use crate::cpu::cpu::CPU;

mod cpu;
mod helpers;

fn main() {
    let mut cpu = CPU::new();
    let instruction_set = cpu.get_instruction_set();

    let bytes = [
        0x20, 0x09, 0x00, 0x20, 0x0c, 0x00, 0x20, 0x12, 0x00, 0xa2, 0x00, 0x60, 0xe8, 0xe0, 0x05, 0xd0, 0xfb, 0x60, 0x00
    ];

    cpu.set_interrupt_vector(0xFF, 0xFF);
    while (cpu.registers.pc as usize) < bytes.len() {
        let opcode = cpu.get_next_byte(&mut bytes.as_slice());
        let instruction = instruction_set.get(&opcode).unwrap_or_else(|| {
            panic!("Instruction {:#04X?} not found.", opcode)
        });
        cpu.execute_instruction(instruction, bytes.as_slice());
    }

    println!("PC {:#04X?}, SP: {:#04X?}", cpu.registers.pc, cpu.registers.sp);
    println!("ACC: {:#04X?}, XR: {:#04X?}, YR: {:#04X?}", cpu.registers.acc, cpu.registers.xr, cpu.registers.yr);
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
