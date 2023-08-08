use cpu::Cpu;

mod cpu;
mod memory;

fn main() {
    let mut cpu = Cpu::new_test();
    cpu.run();
}