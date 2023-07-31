use cpu::Cpu;

mod cpu;
mod memory;

fn main() {
    let mut cpu = Cpu::new();
    cpu.run();
}