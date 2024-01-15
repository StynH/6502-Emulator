use crate::cpu::cpu::CPU;

mod cpu;
mod helpers;

fn main() {
    let mut cpu = CPU::new();
    cpu.op_lda(1u8);
    cpu.op_sta(0x02);
    cpu.memory[0x02] = cpu.op_inc(0x02);
    cpu.op_lda(cpu.memory[0x02]);

    println!("{}", cpu.registers.acc)
}
