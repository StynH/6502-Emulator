use crate::cpu::cpu::CPU;

mod cpu;
mod helpers;

fn main() {
    let mut cpu = CPU::new();
    let instruction_set = cpu.get_instruction_set();

    let mut bytes = [
        0xA9, 0x01, // LDA #$01 - Load 1 into the accumulator
        0x69, 0x05, // ADC #$05 - Add 5 to the accumulator
        0x85, 0x10, // STA $10 - Store the accumulator result (6) at memory location $10
        0xA2, 0x03, // LDX #$03 - Load 3 into the X register
        0xE0, 0x03, // CPX #$03 - Compare X register with 3
        0xD0, 0x02, // BNE $02 - Branch forward 2 bytes if comparison is not equal (it should be equal, so no branch)
        0xE8,       // INX - Increment X register
        0x00        // BRK - Break
    ];

    while(cpu.registers.pc < bytes.len() as u16){
        let opcode = cpu.get_next_byte(&mut bytes.as_slice());
        let instruction = instruction_set.get(&opcode).unwrap_or_else(|| {
            panic!("Instruction not found.")
        });
        cpu.execute_instruction(instruction, bytes.as_slice());
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
